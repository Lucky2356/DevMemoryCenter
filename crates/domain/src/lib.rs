#![deny(unsafe_code)]

use std::{error::Error, fmt, path::PathBuf};

pub const MAX_PROJECT_DISPLAY_NAME_BYTES: usize = 128;
pub const MAX_PROJECT_DESCRIPTION_BYTES: usize = 4_096;
pub const MAX_TIMESTAMP_MILLIS: u64 = i64::MAX as u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectValidationError {
    InvalidProjectId,
    InvalidOwnerId,
    EmptyRootPath,
    EmptyCanonicalPath,
    EmptyDisplayName,
    DisplayNameTooLong,
    DescriptionTooLong,
    DisallowedTextControl,
    TimestampOutOfRange,
    UpdatedBeforeCreated,
    ArchiveBeforeCreated,
    ArchiveAfterUpdated,
}

impl fmt::Display for ProjectValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidProjectId => "project identifier is not a valid UUID",
            Self::InvalidOwnerId => "owner identifier is not a valid UUID",
            Self::EmptyRootPath => "project root path is empty",
            Self::EmptyCanonicalPath => "project canonical path is empty",
            Self::EmptyDisplayName => "project display name is empty",
            Self::DisplayNameTooLong => "project display name exceeds its size limit",
            Self::DescriptionTooLong => "project description exceeds its size limit",
            Self::DisallowedTextControl => "project text contains a disallowed control character",
            Self::TimestampOutOfRange => "project timestamp exceeds the supported range",
            Self::UpdatedBeforeCreated => "project update time precedes creation time",
            Self::ArchiveBeforeCreated => "project archive time precedes creation time",
            Self::ArchiveAfterUpdated => "project archive time follows the last update time",
        };

        formatter.write_str(message)
    }
}

impl Error for ProjectValidationError {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProjectId([u8; 16]);

impl ProjectId {
    pub fn from_uuid_bytes(value: [u8; 16]) -> Result<Self, ProjectValidationError> {
        if !is_valid_uuid(value) {
            return Err(ProjectValidationError::InvalidProjectId);
        }

        Ok(Self(value))
    }

    pub const fn as_uuid_bytes(self) -> [u8; 16] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OwnerId([u8; 16]);

impl OwnerId {
    pub fn from_uuid_bytes(value: [u8; 16]) -> Result<Self, ProjectValidationError> {
        if !is_valid_uuid(value) {
            return Err(ProjectValidationError::InvalidOwnerId);
        }

        Ok(Self(value))
    }

