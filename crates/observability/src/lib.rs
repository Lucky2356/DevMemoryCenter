#![deny(unsafe_code)]

//! Bounded local JSON-lines logging with no arbitrary string value surface.
//!
//! Event names, components, levels, and metric names are closed enums. Context
//! values and operation identifiers are numeric, so callers cannot pass raw
//! commands, notes, paths, headers, credentials, or error text into log files.

use std::{
    error::Error,
    fmt,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const LOG_DIRECTORY_NAME: &str = "logs";
const ACTIVE_LOG_FILE_NAME: &str = "dev-recall.log";
const DEFAULT_MAX_FILE_BYTES: u64 = 1024 * 1024;
const DEFAULT_MAX_ARCHIVES: u8 = 5;
const DEFAULT_RETENTION_DAYS: u16 = 7;
const MIN_FILE_BYTES: u64 = 4 * 1024;
const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_ARCHIVES: u8 = 10;
const MAX_RETENTION_DAYS: u16 = 365;
const MAX_METRICS: usize = 8;
const MAX_LOG_PATH_UNITS: usize = 4_096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogComponent {
    Application,
    Ipc,
    Persistence,
    Privacy,
    Import,
}

impl LogComponent {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Application => "application",
            Self::Ipc => "ipc",
            Self::Persistence => "persistence",
            Self::Privacy => "privacy",
            Self::Import => "import",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogEventCode {
    ApplicationStarted,
    ApplicationStopped,
    OperationSucceeded,
    OperationFailed,
    RetentionCompleted,
}

impl LogEventCode {
    const fn as_str(self) -> &'static str {
        match self {
            Self::ApplicationStarted => "application_started",
            Self::ApplicationStopped => "application_stopped",
            Self::OperationSucceeded => "operation_succeeded",
            Self::OperationFailed => "operation_failed",
            Self::RetentionCompleted => "retention_completed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricName {
    BatchSize,
    Bytes,
    Count,
    DeletedCount,
    DurationMs,
    ErrorCount,
    FileCount,
    QueueDepth,
    RecordCount,
    RetryCount,
}

impl MetricName {
    const fn as_str(self) -> &'static str {
        match self {
            Self::BatchSize => "batch_size",
            Self::Bytes => "bytes",
            Self::Count => "count",
            Self::DeletedCount => "deleted_count",
            Self::DurationMs => "duration_ms",
            Self::ErrorCount => "error_count",
            Self::FileCount => "file_count",
            Self::QueueDepth => "queue_depth",
            Self::RecordCount => "record_count",
            Self::RetryCount => "retry_count",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationId(u64);

impl OperationId {
    pub const SYSTEM: Self = Self(0);

    pub const fn from_sequence(sequence: u64) -> Self {
        Self(sequence)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LogMetric {
    name: MetricName,
    value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredEvent {
    level: LogLevel,
    component: LogComponent,
    operation_id: OperationId,
    code: LogEventCode,
    metrics: Vec<LogMetric>,
}

impl StructuredEvent {
    pub fn new(
        level: LogLevel,
        component: LogComponent,
        operation_id: OperationId,
        code: LogEventCode,
    ) -> Self {
        Self {
            level,
            component,
            operation_id,
            code,
            metrics: Vec::with_capacity(MAX_METRICS),
        }
    }

    pub fn with_metric(mut self, name: MetricName, value: u64) -> Result<Self, LogEventError> {
        if self.metrics.len() >= MAX_METRICS {
            return Err(LogEventError::TooManyMetrics);
        }
        if self.metrics.iter().any(|metric| metric.name == name) {
            return Err(LogEventError::DuplicateMetric);
        }

        self.metrics.push(LogMetric { name, value });
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogEventError {
    TooManyMetrics,
    DuplicateMetric,
}

impl fmt::Display for LogEventError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::TooManyMetrics => "the structured log event has too many metrics",
            Self::DuplicateMetric => "the structured log event has a duplicate metric",
        })
    }
}

impl Error for LogEventError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogLimits {
    max_file_bytes: u64,
    max_archives: u8,
    retention: Duration,
}

impl LogLimits {
    pub fn new(
        max_file_bytes: u64,
        max_archives: u8,
        retention_days: u16,
    ) -> Result<Self, LogConfigError> {
        if !(MIN_FILE_BYTES..=MAX_FILE_BYTES).contains(&max_file_bytes)
            || !(1..=MAX_ARCHIVES).contains(&max_archives)
            || !(1..=MAX_RETENTION_DAYS).contains(&retention_days)
        {
            return Err(LogConfigError::InvalidLimits);
        }

        Ok(Self {
            max_file_bytes,
            max_archives,
            retention: Duration::from_secs(u64::from(retention_days) * 24 * 60 * 60),
        })
    }
}

impl Default for LogLimits {
    fn default() -> Self {
        Self {
            max_file_bytes: DEFAULT_MAX_FILE_BYTES,
            max_archives: DEFAULT_MAX_ARCHIVES,
            retention: Duration::from_secs(u64::from(DEFAULT_RETENTION_DAYS) * 24 * 60 * 60),
        }
    }
}

#[derive(Debug, Clone)]
struct LogConfig {
    log_directory: PathBuf,
    limits: LogLimits,
}

impl LogConfig {
    fn for_application_data_directory(
        application_data_directory: &Path,
        limits: LogLimits,
    ) -> Result<Self, LogConfigError> {
        if !application_data_directory.is_absolute() {
            return Err(LogConfigError::InvalidDirectory);
        }

        let canonical_directory = fs::canonicalize(application_data_directory)
            .map_err(|_| LogConfigError::UnavailableDirectory)?;
        if !fs::metadata(&canonical_directory)
            .map_err(|_| LogConfigError::UnavailableDirectory)?
            .is_dir()
        {
            return Err(LogConfigError::InvalidDirectory);
        }

        let log_directory = canonical_directory.join(LOG_DIRECTORY_NAME);
        if platform_path_length(&log_directory.join(ACTIVE_LOG_FILE_NAME)) > MAX_LOG_PATH_UNITS {
            return Err(LogConfigError::InvalidDirectory);
        }

        Ok(Self {
            log_directory,
            limits,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogConfigError {
    InvalidDirectory,
    UnavailableDirectory,
    InvalidLimits,
}

impl fmt::Display for LogConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidDirectory => "the application log directory is invalid",
            Self::UnavailableDirectory => "the application log directory is unavailable",
            Self::InvalidLimits => "the application log limits are invalid",
        })
    }
}

impl Error for LogConfigError {}

#[derive(Debug)]
pub enum LogError {
    Configuration(LogConfigError),
    Io(io::Error),
    Clock,
    Unavailable,
    EventTooLarge,
}

impl fmt::Display for LogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Configuration(_) => "the local logger configuration is invalid",
            Self::Io(_) => "the local log could not be updated",
            Self::Clock => "the local log timestamp is unavailable",
            Self::Unavailable => "the local logger is unavailable",
            Self::EventTooLarge => "the structured log event exceeds the file limit",
        })
    }
}

impl Error for LogError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Configuration(source) => Some(source),
            Self::Io(source) => Some(source),
            Self::Clock | Self::Unavailable | Self::EventTooLarge => None,
        }
    }
}

