use crate::error::{SeamError, SeamResult};
use reqwest::Client;
use serde_json::Value;
use std::env;

const DEFAULT_API_ENDPOINT: &str = "https://connect.getseam.com";

pub struct SeamClient {
    client: Client,
    api_key: String,
    endpoint: String,
}

impl SeamClient {
    /// Create a new SEAM API client from environment variable
    pub fn from_env() -> SeamResult<Self> {
        let api_key = env::var("SEAM_API_KEY").map_err(|_| {
            SeamError::AuthError("SEAM_API_KEY environment variable not set".to_string())
        })?;

        let endpoint =
            env::var("SEAM_API_ENDPOINT").unwrap_or_else(|_| DEFAULT_API_ENDPOINT.to_string());

        Ok(Self {
            client: Client::new(),
            api_key,
            endpoint,
        })
    }

    /// Create a new SEAM API client with explicit configuration (for testing)
    #[cfg(test)]
    pub fn new(api_key: String, endpoint: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            endpoint,
        }
    }

    /// Make a POST request to the SEAM API
    pub async fn post(&self, path: &str, params: Value) -> SeamResult<Value> {
        let url = format!("{}{}", self.endpoint, path);

        let body = match params {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(serde_json::Map::new()),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            serde_json::from_str(&text).map_err(SeamError::SerdeError)
        } else {
            // Try to parse error JSON for user-friendly message
            let error_msg = parse_error_response(&text)
                .unwrap_or_else(|| text.clone());
            Err(SeamError::ApiError(format!("[{}] {}", status.as_u16(), error_msg)))
        }
    }

    /// Get the endpoint URL (used for debugging/testing)
    #[cfg(test)]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get the API key (masked for display)
    #[cfg(test)]
    pub fn api_key_masked(&self) -> String {
        if self.api_key.len() > 10 {
            format!("{}...", &self.api_key[..10])
        } else {
            "***".to_string()
        }
    }
}

/// Parse Seam API error response and extract user-friendly message
fn parse_error_response(text: &str) -> Option<String> {
    // Try to parse as JSON
    let json: serde_json::Value = serde_json::from_str(text).ok()?;
    
    // Extract error.type and error.message
    let error_type = json.get("error")
        .and_then(|e| e.get("type"))
        .and_then(|t| t.as_str())
        .unwrap_or("unknown_error");
    
    let error_message = json.get("error")
        .and_then(|e| e.get("message"))
        .and_then(|m| m.as_str())
        .unwrap_or("No error message provided");
    
    Some(format!("{} - {}", error_type, error_message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = SeamClient::new(
            "test_api_key".to_string(),
            "https://test.example.com".to_string(),
        );
        assert_eq!(client.endpoint(), "https://test.example.com");
        assert_eq!(client.api_key_masked(), "test_api_k...");
    }

    #[test]
    fn test_client_from_env_missing_key() {
        // Temporarily unset the env var if it exists
        let original = env::var("SEAM_API_KEY").ok();
        env::remove_var("SEAM_API_KEY");

        let result = SeamClient::from_env();
        assert!(result.is_err());

        if let Err(SeamError::AuthError(msg)) = result {
            assert!(msg.contains("SEAM_API_KEY"));
        } else {
            panic!("Expected AuthError");
        }

        // Restore original value
        if let Some(val) = original {
            env::set_var("SEAM_API_KEY", val);
        }
    }

    #[test]
    fn test_default_endpoint() {
        assert_eq!(DEFAULT_API_ENDPOINT, "https://connect.getseam.com");
    }
}
