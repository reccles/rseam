pub mod access_codes;
pub mod connect_webviews;
pub mod devices;
pub mod health;
pub mod locks;

use serde_json::Value;

/// Helper to format and print output
pub fn print_output(data: &Value, id_only: bool, raw: bool) {
    if id_only {
        // Handle arrays - extract ID from each element
        if let Some(arr) = data.as_array() {
            for item in arr {
                if let Some(id) = extract_id(item) {
                    println!("{}", id);
                }
            }
        } else {
            // Single object - extract ID
            if let Some(id) = extract_id(data) {
                println!("{}", id);
            }
        }
    } else if raw {
        println!("{}", data);
    } else {
        println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
    }
}

/// Try to extract an ID from the response
fn extract_id(data: &Value) -> Option<String> {
    // Try common ID patterns - check nested objects too
    let id_patterns = [
        "device_id",
        "access_code_id",
        "connect_webview_id",
        "action_attempt_id",
        "id",
    ];

    // First try top-level
    for pattern in &id_patterns {
        if let Some(id) = data.get(pattern).and_then(|v| v.as_str()) {
            return Some(id.to_string());
        }
    }

    // Then try common nested objects
    let nested_objects = ["device", "access_code", "connect_webview", "action_attempt"];
    for obj_name in &nested_objects {
        if let Some(obj) = data.get(obj_name) {
            for pattern in &id_patterns {
                if let Some(id) = obj.get(pattern).and_then(|v| v.as_str()) {
                    return Some(id.to_string());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_id_top_level() {
        let data = json!({"device_id": "dev_123", "name": "Test"});
        assert_eq!(extract_id(&data), Some("dev_123".to_string()));
    }

    #[test]
    fn test_extract_id_nested() {
        let data = json!({"device": {"device_id": "dev_456", "name": "Test"}});
        assert_eq!(extract_id(&data), Some("dev_456".to_string()));
    }

    #[test]
    fn test_extract_id_access_code() {
        let data = json!({"access_code": {"access_code_id": "ac_789"}});
        assert_eq!(extract_id(&data), Some("ac_789".to_string()));
    }

    #[test]
    fn test_extract_id_not_found() {
        let data = json!({"name": "Test", "value": 123});
        assert_eq!(extract_id(&data), None);
    }

    #[test]
    fn test_extract_id_prefers_top_level() {
        let data = json!({
            "device_id": "top_level_id",
            "device": {"device_id": "nested_id"}
        });
        assert_eq!(extract_id(&data), Some("top_level_id".to_string()));
    }

    #[test]
    fn test_extract_id_from_array() {
        let data = json!([
            {"device_id": "dev_1", "name": "Device 1"},
            {"device_id": "dev_2", "name": "Device 2"},
            {"device_id": "dev_3", "name": "Device 3"}
        ]);
        // Verify array can be extracted
        assert!(data.as_array().is_some());
        if let Some(arr) = data.as_array() {
            let ids: Vec<String> = arr.iter()
                .filter_map(|item| extract_id(item))
                .collect();
            assert_eq!(ids, vec!["dev_1", "dev_2", "dev_3"]);
        }
    }
}
