# Seam API Endpoint Test Matrix

This file tracks which endpoints we've tested and which need coverage.

## Format

```
Entity / Endpoint Status | Params Validated | API Works | Output Formats | Test File
```

## DEVICES (Smart Locks)

| Endpoint | CLI Args ✅ | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|-----------|----------|------------|----------|---------|-----------|
| devices list | ✅ | ❌ | ❌ | ❌ | ❌ | src/commands/devices.rs |
| devices get | ✅ | ❌ | ❌ | ❌ | ❌ | src/commands/devices.rs |
| devices delete | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| devices update | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## ACCESS CODES (PIN/Credential Management)

| Endpoint | CLI Args ✅ | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|-----------|----------|------------|----------|---------|-----------|
| access-codes create | ✅ | ❌ | ❌ | ❌ | ❌ | src/commands/access_codes.rs |
| access-codes list | ✅ | ❌ | ❌ | ❌ | ❌ | src/commands/access_codes.rs |
| access-codes delete | ✅ | ❌ | ❌ | ❌ | ❌ | src/commands/access_codes.rs |
| access-codes get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| access-codes update | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## LOCKS (Door/Lock Control)

| Endpoint | CLI Args ✅ | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|-----------|----------|------------|----------|---------|-----------|
| locks unlock-door | ✅ | ❌ | ❌ | ❌ | ❌ | src/commands/locks.rs |
| locks lock-door | ✅ | ❌ | ❌ | ❌ | ❌ | src/commands/locks.rs |
| locks get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## CONNECT WEBVIEWS (Device Pairing)

| Endpoint | CLI Args ✅ | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|-----------|----------|------------|----------|---------|-----------|
| connect-webviews create | ✅ | ❌ | ❌ | ❌ | ❌ | src/commands/connect_webviews.rs |
| connect-webviews get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| connect-webviews list | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## HEALTH (API Status)

| Endpoint | CLI Args ✅ | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|-----------|----------|------------|----------|---------|-----------|
| health get-health | ✅ | ✅ | ✅ | ❌ | ❌ | src/commands/health.rs |

## EVENTS (Event History)

| Endpoint | CLI Args | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|---------|----------|------------|----------|---------|-----------|
| events list | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| events get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## CONNECTED ACCOUNTS (Integrations)

| Endpoint | CLI Args | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|---------|----------|------------|----------|---------|-----------|
| connected-accounts list | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| connected-accounts get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| connected-accounts disconnect | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## WORKSPACES (Team/Org Management)

| Endpoint | CLI Args | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|---------|----------|------------|----------|---------|-----------|
| workspaces list | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| workspaces get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## ACS SYSTEMS (Access Control Systems)

| Endpoint | CLI Args | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|---------|----------|------------|----------|---------|-----------|
| acs-systems list | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| acs-systems get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## ACS ENTRANCES (Building Entrances)

| Endpoint | CLI Args | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|---------|----------|------------|----------|---------|-----------|
| acs-entrances list | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| acs-entrances get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## ACS USERS (Access Control Users)

| Endpoint | CLI Args | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|---------|----------|------------|----------|---------|-----------|
| acs-users list | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| acs-users get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| acs-users create | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| acs-users delete | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## ACS ACCESS GROUPS (Permissions)

| Endpoint | CLI Args | API Test | Pretty JSON | Raw JSON | ID-Only | Test File |
|----------|---------|----------|------------|----------|---------|-----------|
| acs-access-groups list | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |
| acs-access-groups get | ❌ | ❌ | ❌ | ❌ | ❌ | NOT IMPLEMENTED |

## SUMMARY

### Fully Tested ✅
- health/get-health

### Partially Implemented (CLI parsing works, needs API tests)
- devices/list
- devices/get
- access-codes/create
- access-codes/list
- access-codes/delete
- locks/unlock-door
- locks/lock-door
- connect-webviews/create

### Not Implemented (0%)
- 20+ other endpoints

## Legend

- ✅ = Implemented/Tested
- ❌ = Not implemented
- Blank = Needs assessment

## Next Steps

1. **Immediate**: API test coverage for implemented endpoints
2. **Short-term**: Add missing variants (get, update) for existing entities
3. **Medium-term**: Events, Connected Accounts, Workspaces
4. **Long-term**: ACS systems and other advanced features
