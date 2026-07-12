#![deny(unsafe_code)]

use std::{
    error::Error,
    fmt, fs,
    path::{Component, Path, PathBuf},
};

pub const MAX_PROJECT_PATH_UNITS: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectPathError {
    Empty,
    InvalidUnicode,
    TooLong,
    NotAbsolute,
    Traversal,
    SpecialPath,
    Unavailable,
    NotDirectory,
    Symlink,
    OutsideProjectRoot,
}

impl fmt::Display for ProjectPathError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "project path is empty",
            Self::InvalidUnicode => "project path contains unsupported text encoding",
            Self::TooLong => "project path exceeds the supported length",
            Self::NotAbsolute => "project path is not absolute",
            Self::Traversal => "project path contains a traversal component",
            Self::SpecialPath => "project path targets a special filesystem location",
            Self::Unavailable => "project path is unavailable",
            Self::NotDirectory => "project path is not a directory",
            Self::Symlink => "project path contains a symbolic link",
            Self::OutsideProjectRoot => "path leaves the approved project root",
        };

        formatter.write_str(message)
    }
}

impl Error for ProjectPathError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedProjectDirectory {
    selected_path: PathBuf,
    canonical_path: PathBuf,
}

impl ValidatedProjectDirectory {
    pub fn validate(path: &Path) -> Result<Self, ProjectPathError> {
        validate_path_shape(path, true)?;
        reject_platform_special_path(path, false)?;
        inspect_existing_components(path, true)?;

        let canonical_path = fs::canonicalize(path).map_err(|_| ProjectPathError::Unavailable)?;
        validate_path_shape(&canonical_path, true)?;
        reject_platform_special_path(&canonical_path, true)?;
        inspect_existing_components(path, true)?;

        let metadata =
            fs::symlink_metadata(&canonical_path).map_err(|_| ProjectPathError::Unavailable)?;
        if metadata.file_type().is_symlink() {
            return Err(ProjectPathError::Symlink);
        }
        if !metadata.is_dir() {
            return Err(ProjectPathError::NotDirectory);
        }

        Ok(Self {
            selected_path: path.to_path_buf(),
            canonical_path,
        })
    }

    pub fn selected_path(&self) -> &Path {
        &self.selected_path
    }

    pub fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    pub fn resolve_existing(&self, relative_path: &Path) -> Result<PathBuf, ProjectPathError> {
        validate_relative_path(relative_path)?;
        let candidate = self.canonical_path.join(relative_path);
        if platform_path_length(&candidate) > MAX_PROJECT_PATH_UNITS {
            return Err(ProjectPathError::TooLong);
        }
        reject_platform_special_path(&candidate, true)?;
        inspect_existing_components(&candidate, false)?;

        let canonical_candidate =
            fs::canonicalize(&candidate).map_err(|_| ProjectPathError::Unavailable)?;
        reject_platform_special_path(&canonical_candidate, true)?;
        if !canonical_candidate.starts_with(&self.canonical_path) {
            return Err(ProjectPathError::OutsideProjectRoot);
        }
        inspect_existing_components(&candidate, false)?;

        Ok(canonical_candidate)
    }
}

fn validate_path_shape(path: &Path, require_absolute: bool) -> Result<(), ProjectPathError> {
    if path.as_os_str().is_empty() {
        return Err(ProjectPathError::Empty);
    }
    if path.to_str().is_none() {
        return Err(ProjectPathError::InvalidUnicode);
    }
    if platform_path_length(path) > MAX_PROJECT_PATH_UNITS {
        return Err(ProjectPathError::TooLong);
    }
    if require_absolute && !path.is_absolute() {
        return Err(ProjectPathError::NotAbsolute);
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ProjectPathError::Traversal);
    }

    Ok(())
}

fn validate_relative_path(path: &Path) -> Result<(), ProjectPathError> {
    validate_path_shape(path, false)?;
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::Prefix(_) | Component::RootDir | Component::ParentDir
            )
        })
    {
        return Err(ProjectPathError::Traversal);
    }

    Ok(())
}

