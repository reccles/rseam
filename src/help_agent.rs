/// Generate self-documenting markdown for agents
/// Output by `rseam --help-agent`
pub fn generate_agent_context() -> String {
    r#"# rseam CLI Reference

A Rust-based Seam API CLI for non-interactive scripting and automation.

## Global Flags

- `--id-only`: Output only the ID field (useful for scripting)
- `--raw`: Output raw JSON (default: pretty-printed)
- `--help`: Display help information
- `--help-agent`: Display this comprehensive agent context (markdown format)

---

## Device Commands

### devices list
List all devices in the workspace.

**Purpose:** Retrieve all connected devices.

**Parameters:**
- `--device-id` (optional): Filter by specific device ID
- `--name` (optional): Filter by device name

**Output:** Array of device objects with ID, name, device_type, properties, etc.

**Examples:**
```bash
# List all devices
rseam devices list

# Get ID only (useful in scripts)
rseam devices list --id-only

# Raw JSON output
rseam devices list --raw

# Get ID of first result for piping
DEVICE_ID=$(rseam devices list --id-only)
```

**Use Cases:**
- Inventory devices in workspace
- Find device IDs for other operations
- Monitor connected devices

---

### devices get
Get a specific device by ID or name.

**Purpose:** Retrieve detailed information about a single device.

**Parameters:**
- `--device-id` (optional): Device ID to retrieve
- `--name` (optional): Device name to retrieve

**Output:** Single device object

**Examples:**
```bash
# Get device by ID
rseam devices get --device-id "dev_123"

# Get device by name
rseam devices get --name "Front Door Lock"

# Get and extract ID
rseam devices get --device-id "dev_123" --id-only
```

**Use Cases:**
- Verify device exists
- Get device properties before making changes
- Find device state/status

---

## Lock Commands

### locks unlock-door
Unlock a smart lock.

**Purpose:** Remotely unlock a door via the Seam API.

**Parameters:**
- `--device-id` (required): The lock device to unlock

**Output:** Lock state confirmation

**Examples:**
```bash
# Unlock a door
rseam locks unlock-door --device-id "dev_123"

# Get device ID first, then unlock
LOCK_ID=$(rseam devices get --name "Front Door" --id-only)
rseam locks unlock-door --device-id "$LOCK_ID"
```

**Use Cases:**
- Emergency unlock
- Temporary access grants
- Automated unlock workflows

---

### locks lock-door
Lock a smart lock.

**Purpose:** Remotely lock a door via the Seam API.

**Parameters:**
- `--device-id` (required): The lock device to lock

**Output:** Lock state confirmation

**Examples:**
```bash
# Lock a door
rseam locks lock-door --device-id "dev_123"

# Verify then lock
rseam devices get --device-id "dev_123"
rseam locks lock-door --device-id "dev_123"
```

**Use Cases:**
- Secure building after hours
- Emergency lockdown
- Verify lock state

---

## Access Code Commands

### access-codes create
Create a new access code for a device.

**Purpose:** Generate a new PIN/code for temporary or permanent access.

**Parameters:**
- `--device-id` (required): The lock to add code to
- `--code` (required): The PIN/code value
- `--name` (optional): Human-readable name for this code

**Output:** Created access code object with ID and metadata

**Examples:**
```bash
# Create guest code
rseam access-codes create \
  --device-id "dev_123" \
  --code "1234" \
  --name "Guest Code"

# Create code and capture ID
CODE_ID=$(rseam access-codes create \
  --device-id "dev_123" \
  --code "5678" \
  --name "Delivery" \
  --id-only)
```

**Use Cases:**
- Add guest access
- Create temporary contractor codes
- Manage access for specific users
- Multi-code per lock scenarios

---

### access-codes list
List all access codes for a device.

**Purpose:** Show all codes that work on a specific lock.

**Parameters:**
- `--device-id` (optional): Filter codes for specific device

**Output:** Array of access code objects

**Examples:**
```bash
# List all codes for a device
rseam access-codes list --device-id "dev_123"

# List and get raw JSON
rseam access-codes list --device-id "dev_123" --raw

# List all codes across all devices
rseam access-codes list
```

**Use Cases:**
- Audit active codes
- Find code to delete
- Track access changes

---

### access-codes delete
Delete an access code.

**Purpose:** Revoke access by removing a code.

**Parameters:**
- `--access-code-id` (required): The code to delete

**Output:** Confirmation of deletion

**Examples:**
```bash
# Delete a code
rseam access-codes delete --access-code-id "code_456"

# Get code ID then delete
CODE_ID=$(rseam access-codes list --device-id "dev_123" --id-only)
rseam access-codes delete --access-code-id "$CODE_ID"
```

**Use Cases:**
- Revoke guest access
- Remove expired codes
- Deactivate contractor access

---

## Health Commands

### health get-health
Check API health status.

**Purpose:** Verify Seam API is reachable and working.

**Parameters:** None

**Output:** Health status object

**Examples:**
```bash
# Check API health
rseam health get-health

# Use in automation to gate other commands
if rseam health get-health; then
  echo "API is healthy"
fi
```

**Use Cases:**
- Verify connectivity before automation
- Health check in monitoring
- Diagnose API issues

---

## Connect Webview Commands

### connect-webviews create
Create a connect webview for device pairing.

**Purpose:** Generate a webview URL for users to add devices to workspace.

**Parameters:**
- `--accepted-providers` (optional): Comma-separated list of allowed providers (e.g., "august,level")

**Output:** Webview object with URL and configuration

**Examples:**
```bash
# Create unrestricted webview
rseam connect-webviews create

# Create webview for specific providers
rseam connect-webviews create --accepted-providers "august,level"

# Get webview URL
URL=$(rseam connect-webviews create --raw | jq -r '.url')
echo "Send this to user: $URL"
```

**Use Cases:**
- Onboard new users
- Add specific device types only
- Self-service device pairing

---

## Common Patterns

### Piping IDs Between Commands
```bash
# Get device, then unlock
DEVICE_ID=$(rseam devices get --name "Front Door" --id-only)
rseam locks unlock-door --device-id "$DEVICE_ID"
```

### Bulk Operations
```bash
# Create multiple codes for same lock
for code in 1111 2222 3333; do
  rseam access-codes create --device-id "dev_123" --code "$code"
done
```

### Error Checking
```bash
# Check API before running
rseam health get-health || exit 1

# Now proceed
rseam devices list
```

### JSON Processing
```bash
# List devices and extract names
rseam devices list --raw | jq '.[] | .name'

# Get specific device properties
rseam devices get --device-id "dev_123" --raw | jq '.properties'
```

---

## Output Formats

### Pretty JSON (Default)
```bash
rseam devices list
```
Human-readable, indented output.

### Raw JSON
```bash
rseam devices list --raw
```
Compact single-line JSON, good for piping.

### ID Only
```bash
rseam devices list --id-only
```
Just the ID field, useful for scripting and chaining commands.

---

## Authentication

Set your API key via environment variable:
```bash
export SEAM_API_KEY="your_seam_api_key"
```

All commands use this key automatically.

---

## Tips for Agents

1. **Always check health first** if you're unsure of connectivity
2. **Use --raw for JSON parsing** when building responses
3. **Use --id-only for piping** between commands
4. **List first, then get** if you need to find things by name
5. **Check parameter requirements** - some flags are mandatory, others optional

---

Generated for agent consumption. Full API documentation: https://docs.seam.co
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_context_generates() {
        let context = generate_agent_context();
        assert!(context.contains("devices list"));
        assert!(context.contains("locks unlock-door"));
        assert!(context.contains("access-codes create"));
        assert!(context.contains("--help-agent"));
    }

    #[test]
    fn test_agent_context_has_examples() {
        let context = generate_agent_context();
        assert!(context.contains("Examples:"));
        assert!(context.contains("```bash"));
    }

    #[test]
    fn test_agent_context_has_use_cases() {
        let context = generate_agent_context();
        assert!(context.contains("Use Cases:"));
    }
}
