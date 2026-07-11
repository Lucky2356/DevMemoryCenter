use serde::{Deserialize, Serialize};

const HEALTH_API_VERSION: u16 = 1;
const MAX_CORRELATION_ID_BYTES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationErrorCode {
    InvalidRequest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationErrorMessageKey {
    InvalidRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationError {
    code: ApplicationErrorCode,
    message_key: ApplicationErrorMessageKey,
    retryable: bool,
}

impl ApplicationError {
    fn invalid_request() -> Self {
        Self {
            code: ApplicationErrorCode::InvalidRequest,
            message_key: ApplicationErrorMessageKey::InvalidRequest,
            retryable: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HealthRequest {
    correlation_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Ready,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    status: HealthStatus,
    api_version: u16,
    correlation_id: String,
}

impl HealthRequest {
    fn validate(self) -> Result<Self, ApplicationError> {
        let correlation_id = self.correlation_id.as_bytes();
        let is_valid_length =
            !correlation_id.is_empty() && correlation_id.len() <= MAX_CORRELATION_ID_BYTES;
        let has_valid_characters = correlation_id
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'));

        if is_valid_length && has_valid_characters {
            Ok(self)
        } else {
            Err(ApplicationError::invalid_request())
        }
    }
}

#[tauri::command]
pub fn get_application_health(request: HealthRequest) -> Result<HealthResponse, ApplicationError> {
    let request = request.validate()?;

    Ok(HealthResponse {
        status: HealthStatus::Ready,
        api_version: HEALTH_API_VERSION,
        correlation_id: request.correlation_id,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        ApplicationError, HealthRequest, HealthResponse, HealthStatus, MAX_CORRELATION_ID_BYTES,
        get_application_health,
    };

    fn request(correlation_id: impl Into<String>) -> HealthRequest {
        HealthRequest {
            correlation_id: correlation_id.into(),
        }
    }

    #[test]
    fn returns_only_fixed_health_metadata_and_the_validated_correlation_id() {
        let result = get_application_health(request("health-check_01"));

        assert_eq!(
            result,
            Ok(HealthResponse {
                status: HealthStatus::Ready,
                api_version: 1,
                correlation_id: "health-check_01".to_owned(),
            })
        );
    }

    #[test]
    fn accepts_the_maximum_bounded_correlation_id() {
        let correlation_id = "a".repeat(MAX_CORRELATION_ID_BYTES);

        assert!(get_application_health(request(correlation_id)).is_ok());
    }

    #[test]
    fn rejects_empty_oversized_or_unsafe_correlation_ids() {
        let oversized = "a".repeat(MAX_CORRELATION_ID_BYTES + 1);
        let invalid_values = [
            "",
            oversized.as_str(),
            "contains space",
            "control\n",
            "кириллица",
        ];

        for value in invalid_values {
            assert_eq!(
                get_application_health(request(value)),
                Err(ApplicationError::invalid_request())
            );
        }
    }

    #[test]
    fn rejects_unknown_request_fields() {
        let encoded = r#"{"correlationId":"health-1","unexpected":"value"}"#;

        assert!(serde_json::from_str::<HealthRequest>(encoded).is_err());
    }

    #[test]
    fn serialized_errors_are_fixed_and_do_not_echo_rejected_input() {
        let rejected_value = "not safe";
        let error = get_application_health(request(rejected_value))
            .expect_err("the unsafe test value must be rejected");
        let serialized =
            serde_json::to_string(&error).expect("the fixed application error must serialize");

        assert_eq!(
            serialized,
            r#"{"code":"invalid_request","messageKey":"invalid_request","retryable":false}"#
        );
        assert!(!serialized.contains(rejected_value));
    }
}
