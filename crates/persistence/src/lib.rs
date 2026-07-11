#![deny(unsafe_code)]

use std::{
    borrow::Cow,
    error::Error,
    fmt,
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
    time::Duration,
};

use futures_core::future::BoxFuture;
use sqlx::{
    SqlitePool,
    migrate::{MigrateError, Migration, MigrationSource, MigrationType, Migrator},
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};

const DATABASE_FILE_NAME: &str = "dev-recall.sqlite3";
const MAX_CONNECTIONS: u32 = 4;
const MIN_CONNECTIONS: u32 = 0;
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(5);
const BUSY_TIMEOUT: Duration = Duration::from_secs(5);
const IDLE_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_LIFETIME: Duration = Duration::from_secs(30 * 60);
const STATEMENT_CACHE_CAPACITY: usize = 64;
const COMMAND_BUFFER_SIZE: usize = 32;
const ROW_BUFFER_SIZE: usize = 128;
const MAX_DATABASE_PATH_UNITS: usize = 4_096;

const INITIAL_MIGRATION_SQL: &str = include_str!("../../../migrations/0001_initial.sql");

#[derive(Debug, Clone, Copy)]
struct EmbeddedMigrations;

impl MigrationSource<'static> for EmbeddedMigrations {
    fn resolve(self) -> BoxFuture<'static, Result<Vec<Migration>, Box<dyn Error + Send + Sync>>> {
        Box::pin(async {
            Ok(vec![Migration::new(
                1,
                Cow::Borrowed("initial"),
                MigrationType::Simple,
                Cow::Borrowed(INITIAL_MIGRATION_SQL),
                false,
            )])
        })
    }
}

/// Validated configuration for the application-owned database file.
///
/// The directory must come from the desktop platform's application-data API,
/// not from IPC or another user-controlled input.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    database_path: PathBuf,
}

impl DatabaseConfig {
    pub fn for_application_data_directory(
        application_data_directory: &Path,
    ) -> Result<Self, DatabaseConfigError> {
        if !application_data_directory.is_absolute() {
            return Err(DatabaseConfigError::InvalidDirectory);
        }

        let canonical_directory = fs::canonicalize(application_data_directory)
            .map_err(|_| DatabaseConfigError::UnavailableDirectory)?;
        let metadata = fs::metadata(&canonical_directory)
            .map_err(|_| DatabaseConfigError::UnavailableDirectory)?;

        if !metadata.is_dir() {
            return Err(DatabaseConfigError::InvalidDirectory);
        }

        let database_path = canonical_directory.join(DATABASE_FILE_NAME);
        if platform_path_length(&database_path) > MAX_DATABASE_PATH_UNITS {
            return Err(DatabaseConfigError::InvalidDirectory);
        }

        if let Ok(database_metadata) = fs::symlink_metadata(&database_path) {
            if !database_metadata.is_file() || database_metadata.file_type().is_symlink() {
                return Err(DatabaseConfigError::UnsafeDatabaseFile);
            }
        }

        Ok(Self { database_path })
    }

    pub fn max_connections(&self) -> u32 {
        MAX_CONNECTIONS
    }

    fn connect_options(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new()
            .filename(&self.database_path)
            .create_if_missing(false)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Delete)
            .synchronous(SqliteSynchronous::Full)
            .busy_timeout(BUSY_TIMEOUT)
            .statement_cache_capacity(STATEMENT_CACHE_CAPACITY)
            .command_buffer_size(COMMAND_BUFFER_SIZE)
            .row_buffer_size(ROW_BUFFER_SIZE)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseConfigError {
    InvalidDirectory,
    UnavailableDirectory,
    UnsafeDatabaseFile,
}

impl fmt::Display for DatabaseConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidDirectory => "the application data directory is invalid",
            Self::UnavailableDirectory => "the application data directory is unavailable",
            Self::UnsafeDatabaseFile => "the application database file is unsafe",
        })
    }
}

impl Error for DatabaseConfigError {}

#[derive(Debug)]
pub enum DatabaseError {
    FilePreparation(io::Error),
    Connection(sqlx::Error),
    Migration(MigrateError),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::FilePreparation(_) => "the local database file could not be prepared",
            Self::Connection(_) => "the local database could not be opened",
            Self::Migration(_) => "the local database schema could not be prepared",
        })
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FilePreparation(source) => Some(source),
            Self::Connection(source) => Some(source),
            Self::Migration(source) => Some(source),
        }
    }
}

/// Owns the bounded SQLite pool and its worker threads.
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn open(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        prepare_database_file(&config.database_path).map_err(DatabaseError::FilePreparation)?;

        let pool = pool_options()
            .connect_with(config.connect_options())
            .await
            .map_err(DatabaseError::Connection)?;

        let migrator = match Migrator::new(EmbeddedMigrations).await {
            Ok(migrator) => migrator,
            Err(source) => {
                pool.close().await;
                return Err(DatabaseError::Migration(source));
            }
        };

        if let Err(source) = migrator.run(&pool).await {
            pool.close().await;
            return Err(DatabaseError::Migration(source));
        }

        Ok(Self { pool })
    }

    pub async fn close(self) {
        self.pool.close().await;
    }

    #[cfg(test)]
    fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

fn prepare_database_file(database_path: &Path) -> io::Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(0o600);
    }

    match options.open(database_path) {
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            validate_existing_database_file(database_path)
        }
        Err(error) => Err(error),
    }
}