fn inspect_existing_components(
    path: &Path,
    final_must_be_directory: bool,
) -> Result<(), ProjectPathError> {
    let mut current = PathBuf::new();
    let mut saw_normal_component = false;

    for component in path.components() {
        current.push(component.as_os_str());
        if !matches!(component, Component::Normal(_)) {
            continue;
        }
        saw_normal_component = true;

        let metadata = fs::symlink_metadata(&current).map_err(|_| ProjectPathError::Unavailable)?;
        if metadata.file_type().is_symlink() {
            return Err(ProjectPathError::Symlink);
        }
    }

    if !saw_normal_component {
        return Err(ProjectPathError::SpecialPath);
    }

    let metadata = fs::symlink_metadata(path).map_err(|_| ProjectPathError::Unavailable)?;
    if metadata.file_type().is_symlink() {
        return Err(ProjectPathError::Symlink);
    }
    if final_must_be_directory && !metadata.is_dir() {
        return Err(ProjectPathError::NotDirectory);
    }

    Ok(())
}

#[cfg(windows)]
fn reject_platform_special_path(
    path: &Path,
    allow_canonical_verbatim_disk: bool,
) -> Result<(), ProjectPathError> {
    use std::path::Prefix;

    let mut components = path.components();
    match components.next() {
        Some(Component::Prefix(prefix))
            if matches!(prefix.kind(), Prefix::Disk(_))
                || (allow_canonical_verbatim_disk
                    && matches!(prefix.kind(), Prefix::VerbatimDisk(_))) => {}
        _ => return Err(ProjectPathError::SpecialPath),
    }

    for component in components {
        let Component::Normal(value) = component else {
            continue;
        };
        let Some(value) = value.to_str() else {
            return Err(ProjectPathError::InvalidUnicode);
        };
        if is_unsafe_windows_component(value) {
            return Err(ProjectPathError::SpecialPath);
        }
    }

    Ok(())
}

#[cfg(windows)]
fn is_unsafe_windows_component(value: &str) -> bool {
    if value.contains(':') || value.ends_with([' ', '.']) {
        return true;
    }

    let stem = value.split('.').next().unwrap_or_default();
    let stem = stem.to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || stem
            .strip_prefix("COM")
            .is_some_and(is_reserved_device_number)
        || stem
            .strip_prefix("LPT")
            .is_some_and(is_reserved_device_number)
}

