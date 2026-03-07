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

### devices update
Update device properties.

**Purpose:** Modify device attributes like name.

**Parameters:**
- `--device-id` (required): Device ID to update
- `--name` (optional): New name for the device

**Output:** Updated device object

**Examples:**
```bash
# Rename a device
rseam devices update --device-id "dev_123" --name "Back Door Lock"

# Update without name (no-op, useful for refresh)
rseam devices update --device-id "dev_123"
```

**Use Cases:**
- Rename devices for clarity
- Update device metadata
- Batch rename in scripts

---

### devices delete
Delete a device from the workspace.

**Purpose:** Remove a device from Seam management.

**Parameters:**
- `--device-id` (required): Device ID to delete

**Output:** Confirmation of deletion

**Examples:**
```bash
# Delete a device
rseam devices delete --device-id "dev_123"

# Find and delete by name
DEVICE_ID=$(rseam devices get --name "Old Lock" --id-only)
rseam devices delete --device-id "$DEVICE_ID"
```

**Use Cases:**
- Remove decommissioned devices
- Clean up workspace
- Reset device connections

---

## Lock Commands

### locks get
Get a specific lock's status and properties.

**Purpose:** Retrieve detailed information about a lock.

**Parameters:**
- `--device-id` (required): Lock device ID

**Output:** Lock object with status, properties, and capabilities

**Examples:**
```bash
# Get lock details
rseam locks get --device-id "dev_123"

# Get raw JSON for parsing
rseam locks get --device-id "dev_123" --raw
```

**Use Cases:**
- Check lock state (locked/unlocked)
- View lock capabilities
- Verify lock before operations

---

### locks list
List all locks in the workspace.

**Purpose:** Retrieve all lock devices with filtering.

**Parameters:**
- `--device-id` (optional): Filter by specific device ID

**Output:** Array of lock objects

**Examples:**
```bash
# List all locks
rseam locks list

# List with raw JSON
rseam locks list --raw

# Get IDs only
rseam locks list --id-only
```

**Use Cases:**
- Inventory all locks
- Find locks for bulk operations
- Monitor lock fleet

---

### locks unlock-door
Unlock a smart lock.

**Purpose:** Remotely unlock a door via the Seam API.

**Parameters:**
- `--device-id` (required): The lock device to unlock

**Output:** Action attempt with status

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

**Output:** Action attempt with status

**Examples:**
```bash
# Lock a door
rseam locks lock-door --device-id "dev_123"

# Lock all doors
for id in $(rseam locks list --id-only); do
  rseam locks lock-door --device-id "$id"
done
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

---

### access-codes get
Get a specific access code by ID.

**Purpose:** Retrieve details about a single access code.

**Parameters:**
- `--access-code-id` (required): Access code ID to retrieve

**Output:** Access code object with properties

**Examples:**
```bash
# Get access code details
rseam access-codes get --access-code-id "ac_123"

# Get raw JSON
rseam access-codes get --access-code-id "ac_123" --raw
```

**Use Cases:**
- Verify code exists
- Check code status
- Get code properties before update

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

# List all codes across all devices
rseam access-codes list

# Get IDs only
rseam access-codes list --device-id "dev_123" --id-only
```

**Use Cases:**
- Audit active codes
- Find code to delete
- Track access changes

---

### access-codes update
Update an existing access code.

**Purpose:** Modify access code properties.

**Parameters:**
- `--access-code-id` (required): Access code ID to update
- `--name` (optional): New name for the code
- `--code` (optional): New PIN/code value
- `--starts-at` (optional): ISO8601 start time for time-limited codes
- `--ends-at` (optional): ISO8601 end time for time-limited codes

**Output:** Updated access code object

**Examples:**
```bash
# Rename a code
rseam access-codes update --access-code-id "ac_123" --name "VIP Guest"

# Change the PIN
rseam access-codes update --access-code-id "ac_123" --code "9999"

# Set time-limited access
rseam access-codes update --access-code-id "ac_123" \
  --starts-at "2024-01-01T09:00:00Z" \
  --ends-at "2024-01-01T17:00:00Z"
```

**Use Cases:**
- Update code names
- Change PINs
- Convert to time-limited access
- Extend/modify access windows

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
rseam access-codes delete --access-code-id "ac_123"

# Delete all codes for a device
for id in $(rseam access-codes list --device-id "dev_123" --id-only); do
  rseam access-codes delete --access-code-id "$id"