impl From<io::Error> for LogError {
    fn from(source: io::Error) -> Self {
        Self::Io(source)
    }
}

struct WriterState {
    file: Option<File>,
    length: u64,
}

pub struct LocalLogger {
    config: LogConfig,
    writer: Mutex<WriterState>,
}

impl LocalLogger {
    pub fn open_for_application_data_directory(
        application_data_directory: &Path,
    ) -> Result<Self, LogError> {
        Self::open_with_limits(application_data_directory, LogLimits::default())
    }

    pub fn open_with_limits(
        application_data_directory: &Path,
        limits: LogLimits,
    ) -> Result<Self, LogError> {
        let config = LogConfig::for_application_data_directory(application_data_directory, limits)
            .map_err(LogError::Configuration)?;
        prepare_log_directory(&config.log_directory)?;
        prune_expired_files(&config, SystemTime::now())?;
        let (file, length) = open_active_log(&config)?;

        Ok(Self {
            config,
            writer: Mutex::new(WriterState {
                file: Some(file),
                length,
            }),
        })
    }

    pub fn log(&self, event: StructuredEvent) -> Result<(), LogError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| LogError::Clock)?
            .as_millis();
        let line = format_event(timestamp, &event);
        let line_length = u64::try_from(line.len()).map_err(|_| LogError::EventTooLarge)?;
        if line_length > self.config.limits.max_file_bytes {
            return Err(LogError::EventTooLarge);
        }

        let mut writer = self.writer.lock().map_err(|_| LogError::Unavailable)?;
        if writer.length > 0
            && writer.length.saturating_add(line_length) > self.config.limits.max_file_bytes
        {
            rotate(&self.config, &mut writer, SystemTime::now())?;
        }

        let file = writer.file.as_mut().ok_or(LogError::Unavailable)?;
        file.write_all(line.as_bytes())?;
        writer.length = writer.length.saturating_add(line_length);
        Ok(())
    }

    pub fn clear(&self) -> Result<(), LogError> {
        let mut writer = self.writer.lock().map_err(|_| LogError::Unavailable)?;
        let close_result = close_writer_file(&mut writer);

        let clear_result = close_result.and_then(|()| clear_files(&self.config));
        match open_active_log(&self.config) {
            Ok((file, length)) => {
                writer.file = Some(file);
                writer.length = length;
                clear_result.map_err(LogError::Io)
            }
            Err(error) => Err(LogError::Io(error)),
        }
    }
}

