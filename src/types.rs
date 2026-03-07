use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct ApiResponse {
    #[serde(flatten)]
    pub data: Value,
}

/// Device type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct Device {
    pub device_id: String,
    #[serde(default)]
    pub device_name: Option<String>,
    #[serde(default)]
    pub device_type: Option<String>,
    #[serde(default)]
    pub properties: Option<Value>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Access Code type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct AccessCode {
    pub access_code_id: String,
    pub code: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Connect Webview type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct ConnectWebview {
    pub connect_webview_id: String,
    pub url: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// Action Attempt type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct ActionAttempt {
    pub action_attempt_id: String,
    pub status: String,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(flatten)]
    pub extra: Value,
}

/// List response wrapper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    #[serde(flatten)]
    pub extra: Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_device_deserialization() {
        let json = json!({
            "device_id": "dev_123",
            "device_name": "Front Door Lock",
            "device_type": "august_lock",
            "properties": {"locked": true}
        });

        let device: Device = serde_json::from_value(json).unwrap();
        assert_eq!(device.device_id, "dev_123");
        assert_eq!(device.device_name, Some("Front Door Lock".to_string()));
        assert_eq!(device.device_type, Some("august_lock".to_string()));
    }

    #[test]
    fn test_device_minimal() {
        let json = json!({
            "device_id": "dev_456"
        });

        let device: Device = serde_json::from_value(json).unwrap();
        assert_eq!(device.device_id, "dev_456");
        assert_eq!(device.device_name, None);
        assert_eq!(device.device_type, None);
    }

    #[test]
    fn test_access_code_deserialization() {
        let json = json!({
            "access_code_id": "ac_123",
            "code": "1234",
            "name": "Guest Code"
        });

        let code: AccessCode = serde_json::from_value(json).unwrap();
        assert_eq!(code.access_code_id, "ac_123");
        assert_eq!(code.code, "1234");
        assert_eq!(code.name, Some("Guest Code".to_string()));
    }

    #[test]
    fn test_connect_webview_deserialization() {
        let json = json!({
            "connect_webview_id": "cw_123",
            "url": "https://connect.getseam.com/webview/abc"
        });

        let webview: ConnectWebview = serde_json::from_value(json).unwrap();
        assert_eq!(webview.connect_webview_id, "cw_123");
        assert_eq!(webview.url, "https://connect.getseam.com/webview/abc");
    }

    #[test]
    fn test_action_attempt_deserialization() {
        let json = json!({
            "action_attempt_id": "aa_123",
            "status": "success",
            "result": {"message": "Door unlocked"}
        });

        let attempt: ActionAttempt = serde_json::from_value(json).unwrap();
        assert_eq!(attempt.action_attempt_id, "aa_123");
        assert_eq!(attempt.status, "success");
        assert!(attempt.result.is_some());
    }

    #[test]
    fn test_device_serialization_roundtrip() {
        let device = Device {
            device_id: "dev_789".to_string(),
            device_name: Some("Test Device".to_string()),
            device_type: Some("test_type".to_string()),
            properties: Some(json!({"key": "value"})),
            extra: json!({}),
        };

        let json = serde_json::to_string(&device).unwrap();
        let parsed: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(device.device_id, parsed.device_id);
        assert_eq!(device.device_name, parsed.device_name);
    }

    #[test]
    fn test_extra_fields_preserved() {
        let json = json!({
            "device_id": "dev_extra",
            "unknown_field": "should be preserved",
            "another_field": 123
        });

        let device: Device = serde_json::from_value(json).unwrap();
        assert_eq!(device.device_id, "dev_extra");
        // Extra fields should be captured in `extra`
        assert!(device.extra.get("unknown_field").is_some());
    }
}