done
```

**Use Cases:**
- Revoke guest access
- Remove expired codes
- Deactivate contractor access

---

### access-codes generate-code
Generate a new access code automatically.

**Purpose:** Let Seam auto-generate a unique PIN for a device.

**Parameters:**
- `--device-id` (required): The lock to add code to
- `--name` (optional): Human-readable name for this code

**Output:** Created access code object with auto-generated PIN

**Examples:**
```bash
# Generate code with auto PIN
rseam access-codes generate-code --device-id "dev_123" --name "Auto Guest"

# Generate and capture the code value
CODE=$(rseam access-codes generate-code \
  --device-id "dev_123" \
  --name "Temp Access" \
  --raw | jq -r '.code')
echo "Generated code: $CODE"
```

**Use Cases:**
- Auto-generate unique PINs
- Avoid PIN conflicts
- Temporary access without choosing codes

---

### access-codes create-multiple
Create multiple access codes at once.

**Purpose:** Batch create access codes for a device.

**Parameters:**
- `--device-id` (required): The lock to add codes to
- `--codes-json` (required): JSON array of code specs

**Output:** Array of created access code objects

**Examples:**
```bash
# Create multiple guest codes
rseam access-codes create-multiple \
  --device-id "dev_123" \
  --codes-json '[{"code":"1234","name":"Guest 1"},{"code":"5678","name":"Guest 2"}]'

# Create codes from a file
rseam access-codes create-multiple \
  --device-id "dev_123" \
  --codes-json "$(cat codes.json)"
```

**Use Cases:**
- Bulk onboarding of access codes
- Setting up multiple guest codes at once
- Scripted provisioning

---

### access-codes update-multiple
Update multiple access codes at once.

**Purpose:** Batch update access codes.

**Parameters:**
- `--updates-json` (required): JSON array of update specs (must include access_code_id)

**Output:** Array of updated access code objects

**Examples:**
```bash
# Update multiple codes
rseam access-codes update-multiple \
  --updates-json '[{"access_code_id":"ac_123","name":"VIP 1"},{"access_code_id":"ac_456","name":"VIP 2"}]'

# Extend multiple codes' end times
rseam access-codes update-multiple \
  --updates-json '[{"access_code_id":"ac_123","ends_at":"2024-12-31T23:59:59Z"},{"access_code_id":"ac_456","ends_at":"2024-12-31T23:59:59Z"}]'
```

**Use Cases:**
- Bulk rename access codes
- Extend expiration for multiple codes
- Batch update time windows

---

### access-codes get-timeline
Get the change history for an access code.

**Purpose:** View when an access code was created, modified, or used.

**Parameters:**
- `--access-code-id` (required): The access code to get history for

**Output:** Timeline of events for the access code

**Examples:**
```bash
# Get timeline for a code
rseam access-codes get-timeline --access-code-id "ac_123"

# Get raw JSON for parsing
rseam access-codes get-timeline --access-code-id "ac_123" --raw
```

**Use Cases:**
- Audit access code changes
- Debug code sync issues
- Track when codes were used

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
# Lock all doors
for id in $(rseam locks list --id-only); do
  rseam locks lock-door --device-id "$id"
done

# Create multiple codes
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

## Command Summary

| Command | Purpose |
|---------|---------|
| `devices list` | List all devices |
| `devices get` | Get device by ID or name |
| `devices update` | Update device properties |
| `devices delete` | Delete a device |
| `locks get` | Get lock status |
| `locks list` | List all locks |
| `locks unlock-door` | Unlock a door |
| `locks lock-door` | Lock a door |
| `access-codes create` | Create access code with specific PIN |
| `access-codes get` | Get access code details |
| `access-codes list` | List access codes |
| `access-codes update` | Update access code |
| `access-codes delete` | Delete access code |
| `access-codes generate-code` | Auto-generate access code |
| `access-codes create-multiple` | Batch create access codes |
| `access-codes update-multiple` | Batch update access codes |
| `health get-health` | Check API health |
| `connect-webviews create` | Create device pairing webview |

---

## Tips for Agents

1. **Always check health first** if you're unsure of connectivity
2. **Use --raw for JSON parsing** when building responses
3. **Use --id-only for piping** between commands
4. **List first, then get** if you need to find things by name
5. **Check parameter requirements** - some flags are mandatory, others optional
6. **Use generate-code** when you don't need a specific PIN

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
        assert!(context.contains("devices update"));
        assert!(context.contains("devices delete"));
        assert!(context.contains("locks get"));
        assert!(context.contains("locks list"));
        assert!(context.contains("locks unlock-door"));
        assert!(context.contains("access-codes create"));
        assert!(context.contains("access-codes get"));
        assert!(context.contains("access-codes update"));
        assert!(context.contains("access-codes generate-code"));
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

    #[test]
    fn test_agent_context_has_command_summary() {
        let context = generate_agent_context();
        assert!(context.contains("Command Summary"));
    }
}
