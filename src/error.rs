use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeamError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

pub type SeamResult<T> = Result<T, SeamError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let err = SeamError::ApiError("[401] Unauthorized".to_string());
        assert_eq!(err.to_string(), "API error: [401] Unauthorized");
    }

    #[test]
    fn test_missing_parameter_display() {
        let err = SeamError::MissingParameter("device_id".to_string());
        assert_eq!(err.to_string(), "Missing required parameter: device_id");
    }

    #[test]
    fn test_auth_error_display() {
        let err = SeamError::AuthError("SEAM_API_KEY not set".to_string());
        assert_eq!(err.to_string(), "Authentication error: SEAM_API_KEY not set");
    }

    #[test]
    fn test_serde_error_conversion() {
        let json_err: Result<serde_json::Value, _> = serde_json::from_str("not valid json");
        let seam_err: SeamError = json_err.unwrap_err().into();
        assert!(matches!(seam_err, SeamError::SerdeError(_)));
    }
}