#[cfg(windows)]
fn is_reserved_device_number(value: &str) -> bool {
    matches!(value, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
}

#[cfg(unix)]
fn reject_platform_special_path(
    path: &Path,
    _allow_canonical_verbatim_disk: bool,
) -> Result<(), ProjectPathError> {
    const SPECIAL_ROOTS: [&str; 3] = ["/dev", "/proc", "/sys"];

    if SPECIAL_ROOTS
        .iter()
        .any(|root| path == Path::new(root) || path.starts_with(Path::new(root)))
    {
        return Err(ProjectPathError::SpecialPath);
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

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::{
        MAX_PROJECT_PATH_UNITS, ProjectPathError, ValidatedProjectDirectory, platform_path_length,
    };

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn create() -> Self {
            let sequence = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "dev-recall-platform-test-{}-{sequence}",
                std::process::id()
            ));
            if let Err(error) = fs::create_dir(&path) {
                panic!("test directory must be created: {error}");
            }
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

    fn validated(directory: &TestDirectory) -> ValidatedProjectDirectory {
        match ValidatedProjectDirectory::validate(directory.path()) {
            Ok(value) => value,
            Err(error) => panic!("test directory must validate: {error}"),
        }
    }

    #[test]
    fn validates_existing_directory_and_records_canonical_path() {
        let directory = TestDirectory::create();
        let project = validated(&directory);

        assert_eq!(project.selected_path(), directory.path());
        assert_eq!(
            project.canonical_path(),
            directory
                .path()
                .canonicalize()
                .ok()
                .as_deref()
                .unwrap_or(Path::new(""))
        );
    }

    #[test]
    fn rejects_relative_traversal_missing_and_file_paths() {
        assert_eq!(
            ValidatedProjectDirectory::validate(Path::new("relative")),
            Err(ProjectPathError::NotAbsolute)
        );

        let directory = TestDirectory::create();
        let traversal = directory.path().join("child").join("..");
        assert_eq!(
            ValidatedProjectDirectory::validate(&traversal),
            Err(ProjectPathError::Traversal)
        );
        assert_eq!(
            ValidatedProjectDirectory::validate(&directory.path().join("missing")),
            Err(ProjectPathError::Unavailable)
        );

        let file = directory.path().join("file");
        if let Err(error) = fs::write(&file, b"fixture") {
            panic!("test file must be created: {error}");
        }
        assert_eq!(
            ValidatedProjectDirectory::validate(&file),
            Err(ProjectPathError::NotDirectory)
        );
    }

    #[test]
    fn rejects_paths_over_the_platform_limit_before_filesystem_access() {
        let oversized = std::env::temp_dir().join("x".repeat(MAX_PROJECT_PATH_UNITS + 1));
        assert!(platform_path_length(&oversized) > MAX_PROJECT_PATH_UNITS);
        assert_eq!(
            ValidatedProjectDirectory::validate(&oversized),
            Err(ProjectPathError::TooLong)
        );
    }

    #[test]
    fn resolves_only_existing_entries_inside_the_approved_root() {
        let directory = TestDirectory::create();
        let nested = directory.path().join("nested");
        if let Err(error) = fs::create_dir(&nested) {
            panic!("nested test directory must be created: {error}");
        }
        let file = nested.join("manifest.txt");
        if let Err(error) = fs::write(&file, b"fixture") {
            panic!("test file must be created: {error}");
        }
        let project = validated(&directory);

        assert_eq!(
            project.resolve_existing(Path::new("nested/manifest.txt")),
            file.canonicalize()
                .map_err(|_| ProjectPathError::Unavailable)
        );
        assert_eq!(
            project.resolve_existing(Path::new("../outside")),
            Err(ProjectPathError::Traversal)
        );
        assert_eq!(
            project.resolve_existing(Path::new("missing")),
            Err(ProjectPathError::Unavailable)
        );
    }

    #[cfg(unix)]
    #[test]
    fn rejects_invalid_unicode_and_virtual_filesystem_roots() {
        use std::os::unix::ffi::OsStringExt;

        let invalid = PathBuf::from(std::ffi::OsString::from_vec(vec![b'/', 0xff]));
        assert_eq!(
            ValidatedProjectDirectory::validate(&invalid),
            Err(ProjectPathError::InvalidUnicode)
        );
        assert_eq!(
            ValidatedProjectDirectory::validate(Path::new("/proc")),
            Err(ProjectPathError::SpecialPath)
        );
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn rejects_root_and_nested_symlinks() {
        let directory = TestDirectory::create();
        let target = directory.path().join("target");
        let link = directory.path().join("link");
        if let Err(error) = fs::create_dir(&target) {
            panic!("symlink target must be created: {error}");
        }
        if !create_directory_symlink(&target, &link) {
            return;
        }

        assert_eq!(
            ValidatedProjectDirectory::validate(&link),
            Err(ProjectPathError::Symlink)
        );

        let project = validated(&directory);
        assert_eq!(
            project.resolve_existing(Path::new("link")),
            Err(ProjectPathError::Symlink)
        );
    }

    #[cfg(unix)]
    fn create_directory_symlink(target: &Path, link: &Path) -> bool {
        match std::os::unix::fs::symlink(target, link) {
            Ok(()) => true,
            Err(error) => panic!("symlink fixture must be created: {error}"),
        }
    }

    #[cfg(windows)]
    fn create_directory_symlink(target: &Path, link: &Path) -> bool {
        match std::os::windows::fs::symlink_dir(target, link) {
            Ok(()) => true,
            Err(error)
                if error.kind() == std::io::ErrorKind::PermissionDenied
                    || error.raw_os_error() == Some(1_314) =>
            {
                false
            }
            Err(error) => panic!("symlink fixture must be created: {error}"),
        }
    }

    #[cfg(windows)]
    #[test]
    fn rejects_windows_unc_device_reserved_and_alternate_stream_paths() {
        for path in [
            r"\\server\share\project",
            r"\\?\C:\project",
            r"\\.\C:\project",
            r"C:\NUL",
            r"C:\COM1.txt",
            r"C:\project:stream",
        ] {
            assert_eq!(
                ValidatedProjectDirectory::validate(Path::new(path)),
                Err(ProjectPathError::SpecialPath)
            );
        }
    }

    #[cfg(windows)]
    #[test]
    fn rejects_invalid_windows_unicode() {
        use std::os::windows::ffi::OsStringExt;

        let invalid = PathBuf::from(std::ffi::OsString::from_wide(&[
            b'C' as u16,
            b':' as u16,
            b'\\' as u16,
            0xd800,
        ]));
        assert_eq!(
            ValidatedProjectDirectory::validate(&invalid),
            Err(ProjectPathError::InvalidUnicode)
        );
    }
}