fn validate_existing_database_file(database_path: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(database_path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "application database path is not a regular file",
        ));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "application database permissions are too broad",
            ));
        }
    }

    Ok(())
}

#[cfg(windows)]
fn platform_path_length(path: &Path) -> usize {
    use std::os::windows::ffi::OsStrExt;

    path.as_os_str().encode_wide().count()
}

#[cfg(unix)]
fn platform_path_length(path: &Path) -> usize {
    use std::os::unix::ffi::OsStrExt;

    path.as_os_str().as_bytes().len()
}

fn pool_options() -> SqlitePoolOptions {
    SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .min_connections(MIN_CONNECTIONS)
        .acquire_timeout(ACQUIRE_TIMEOUT)
        .idle_timeout(Some(IDLE_TIMEOUT))
        .max_lifetime(Some(MAX_LIFETIME))
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    use sqlx::Row;

    use super::{Database, DatabaseConfig, DatabaseConfigError, MAX_CONNECTIONS, pool_options};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn create() -> Self {
            let sequence = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "dev-recall-persistence-test-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir(&path).expect("test directory should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn rejects_relative_application_data_directory() {
        let result = DatabaseConfig::for_application_data_directory(Path::new("relative"));

        assert_eq!(result.unwrap_err(), DatabaseConfigError::InvalidDirectory);
    }

    #[test]
    fn rejects_non_file_database_target() {
        let directory = TestDirectory::create();
        fs::create_dir(directory.path().join(super::DATABASE_FILE_NAME))
            .expect("unsafe database directory should be created");

        let result = DatabaseConfig::for_application_data_directory(directory.path());

        assert_eq!(result.unwrap_err(), DatabaseConfigError::UnsafeDatabaseFile);
    }

    #[test]
    fn pool_has_explicit_connection_and_timeout_bounds() {
        let options = pool_options();

        assert_eq!(options.get_max_connections(), MAX_CONNECTIONS);
        assert_eq!(options.get_min_connections(), 0);
        assert_eq!(options.get_acquire_timeout().as_secs(), 5);
        assert_eq!(
            options.get_idle_timeout().map(|value| value.as_secs()),
            Some(60)
        );
        assert_eq!(
            options.get_max_lifetime().map(|value| value.as_secs()),
            Some(30 * 60)
        );
    }

    #[tokio::test]
    async fn opens_database_with_foreign_keys_and_initial_migration() {
        let directory = TestDirectory::create();
        let config = DatabaseConfig::for_application_data_directory(directory.path())
            .expect("test directory should be accepted");
        assert_eq!(config.max_connections(), MAX_CONNECTIONS);
        #[cfg(unix)]
        let database_path = config.database_path.clone();

        let database = Database::open(config.clone())
            .await
            .expect("database should open and migrate");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let mode = fs::metadata(database_path)
                .expect("database metadata should be readable")
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, 0o600);
        }

        let foreign_keys: i64 = sqlx::query_scalar("PRAGMA foreign_keys")
            .fetch_one(database.pool())
            .await
            .expect("foreign key state should be readable");
        assert_eq!(foreign_keys, 1);

        let journal_mode: String = sqlx::query_scalar("PRAGMA journal_mode")
            .fetch_one(database.pool())
            .await
            .expect("journal mode should be readable");
        assert_ne!(journal_mode.to_ascii_lowercase(), "wal");

        let migration_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(database.pool())
            .await
            .expect("migration history should be readable");
        assert_eq!(migration_count, 1);

        let virtual_table_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_schema WHERE sql LIKE 'CREATE VIRTUAL TABLE%'",
        )
        .fetch_one(database.pool())
        .await
        .expect("virtual table state should be readable");
        assert_eq!(virtual_table_count, 0);

        let owner_table =
            sqlx::query("SELECT name FROM sqlite_schema WHERE type = 'table' AND name = 'owners'")
                .fetch_one(database.pool())
                .await
                .expect("owners table should exist");
        assert_eq!(owner_table.get::<String, _>("name"), "owners");

        let invalid_owner =
            sqlx::query("INSERT INTO owners (id, owner_type, created_at_unix_ms) VALUES (?, ?, ?)")
                .bind("too-short")
                .bind("local")
                .bind(0_i64)
                .execute(database.pool())
                .await;
        assert!(invalid_owner.is_err());

        sqlx::query("INSERT INTO owners (id, owner_type, created_at_unix_ms) VALUES (?, ?, ?)")
            .bind("00000000-0000-0000-0000-000000000001")
            .bind("local")
            .bind(0_i64)
            .execute(database.pool())
            .await
            .expect("valid local owner should be accepted");

        let duplicate_local_owner =
            sqlx::query("INSERT INTO owners (id, owner_type, created_at_unix_ms) VALUES (?, ?, ?)")
                .bind("00000000-0000-0000-0000-000000000002")
                .bind("local")
                .bind(1_i64)
                .execute(database.pool())
                .await;
        assert!(duplicate_local_owner.is_err());

        database.close().await;

        let reopened_database = Database::open(config)
            .await
            .expect("existing database should reopen cleanly");
        let migration_count_after_reopen: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
                .fetch_one(reopened_database.pool())
                .await
                .expect("migration history should remain readable");
        assert_eq!(migration_count_after_reopen, 1);
        reopened_database.close().await;
    }
}