fn format_event(timestamp_unix_ms: u128, event: &StructuredEvent) -> String {
    let mut line = format!(
        "{{\"timestamp_unix_ms\":{timestamp_unix_ms},\"level\":\"{}\",\"component\":\"{}\",\"operation_id\":{},\"event\":\"{}\",\"context\":{{",
        event.level.as_str(),
        event.component.as_str(),
        event.operation_id.0,
        event.code.as_str(),
    );

    for (index, metric) in event.metrics.iter().enumerate() {
        if index > 0 {
            line.push(',');
        }
        line.push('"');
        line.push_str(metric.name.as_str());
        line.push_str("\":");
        line.push_str(&metric.value.to_string());
    }
    line.push_str("}}\n");
    line
}

fn prepare_log_directory(log_directory: &Path) -> io::Result<()> {
    match create_private_log_directory(log_directory) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            validate_existing_log_directory(log_directory)
        }
        Err(error) => Err(error),
    }
}

#[cfg(windows)]
fn create_private_log_directory(log_directory: &Path) -> io::Result<()> {
    fs::create_dir(log_directory)
}

#[cfg(unix)]
fn create_private_log_directory(log_directory: &Path) -> io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;

    let mut builder = std::fs::DirBuilder::new();
    builder.mode(0o700).create(log_directory)
}

fn validate_existing_log_directory(log_directory: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(log_directory)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "application log path is not a regular directory",
        ));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "application log directory permissions are too broad",
            ));
        }
    }

    Ok(())
}

fn open_active_log(config: &LogConfig) -> io::Result<(File, u64)> {
    let path = config.log_directory.join(ACTIVE_LOG_FILE_NAME);
    validate_existing_log_file_if_present(&path)?;

    let mut options = OpenOptions::new();
    options.create(true).append(true).read(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(0o600);
    }

    let file = options.open(path)?;
    let length = file.metadata()?.len();
    Ok((file, length))
}

fn validate_existing_log_file_if_present(path: &Path) -> io::Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };

    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "application log path is not a regular file",
        ));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "application log file permissions are too broad",
            ));
        }
    }

    Ok(())
}

fn rotate(config: &LogConfig, writer: &mut WriterState, now: SystemTime) -> io::Result<()> {
    let rotation_result = close_writer_file(writer).and_then(|()| rotate_files(config, now));
    let reopen_result = open_active_log(config);
    match reopen_result {
        Ok((file, length)) => {
            writer.file = Some(file);
            writer.length = length;
            rotation_result
        }
        Err(error) => Err(error),
    }
}

fn close_writer_file(writer: &mut WriterState) -> io::Result<()> {
    let Some(mut file) = writer.file.take() else {
        return Ok(());
    };

    file.flush().and_then(|()| file.sync_data())
}

fn rotate_files(config: &LogConfig, now: SystemTime) -> io::Result<()> {
    prune_expired_files(config, now)?;

    for index in (1..=config.limits.max_archives).rev() {
        let target = archive_path(&config.log_directory, index);
        remove_if_safe_file(&target)?;

        let source = if index == 1 {
            config.log_directory.join(ACTIVE_LOG_FILE_NAME)
        } else {
            archive_path(&config.log_directory, index - 1)
        };
        validate_existing_log_file_if_present(&source)?;
        if source.exists() {
            fs::rename(source, target)?;
        }
    }
    Ok(())
}

