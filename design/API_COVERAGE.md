# Seam API Coverage Analysis

This document maps the complete Seam API to our rseam implementation.

## Entity Types (from official seam-cli)

Based on the official CLI codebase, the Seam API supports these entity types:

### Currently Implemented ✅
- **Devices** - Smart lock devices
- **Access Codes** - PIN codes and credentials
- **Locks** - Lock control operations
- **Health** - API health checks
- **Connect Webviews** - Device connection flows

### Not Yet Implemented ❌

#### ACS (Access Control System)
- `acs-systems` - ACS system management
- `acs-entrances` - Building entrances
- `acs-users` - Access control users
- `acs-access-groups` - User groups and permissions

#### Account Management
- `connected-accounts` - Third-party integrations
- `credential-pools` - Credential storage
- `workspaces` - Workspace management
- `user-identities` - User identity management

#### Events & Webhooks
- `events` - Event listing and filtering
- `webhooks` - Webhook management

#### Other
- `networks` - Network configuration
- `custom-metadata` - Custom metadata management

## Inferred Endpoint Structure

Based on CLI patterns, endpoints follow this structure:

```
/{entity}/{action}
```

Examples:
- `/devices/list`
- `/devices/get`
- `/access_codes/create`
- `/access_codes/list`
- `/access_codes/delete`
- `/locks/unlock_door`
- `/locks/lock_door`
- `/health/get_health`
- `/connect_webviews/create`

## Common Patterns

All endpoints typically:
1. Accept POST requests
2. Receive `api_key` in request body
3. Return JSON responses
4. Support optional filtering parameters

## Implementation Status

### Phase 1 - Currently Implemented ✅
- Devices: list, get
- Access Codes: create, list, delete
- Locks: unlock-door, lock-door
- Health: get-health
- Connect Webviews: create

### Phase 2 - High Priority (Not Implemented)
- Events: list, get
- Connected Accounts: list, get, disconnect
- Workspaces: list, get

### Phase 3 - Medium Priority (Not Implemented)
- ACS Systems: list, get
- ACS Entrances: list, get
- ACS Users: list, get, create, delete

### Phase 4 - Lower Priority (Not Implemented)
- ACS Access Groups
- Credential Pools
- User Identities
- Webhooks
- Networks
- Custom Metadata

## Test Coverage Plan

Tests needed for each endpoint:
1. ✅ Parameter parsing (CLI args)
2. ✅ Required parameter validation
3. ❌ API response parsing and validation
4. ❌ Output formatting (pretty, raw, id-only)
5. ❌ Error handling (API errors)
6. ❌ Integration testing

## What We Need

To complete comprehensive coverage:
1. Access full Seam API documentation
2. Extract all endpoint parameters
3. Understand required vs optional fields
4. Map response structures
5. Build typed responses for each entity
6. Create integration tests

## References

- Official Seam CLI: https://github.com/seamapi/seam-cli
- Seam API Docs: https://docs.seam.co/latest