    pub const fn as_uuid_bytes(self) -> [u8; 16] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TimestampMillis(u64);

impl TimestampMillis {
    pub const fn from_unix_millis(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_unix_millis(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DataSourceConsent {
    #[default]
    Disabled,
    Enabled,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProjectPrivacySettings {
    pub git_context: DataSourceConsent,
    pub manifest_detection: DataSourceConsent,
    pub terminal_history_association: DataSourceConsent,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProjectType {
    #[default]
    Unknown,
    Rust,
    NodeJs,
    Python,
    Go,
    Java,
    Docker,
    Mixed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectPaths {
    root_path: PathBuf,
    canonical_path: PathBuf,
}

impl ProjectPaths {
    pub fn new(
        root_path: PathBuf,
        canonical_path: PathBuf,
    ) -> Result<Self, ProjectValidationError> {
        if root_path.as_os_str().is_empty() {
            return Err(ProjectValidationError::EmptyRootPath);
        }
        if canonical_path.as_os_str().is_empty() {
            return Err(ProjectValidationError::EmptyCanonicalPath);
        }

        Ok(Self {
            root_path,
            canonical_path,
        })
    }

    pub fn root_path(&self) -> &std::path::Path {
        &self.root_path
    }

    pub fn canonical_path(&self) -> &std::path::Path {
        &self.canonical_path
    }
}

pub struct ProjectInput {
    pub id: ProjectId,
    pub owner_id: OwnerId,
    pub display_name: String,
    pub paths: ProjectPaths,
    pub project_type: ProjectType,
    pub description: Option<String>,
    pub created_at: TimestampMillis,
    pub updated_at: TimestampMillis,
    pub archived_at: Option<TimestampMillis>,
    pub privacy: ProjectPrivacySettings,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Project {
    id: ProjectId,
    owner_id: OwnerId,
    display_name: String,
    paths: ProjectPaths,
    project_type: ProjectType,
    description: Option<String>,
    created_at: TimestampMillis,
    updated_at: TimestampMillis,
    archived_at: Option<TimestampMillis>,
    privacy: ProjectPrivacySettings,
}

impl Project {
    pub fn new(input: ProjectInput) -> Result<Self, ProjectValidationError> {
        let display_name = input.display_name.trim();
        validate_display_name(display_name)?;

        let description = normalize_description(input.description)?;
        validate_timestamps(input.created_at, input.updated_at, input.archived_at)?;

        Ok(Self {
            id: input.id,
            owner_id: input.owner_id,
            display_name: display_name.to_owned(),
            paths: input.paths,
            project_type: input.project_type,
            description,
            created_at: input.created_at,
            updated_at: input.updated_at,
            archived_at: input.archived_at,
            privacy: input.privacy,
        })
    }

    pub const fn id(&self) -> ProjectId {
        self.id
    }

    pub const fn owner_id(&self) -> OwnerId {
        self.owner_id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub const fn paths(&self) -> &ProjectPaths {
        &self.paths
    }

    pub const fn project_type(&self) -> ProjectType {
        self.project_type
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub const fn created_at(&self) -> TimestampMillis {
        self.created_at
    }

    pub const fn updated_at(&self) -> TimestampMillis {
        self.updated_at
    }

    pub const fn archived_at(&self) -> Option<TimestampMillis> {
        self.archived_at
    }

    pub const fn privacy(&self) -> ProjectPrivacySettings {
        self.privacy
    }

    pub const fn is_archived(&self) -> bool {
        self.archived_at.is_some()
    }
}

fn is_valid_uuid(value: [u8; 16]) -> bool {
    let version = value[6] >> 4;
    let has_rfc_4122_variant = value[8] & 0b1100_0000 == 0b1000_0000;
    value != [0; 16] && (1..=8).contains(&version) && has_rfc_4122_variant
}

fn validate_display_name(value: &str) -> Result<(), ProjectValidationError> {
    if value.is_empty() {
        return Err(ProjectValidationError::EmptyDisplayName);
    }
    if value.len() > MAX_PROJECT_DISPLAY_NAME_BYTES {
        return Err(ProjectValidationError::DisplayNameTooLong);
    }
    if contains_disallowed_control(value, false) {
        return Err(ProjectValidationError::DisallowedTextControl);
    }

    Ok(())
}

fn normalize_description(value: Option<String>) -> Result<Option<String>, ProjectValidationError> {
    let Some(value) = value else {
        return Ok(None);
    };
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    if value.len() > MAX_PROJECT_DESCRIPTION_BYTES {
        return Err(ProjectValidationError::DescriptionTooLong);
    }
    if contains_disallowed_control(value, true) {
        return Err(ProjectValidationError::DisallowedTextControl);
    }

    Ok(Some(value.to_owned()))
}

fn contains_disallowed_control(value: &str, allow_layout_controls: bool) -> bool {
    value.chars().any(|character| {
        let allowed_layout_control =
            allow_layout_controls && matches!(character, '\n' | '\r' | '\t');
        (character.is_control() && !allowed_layout_control) || is_bidi_control(character)
    })
}

fn is_bidi_control(character: char) -> bool {
    matches!(
        character,
        '\u{061c}'
            | '\u{200e}'
            | '\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2066}'..='\u{2069}'
    )
}

fn validate_timestamps(
    created_at: TimestampMillis,
    updated_at: TimestampMillis,
    archived_at: Option<TimestampMillis>,
) -> Result<(), ProjectValidationError> {
    if created_at.as_unix_millis() > MAX_TIMESTAMP_MILLIS
        || updated_at.as_unix_millis() > MAX_TIMESTAMP_MILLIS
        || archived_at.is_some_and(|value| value.as_unix_millis() > MAX_TIMESTAMP_MILLIS)
    {
        return Err(ProjectValidationError::TimestampOutOfRange);
    }
    if updated_at < created_at {
        return Err(ProjectValidationError::UpdatedBeforeCreated);
    }
    if let Some(archived_at) = archived_at {
        if archived_at < created_at {
            return Err(ProjectValidationError::ArchiveBeforeCreated);
        }
        if archived_at > updated_at {
            return Err(ProjectValidationError::ArchiveAfterUpdated);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{
        DataSourceConsent, MAX_PROJECT_DESCRIPTION_BYTES, MAX_PROJECT_DISPLAY_NAME_BYTES,
        MAX_TIMESTAMP_MILLIS, OwnerId, Project, ProjectId, ProjectInput, ProjectPaths,
        ProjectPrivacySettings, ProjectType, ProjectValidationError, TimestampMillis,
    };

    const PROJECT_UUID: [u8; 16] = [
        0x01, 0x91, 0x75, 0x2f, 0x9a, 0x30, 0x70, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01,
    ];
    const OWNER_UUID: [u8; 16] = [
        0x01, 0x91, 0x75, 0x2f, 0x9a, 0x30, 0x70, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x02,
    ];

    fn project_id() -> ProjectId {
        match ProjectId::from_uuid_bytes(PROJECT_UUID) {
            Ok(value) => value,
            Err(error) => panic!("test project UUID must be valid: {error}"),
        }
    }

    fn owner_id() -> OwnerId {
        match OwnerId::from_uuid_bytes(OWNER_UUID) {
            Ok(value) => value,
            Err(error) => panic!("test owner UUID must be valid: {error}"),
        }
    }

    fn paths() -> ProjectPaths {
        match ProjectPaths::new(PathBuf::from("project"), PathBuf::from("canonical-project")) {
            Ok(value) => value,
            Err(error) => panic!("test paths must be non-empty: {error}"),
        }
    }

    fn input() -> ProjectInput {
        ProjectInput {
            id: project_id(),
            owner_id: owner_id(),
            display_name: "  Dev Recall  ".to_owned(),
            paths: paths(),
            project_type: ProjectType::Rust,
            description: Some("  Local developer context  ".to_owned()),
            created_at: TimestampMillis::from_unix_millis(1_000),
            updated_at: TimestampMillis::from_unix_millis(2_000),
            archived_at: None,
            privacy: ProjectPrivacySettings::default(),
        }
    }

    #[test]
    fn constructs_bounded_project_and_normalizes_optional_text() {
        let project = match Project::new(input()) {
            Ok(value) => value,
            Err(error) => panic!("valid test project must be accepted: {error}"),
        };

        assert_eq!(project.id().as_uuid_bytes(), PROJECT_UUID);
        assert_eq!(project.owner_id().as_uuid_bytes(), OWNER_UUID);
        assert_eq!(project.display_name(), "Dev Recall");
        assert_eq!(project.description(), Some("Local developer context"));
        assert_eq!(project.project_type(), ProjectType::Rust);
        assert_eq!(project.created_at().as_unix_millis(), 1_000);
        assert_eq!(project.updated_at().as_unix_millis(), 2_000);
        assert!(!project.is_archived());
    }

    #[test]
    fn rejects_nil_non_rfc_and_versionless_identifiers() {
        assert_eq!(
            ProjectId::from_uuid_bytes([0; 16]),
            Err(ProjectValidationError::InvalidProjectId)
        );

        let mut non_rfc = PROJECT_UUID;
        non_rfc[8] = 0;
        assert_eq!(
            ProjectId::from_uuid_bytes(non_rfc),
            Err(ProjectValidationError::InvalidProjectId)
        );

        let mut versionless = OWNER_UUID;
        versionless[6] = 0;
        assert_eq!(
            OwnerId::from_uuid_bytes(versionless),
            Err(ProjectValidationError::InvalidOwnerId)
        );
    }

    #[test]
    fn rejects_empty_and_oversized_display_names() {
        let mut empty = input();
        empty.display_name = "  \t  ".to_owned();
        assert!(matches!(
            Project::new(empty),
            Err(ProjectValidationError::EmptyDisplayName)
        ));

        let mut oversized = input();
        oversized.display_name = "x".repeat(MAX_PROJECT_DISPLAY_NAME_BYTES + 1);
        assert!(matches!(
            Project::new(oversized),
            Err(ProjectValidationError::DisplayNameTooLong)
        ));
    }

    #[test]
    fn applies_limits_to_utf8_bytes_not_character_count() {
        let mut oversized = input();
        oversized.display_name = "я".repeat(MAX_PROJECT_DISPLAY_NAME_BYTES / 2 + 1);
        assert!(matches!(
            Project::new(oversized),
            Err(ProjectValidationError::DisplayNameTooLong)
        ));
    }

    #[test]
    fn rejects_control_and_bidi_characters_in_text() {
        for display_name in ["hidden\0name", "spoof\u{202e}name"] {
            let mut unsafe_input = input();
            unsafe_input.display_name = display_name.to_owned();
            assert!(matches!(
                Project::new(unsafe_input),
                Err(ProjectValidationError::DisallowedTextControl)
            ));
        }

        let mut unsafe_description = input();
        unsafe_description.description = Some("line\u{2066}text".to_owned());
        assert!(matches!(
            Project::new(unsafe_description),
            Err(ProjectValidationError::DisallowedTextControl)
        ));
    }

    #[test]
    fn permits_description_layout_and_untrusted_markup_as_plain_text() {
        let mut project_input = input();
        project_input.description = Some("line one\n\t<script>test-only</script>".to_owned());
        let project = match Project::new(project_input) {
            Ok(value) => value,
            Err(error) => panic!("bounded plain text must be accepted: {error}"),
        };

        assert_eq!(
            project.description(),
            Some("line one\n\t<script>test-only</script>")
        );
    }

    #[test]
    fn normalizes_blank_description_and_rejects_oversized_description() {
        let mut blank = input();
        blank.description = Some(" \n\t ".to_owned());
        let blank = match Project::new(blank) {
            Ok(value) => value,
            Err(error) => panic!("blank optional description must normalize: {error}"),
        };
        assert_eq!(blank.description(), None);

        let mut oversized = input();
        oversized.description = Some("x".repeat(MAX_PROJECT_DESCRIPTION_BYTES + 1));
        assert!(matches!(
            Project::new(oversized),
            Err(ProjectValidationError::DescriptionTooLong)
        ));
    }

    #[test]
    fn rejects_empty_path_placeholders_without_claiming_platform_validation() {
        assert_eq!(
            ProjectPaths::new(PathBuf::new(), PathBuf::from("canonical")),
            Err(ProjectValidationError::EmptyRootPath)
        );
        assert_eq!(
            ProjectPaths::new(PathBuf::from("root"), PathBuf::new()),
            Err(ProjectValidationError::EmptyCanonicalPath)
        );
    }

    #[test]
    fn enforces_timestamp_and_archive_ordering() {
        let mut updated_before_created = input();
        updated_before_created.updated_at = TimestampMillis::from_unix_millis(999);
        assert!(matches!(
            Project::new(updated_before_created),
            Err(ProjectValidationError::UpdatedBeforeCreated)
        ));

        let mut archive_before_created = input();
        archive_before_created.archived_at = Some(TimestampMillis::from_unix_millis(999));
        assert!(matches!(
            Project::new(archive_before_created),
            Err(ProjectValidationError::ArchiveBeforeCreated)
        ));

        let mut archive_after_updated = input();
        archive_after_updated.archived_at = Some(TimestampMillis::from_unix_millis(2_001));
        assert!(matches!(
            Project::new(archive_after_updated),
            Err(ProjectValidationError::ArchiveAfterUpdated)
        ));
    }

    #[test]
    fn rejects_timestamps_outside_the_signed_persistence_range() {
        let mut out_of_range = input();
        out_of_range.updated_at = TimestampMillis::from_unix_millis(MAX_TIMESTAMP_MILLIS + 1);
        assert!(matches!(
            Project::new(out_of_range),
            Err(ProjectValidationError::TimestampOutOfRange)
        ));
    }

    #[test]
    fn privacy_settings_are_disabled_by_default_and_explicitly_enabled() {
        let defaults = ProjectPrivacySettings::default();
        assert_eq!(defaults.git_context, DataSourceConsent::Disabled);
        assert_eq!(defaults.manifest_detection, DataSourceConsent::Disabled);
        assert_eq!(
            defaults.terminal_history_association,
            DataSourceConsent::Disabled
        );

        let mut enabled = input();
        enabled.privacy.git_context = DataSourceConsent::Enabled;
        enabled.archived_at = Some(TimestampMillis::from_unix_millis(2_000));
        let enabled = match Project::new(enabled) {
            Ok(value) => value,
            Err(error) => panic!("explicit consent and valid archive time must pass: {error}"),
        };
        assert_eq!(enabled.privacy().git_context, DataSourceConsent::Enabled);
        assert!(enabled.is_archived());
    }
}