fn clear_files(config: &LogConfig) -> io::Result<()> {
    remove_if_safe_file(&config.log_directory.join(ACTIVE_LOG_FILE_NAME))?;
    for index in 1..=MAX_ARCHIVES {
        remove_if_safe_file(&archive_path(&config.log_directory, index))?;
    }
    Ok(())
}

fn prune_expired_files(config: &LogConfig, now: SystemTime) -> io::Result<()> {
    let active = config.log_directory.join(ACTIVE_LOG_FILE_NAME);
    remove_if_expired_or_oversized(
        &active,
        now,
        config.limits.retention,
        config.limits.max_file_bytes,
    )?;
    for index in 1..=MAX_ARCHIVES {
        let archive = archive_path(&config.log_directory, index);
        if index > config.limits.max_archives {
            remove_if_safe_file(&archive)?;
            continue;
        }
        remove_if_expired_or_oversized(
            &archive,
            now,
            config.limits.retention,
            config.limits.max_file_bytes,
        )?;
    }
    Ok(())
}

fn remove_if_expired_or_oversized(
    path: &Path,
    now: SystemTime,
    retention: Duration,
    max_file_bytes: u64,
) -> io::Result<()> {
    validate_existing_log_file_if_present(path)?;
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    let modified = metadata.modified()?;
    if metadata.len() > max_file_bytes
        || now
            .duration_since(modified)
            .is_ok_and(|age| age >= retention)
    {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn remove_if_safe_file(path: &Path) -> io::Result<()> {
    validate_existing_log_file_if_present(path)?;
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn archive_path(log_directory: &Path, index: u8) -> PathBuf {
    log_directory.join(format!("{ACTIVE_LOG_FILE_NAME}.{index}"))
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

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::Write,
        path::{Path, PathBuf},
        sync::{
            Arc,
            atomic::{AtomicU64, Ordering},
        },
        thread,
        time::{Duration, SystemTime},
    };

    use serde_json::Value;

    use super::{
        ACTIVE_LOG_FILE_NAME, LOG_DIRECTORY_NAME, LocalLogger, LogComponent, LogConfigError,
        LogError, LogEventCode, LogEventError, LogLevel, LogLimits, MetricName, OperationId,
        StructuredEvent, archive_path,
    };

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn create() -> Self {
            let sequence = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "dev-recall-observability-test-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir(&path).expect("test directory should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn log_directory(&self) -> PathBuf {
            self.path.join(super::LOG_DIRECTORY_NAME)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn create_private_fixture(path: &Path, bytes: &[u8]) -> fs::File {
        let mut options = fs::OpenOptions::new();
        options.create_new(true).write(true);

        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;

            options.mode(0o600);
        }

        let mut file = options
            .open(path)
            .expect("private fixture should be created");
        file.write_all(bytes)
            .expect("private fixture should be written");
        file
    }

    #[test]
    fn rejects_invalid_limits_and_duplicate_metrics() {
        assert_eq!(
            LogLimits::new(1, 1, 1).unwrap_err(),
            LogConfigError::InvalidLimits
        );
        assert_eq!(
            LogLimits::new(4096, 0, 1).unwrap_err(),
            LogConfigError::InvalidLimits
        );

        let event = StructuredEvent::new(
            LogLevel::Info,
            LogComponent::Application,
            OperationId::SYSTEM,
            LogEventCode::OperationSucceeded,
        )
        .with_metric(MetricName::Count, 1)
        .expect("first metric should be accepted");
        assert_eq!(
            event.with_metric(MetricName::Count, 2).unwrap_err(),
            LogEventError::DuplicateMetric
        );

        let full_event = StructuredEvent::new(
            LogLevel::Info,
            LogComponent::Application,
            OperationId::SYSTEM,
            LogEventCode::OperationSucceeded,
        )
        .with_metric(MetricName::BatchSize, 1)
        .and_then(|event| event.with_metric(MetricName::Bytes, 2))
        .and_then(|event| event.with_metric(MetricName::Count, 3))
        .and_then(|event| event.with_metric(MetricName::DeletedCount, 4))
        .and_then(|event| event.with_metric(MetricName::DurationMs, 5))
        .and_then(|event| event.with_metric(MetricName::ErrorCount, 6))
        .and_then(|event| event.with_metric(MetricName::FileCount, 7))
        .and_then(|event| event.with_metric(MetricName::QueueDepth, 8))
        .expect("eight metrics should be accepted");
        assert_eq!(
            full_event
                .with_metric(MetricName::RecordCount, 9)
                .unwrap_err(),
            LogEventError::TooManyMetrics
        );
    }

    #[test]
    fn rejects_relative_and_non_directory_application_paths() {
        let relative = LocalLogger::open_for_application_data_directory(Path::new("relative"));
        assert!(matches!(
            relative,
            Err(LogError::Configuration(LogConfigError::InvalidDirectory))
        ));

        let directory = TestDirectory::create();
        let file_path = directory.path().join("not-a-directory");
        fs::write(&file_path, b"synthetic fixture").expect("fixture file should be created");
        let non_directory = LocalLogger::open_for_application_data_directory(&file_path);
        assert!(matches!(
            non_directory,
            Err(LogError::Configuration(LogConfigError::InvalidDirectory))
        ));
    }

    #[test]
    fn rejects_non_directory_log_target() {
        let directory = TestDirectory::create();
        fs::write(
            directory.path().join(LOG_DIRECTORY_NAME),
            b"synthetic fixture",
        )
        .expect("unsafe log target should be created");

        let result = LocalLogger::open_for_application_data_directory(directory.path());
        assert!(matches!(result, Err(LogError::Io(_))));
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_log_file() {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::create();
        let logger = LocalLogger::open_for_application_data_directory(directory.path())
            .expect("logger should open");
        drop(logger);

        let active_log = directory.log_directory().join(ACTIVE_LOG_FILE_NAME);
        fs::remove_file(&active_log).expect("active fixture should be removed");
        let target = directory.path().join("outside-log-target");
        fs::write(&target, b"synthetic fixture").expect("target fixture should be created");
        symlink(&target, &active_log).expect("log symlink fixture should be created");

        let result = LocalLogger::open_for_application_data_directory(directory.path());
        assert!(matches!(result, Err(LogError::Io(_))));
    }

    #[test]
    fn writes_fixed_schema_without_arbitrary_text_fields() {
        let directory = TestDirectory::create();
        let logger = LocalLogger::open_for_application_data_directory(directory.path())
            .expect("logger should open");
        let event = StructuredEvent::new(
            LogLevel::Warn,
            LogComponent::Persistence,
            OperationId::from_sequence(42),
            LogEventCode::OperationFailed,
        )
        .with_metric(MetricName::RetryCount, 3)
        .expect("metric should be accepted");

        logger.log(event).expect("event should be written");
        drop(logger);

        let text = fs::read_to_string(directory.log_directory().join(ACTIVE_LOG_FILE_NAME))
            .expect("log should be readable");
        let value: Value = serde_json::from_str(text.trim()).expect("log line should be JSON");
        assert!(value["timestamp_unix_ms"].is_u64());
        assert_eq!(value["level"], "warn");
        assert_eq!(value["component"], "persistence");
        assert_eq!(value["operation_id"], 42);
        assert_eq!(value["event"], "operation_failed");
        assert_eq!(value["context"]["retry_count"], 3);
        assert_eq!(value.as_object().map(|object| object.len()), Some(6));
    }

    #[test]
    fn rotates_at_size_limit_and_bounds_archive_count() {
        let directory = TestDirectory::create();
        let limits = LogLimits::new(4096, 2, 7).expect("test limits should be accepted");
        let logger =
            LocalLogger::open_with_limits(directory.path(), limits).expect("logger should open");

        for sequence in 1..=100 {
            logger
                .log(StructuredEvent::new(
                    LogLevel::Info,
                    LogComponent::Application,
                    OperationId::from_sequence(sequence),
                    LogEventCode::OperationSucceeded,
                ))
                .expect("bounded event should be written");
        }
        drop(logger);

        let log_directory = directory.log_directory();
        assert!(log_directory.join(ACTIVE_LOG_FILE_NAME).is_file());
        assert!(archive_path(&log_directory, 1).is_file());
        assert!(archive_path(&log_directory, 2).is_file());
        assert!(!archive_path(&log_directory, 3).exists());
        for path in [
            log_directory.join(ACTIVE_LOG_FILE_NAME),
            archive_path(&log_directory, 1),
            archive_path(&log_directory, 2),
        ] {
            assert!(
                fs::metadata(path).expect("log metadata should exist").len() <= 4096,
                "each log file should respect the configured size"
            );
        }
    }

    #[test]
    fn prunes_expired_archives_on_open() {
        let directory = TestDirectory::create();
        let logger = LocalLogger::open_for_application_data_directory(directory.path())
            .expect("logger should open");
        drop(logger);

        let archive = archive_path(&directory.log_directory(), 1);
        let file = create_private_fixture(&archive, b"synthetic fixture");
        file.set_times(
            fs::FileTimes::new().set_modified(SystemTime::now() - Duration::from_secs(8 * 86_400)),
        )
        .expect("archive timestamp should be set");
        drop(file);

        let logger = LocalLogger::open_for_application_data_directory(directory.path())
            .expect("logger should prune and reopen");
        assert!(!archive.exists());
        drop(logger);
    }

    #[test]
    fn prunes_oversized_and_excess_archives_on_open() {
        let directory = TestDirectory::create();
        let limits = LogLimits::new(4096, 1, 7).expect("test limits should be accepted");
        let logger =
            LocalLogger::open_with_limits(directory.path(), limits).expect("logger should open");
        drop(logger);

        let oversized = archive_path(&directory.log_directory(), 1);
        drop(create_private_fixture(&oversized, &vec![0_u8; 4097]));
        let excess = archive_path(&directory.log_directory(), 2);
        drop(create_private_fixture(&excess, b"synthetic fixture"));

        let logger = LocalLogger::open_with_limits(directory.path(), limits)
            .expect("logger should prune and reopen");
        assert!(!oversized.exists());
        assert!(!excess.exists());
        drop(logger);
    }

    #[test]
    fn clear_removes_archives_and_truncates_active_log() {
        let directory = TestDirectory::create();
        let limits = LogLimits::new(4096, 2, 7).expect("test limits should be accepted");
        let logger =
            LocalLogger::open_with_limits(directory.path(), limits).expect("logger should open");
        for sequence in 1..=50 {
            logger
                .log(StructuredEvent::new(
                    LogLevel::Info,
                    LogComponent::Privacy,
                    OperationId::from_sequence(sequence),
                    LogEventCode::RetentionCompleted,
                ))
                .expect("event should be written");
        }

        logger.clear().expect("logs should clear");
        assert_eq!(
            fs::metadata(directory.log_directory().join(ACTIVE_LOG_FILE_NAME))
                .expect("active log should remain")
                .len(),
            0
        );
        assert!(!archive_path(&directory.log_directory(), 1).exists());
    }

    #[test]
    fn serializes_concurrent_writers_without_losing_events() {
        let directory = TestDirectory::create();
        let logger = Arc::new(
            LocalLogger::open_for_application_data_directory(directory.path())
                .expect("logger should open"),
        );
        let mut workers = Vec::new();
        for worker in 0..4_u64 {
            let logger = Arc::clone(&logger);
            workers.push(thread::spawn(move || {
                for sequence in 1..=25_u64 {
                    logger
                        .log(StructuredEvent::new(
                            LogLevel::Debug,
                            LogComponent::Ipc,
                            OperationId::from_sequence(worker * 100 + sequence),
                            LogEventCode::OperationSucceeded,
                        ))
                        .expect("concurrent event should be written");
                }
            }));
        }
        for worker in workers {
            worker.join().expect("worker should finish");
        }
        drop(logger);

        let text = fs::read_to_string(directory.log_directory().join(ACTIVE_LOG_FILE_NAME))
            .expect("log should be readable");
        let lines: Vec<_> = text.lines().collect();
        assert_eq!(lines.len(), 100);
        assert!(
            lines
                .iter()
                .all(|line| serde_json::from_str::<Value>(line).is_ok())
        );
    }

    #[cfg(unix)]
    #[test]
    fn creates_private_directory_and_file_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let directory = TestDirectory::create();
        let logger = LocalLogger::open_for_application_data_directory(directory.path())
            .expect("logger should open");
        drop(logger);

        let directory_mode = fs::metadata(directory.log_directory())
            .expect("log directory metadata should exist")
            .permissions()
            .mode();
        let file_mode = fs::metadata(directory.log_directory().join(ACTIVE_LOG_FILE_NAME))
            .expect("log file metadata should exist")
            .permissions()
            .mode();
        assert_eq!(directory_mode & 0o777, 0o700);
        assert_eq!(file_mode & 0o777, 0o600);
    }
}
