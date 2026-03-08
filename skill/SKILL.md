---
name: rseam
description: Use when the user asks about smart locks, door locks, access codes, or PIN management. Invoke when the user wants to lock/unlock doors remotely, create or manage guest codes, set up temporary access, or automate lock operations via CLI. Also use when the user mentions Seam, Seam API, or smart home access control. Do NOT use for general home automation (lights, thermostats), physical key management, or lock hardware installation.
---

# rseam - Seam Smart Lock CLI

## Table of Contents

- [Read This First](#read-this-first)
- [Quick Reference](#quick-reference)
- [Core Workflows](#core-workflows)
- [Command Reference](#command-reference)
- [Tips for Agents](#tips-for-agents)

---

## Read This First

### What This Skill Does

This skill enables CLI-based control of smart locks via the Seam API. Use it to lock/unlock doors, manage access codes (PINs), and automate access control workflows.

### Prerequisites

```bash
# API key must be set
export SEAM_API_KEY="your_seam_api_key_here"
```

The CLI reads `SEAM_API_KEY` from environment. All commands fail without it.

---

## Quick Reference

| Command | Purpose |
|---------|---------|
| `rseam devices list` | List all devices |
| `rseam devices get --name "X"` | Get device by name |
| `rseam locks list` | List all locks |
| `rseam locks unlock-door --device-id ID` | Unlock a door |
| `rseam locks lock-door --device-id ID` | Lock a door |
| `rseam access-codes create --device-id ID --code PIN` | Create access code |
| `rseam access-codes list --device-id ID` | List codes on a lock (device-id required) |
| `rseam access-codes delete --access-code-id ID` | Revoke access code |
| `rseam health get-health` | Check API connectivity |

**Output flags:** `--raw` (compact JSON), `--id-only` (IDs only, for scripting)

---

## Core Workflows

### Find and Unlock a Door

```bash
# Get device ID by name
DEVICE_ID=$(rseam devices get --name "Front Door" --id-only)

# Unlock it
rseam locks unlock-door --device-id "$DEVICE_ID"
```

### Create a Guest Access Code

```bash
# Find the lock
LOCK_ID=$(rseam devices get --name "Main Entrance" --id-only)

# Create code with specific PIN
rseam access-codes create --device-id "$LOCK_ID" --code "1234" --name "Weekend Guest"

# Or let Seam auto-generate a PIN
rseam access-codes generate-code --device-id "$LOCK_ID" --name "Auto Guest"
```

### Create Time-Limited Access

Time-limited codes are inactive outside their time window and automatically deactivate. 
Times use ISO8601 format with timezone offset (e.g., `-08:00` for PST, `Z` for UTC).

```bash
# Create code with time window (PST business hours)
rseam access-codes create \
  --device-id "$LOCK_ID" \
  --code "5678" \
  --name "Contractor" \
  --starts-at "2024-01-15T09:00:00-08:00" \
  --ends-at "2024-01-15T17:00:00-08:00"

# Or use UTC (Z)
rseam access-codes create \
  --device-id "$LOCK_ID" \
  --code "5678" \
  --name "Contractor" \
  --starts-at "2024-01-15T09:00:00Z" \
  --ends-at "2024-01-15T17:00:00Z"

# Or create then add timing
CODE_ID=$(rseam access-codes create \
  --device-id "$LOCK_ID" \
  --code "5678" \
  --name "Contractor" \
  --id-only)

rseam access-codes update --access-code-id "$CODE_ID" \
  --starts-at "2024-01-15T09:00:00Z" \
  --ends-at "2024-01-15T17:00:00Z"
```

**Use Cases:**
- Business-hours-only contractor access
- Single-day event guest codes
- Time-window deliveries
- Shift-based employee access
- Temporary weekend access
```

### Revoke All Access Codes

```bash
for code_id in $(rseam access-codes list --device-id "$LOCK_ID" --id-only); do
  rseam access-codes delete --access-code-id "$code_id"
done
```

### Lock All Doors

```bash
for lock_id in $(rseam locks list --id-only); do
  rseam locks lock-door --device-id "$lock_id"
done
```

### Check API Health First

```bash
rseam health get-health || { echo "API unreachable"; exit 1; }
```

---

## Command Reference

### Device Commands

### `devices list`
List all devices. Optional filters: `--device-id`, `--name`

### `devices get`
Get single device. Params: `--device-id` or `--name`

### `devices update`
Update device. Params: `--device-id` (required), `--name` (optional)

### `devices delete`
Remove device. Params: `--device-id` (required)

---

### Lock Commands

### `locks list`
List all locks. Optional: `--device-id` filter

### `locks get`
Get lock details. Params: `--device-id` (required)

### `locks unlock-door`
Unlock a door. Params: `--device-id` (required)

### `locks lock-door`
Lock a door. Params: `--device-id` (required)

---

### Access Code Commands

### `access-codes create`
Create code with specific PIN.
- `--device-id` (required)
- `--code` (required) - the PIN
- `--name` (optional)

### `access-codes generate-code`
Auto-generate a unique PIN.
- `--device-id` (required)
- `--name` (optional)

### `access-codes list`
List codes. Params: `--device-id` (required)

### `access-codes get`
Get code details. Params: `--access-code-id` (required)

### `access-codes update`
Modify existing code.
- `--access-code-id` (required)
- `--name` (optional)
- `--code` (optional) - change PIN
- `--starts-at` (optional) - ISO8601 timestamp
- `--ends-at` (optional) - ISO8601 timestamp

### `access-codes delete`
Revoke code. Params: `--access-code-id` (required)

---

### Connect Webview Commands

### `connect-webviews create`
Create device pairing URL for end users.
- `--accepted-providers` (optional) - comma-separated, e.g., "august,level"

```bash
# Get pairing URL
URL=$(rseam connect-webviews create --raw | jq -r '.url')
echo "Send this to user: $URL"
```

---

### JSON Processing Tips

```bash
# Extract specific fields
rseam devices list --raw | jq '.[].name'

# Get device properties
rseam devices get --device-id "dev_123" --raw | jq '.properties'

# Get generated code value
rseam access-codes generate-code --device-id "$ID" --raw | jq -r '.code'
```

---

## Tips for Agents

1. **Always use `--id-only`** when chaining commands
2. **Use `--raw`** when parsing JSON output
3. **Check health first** if connectivity is uncertain
4. **List before get** to find things by name
5. **Use `generate-code`** when the specific PIN doesn't matter
6. **ISO8601 format** for time-limited codes: `2024-01-15T09:00:00Z`

---

