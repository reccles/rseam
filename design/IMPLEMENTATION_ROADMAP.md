# rseam Implementation Roadmap

## Current Status: MVP v0.1.0

**Implemented: 9 endpoints (4.1% coverage)**
- ✅ access_codes: create, delete, list
- ✅ connect_webviews: create
- ✅ devices: get, list
- ✅ health: get-health
- ✅ locks: lock_door, unlock_door

**Tests: 30 unit tests (100% passing)**

## Phase 1: Complete Basic CRUD (Target: v0.2.0)

### Goal: Full CRUD for most common operations

#### 1. Devices (4 new endpoints)
- [ ] `devices/get` - GET - Get single device
- [ ] `devices/delete` - POST - Delete device
- [ ] `devices/update` - POST - Update device properties
- [ ] `devices/list` - Already done ✅

**Complexity:** Low
**Priority:** HIGH - Most-used resource

#### 2. Locks (3 new endpoints)
- [ ] `locks/get` - GET - Get lock details
- [ ] `locks/list` - GET - List all locks
- [ ] `locks/lock_door` - Already done ✅
- [ ] `locks/unlock_door` - Already done ✅

**Complexity:** Low
**Priority:** HIGH - Core feature

#### 3. Access Codes (3 new endpoints)
- [ ] `access_codes/get` - GET - Get single code
- [ ] `access_codes/update` - POST - Update code (time bounds, code value, etc)
- [ ] Rest already done ✅

**Complexity:** Medium (complex update parameters)
**Priority:** HIGH

#### 4. Connect Webviews (2 new endpoints)
- [ ] `connect_webviews/get` - GET - Get webview status
- [ ] `connect_webviews/list` - GET - List webviews
- [ ] `connect_webviews/delete` - POST - Delete webview
- [ ] `connect_webviews/create` - Already done ✅

**Complexity:** Low
**Priority:** MEDIUM

### Effort Estimate: 3-4 days

---

## Phase 2: Account & Event Management (Target: v0.3.0)

### Goal: Support account management and event monitoring

#### 1. Connected Accounts (3 endpoints)
- [ ] `connected_accounts/list` - GET - List integrations
- [ ] `connected_accounts/get` - GET - Get integration details
- [ ] `connected_accounts/delete` - DELETE - Disconnect integration
- [ ] `connected_accounts/update` - POST - Update settings
- [ ] `connected_accounts/sync` - POST - Force sync

**Complexity:** Low-Medium
**Priority:** MEDIUM - Needed for multi-account setups

#### 2. Events (2 endpoints)
- [ ] `events/list` - GET - List events with filtering
- [ ] `events/get` - GET - Get event details

**Complexity:** Medium (filtering, pagination)
**Priority:** MEDIUM - Event sourcing use cases

#### 3. Workspaces (2 endpoints)
- [ ] `workspaces/list` - GET
- [ ] `workspaces/get` - GET
- [ ] `workspaces/update` - POST

**Complexity:** Low
**Priority:** LOWER - Admin feature

### Effort Estimate: 2-3 days

---

## Phase 3: User & Session Management (Target: v0.4.0)

### Goal: Support user identities and client sessions

#### 1. Client Sessions (5 endpoints)
- [ ] `client_sessions/create` - POST
- [ ] `client_sessions/list` - GET
- [ ] `client_sessions/get` - GET
- [ ] `client_sessions/grant_access` - POST
- [ ] `client_sessions/revoke` - POST

**Complexity:** Medium
**Priority:** MEDIUM - SDK/mobile support

#### 2. User Identities (6 endpoints - MVP subset)
- [ ] `user_identities/list` - GET
- [ ] `user_identities/get` - GET
- [ ] `user_identities/create` - POST
- [ ] `user_identities/update` - POST
- [ ] `user_identities/grant_access_to_device` - POST
- [ ] `user_identities/revoke_access_to_device` - DELETE

**Complexity:** Medium-High
**Priority:** MEDIUM

### Effort Estimate: 3-4 days

---

## Phase 4: ACS Systems (Target: v0.5.0)

### Goal: Full Access Control System support

#### 1. ACS Systems (2 endpoints)
- [ ] `acs/systems/list` - GET
- [ ] `acs/systems/get` - GET

#### 2. ACS Entrances (4 endpoints)
- [ ] `acs/entrances/list` - GET
- [ ] `acs/entrances/get` - GET
- [ ] `acs/entrances/grant_access` - POST
- [ ] `acs/entrances/list_credentials_with_access` - GET

#### 3. ACS Users (8 endpoints)
- [ ] `acs/users/list` - GET
- [ ] `acs/users/get` - GET
- [ ] `acs/users/create` - POST
- [ ] `acs/users/update` - POST
- [ ] `acs/users/delete` - DELETE
- [ ] `acs/users/add_to_access_group` - POST
- [ ] `acs/users/remove_from_access_group` - POST
- [ ] `acs/users/suspend` - POST
- [ ] `acs/users/unsuspend` - POST

#### 4. ACS Access Groups (5 endpoints)
- [ ] `acs/access_groups/list` - GET
- [ ] `acs/access_groups/get` - GET
- [ ] `acs/access_groups/add_user` - POST
- [ ] `acs/access_groups/remove_user` - DELETE
- [ ] `acs/access_groups/list_users` - GET

**Complexity:** High (interdependencies)
**Priority:** LOWER - Enterprise features

### Effort Estimate: 4-5 days

---

## Phase 5: Advanced Features (v0.6.0+)

### Lower Priority Features

#### Thermostats (25 endpoints)
- Climate presets
- Schedules
- Temperature control
- HVAC modes

#### Noise Sensors (7 endpoints)
- Noise monitoring
- Thresholds

#### Phones (4 endpoints)
- Phone sessions

#### Webhooks (5 endpoints)
- Event subscriptions

#### Spaces (10 endpoints)
- Area/room management

#### Access Grants (10 endpoints)
- Advanced access control

#### Instant Keys (3 endpoints)
- Temporary access URLs

---

## Implementation Strategy

### For Each Endpoint

1. **Extract from OpenAPI schema** ✅
   - Parameter types and requirements
   - Response structure
   - Documentation

2. **Add CLI command** 
   - New variant in clap enum
   - Argument parsing
   - Help text

3. **Create types.rs structs**
   - Request parameters
   - Response objects
   - Serialization/deserialization

4. **Implement command handler**
   - Call API client
   - Format output (pretty/raw/id-only)
   - Error handling

5. **Write tests**
   - CLI parsing
   - Parameter validation
   - Mock API responses

6. **Integration test**
   - Live API call
   - Verify behavior

### Testing Matrix (Example)

```
Endpoint               | Parse ✅ | Validate | API Test | Format | Error
────────────────────────────────────────────────────────────────────────
devices/list          | ✅     | ✅      | ✅      | ✅    | ✅
devices/get           | ☐      | ☐       | ☐       | ☐     | ☐
locks/get             | ☐      | ☐       | ☐       | ☐     | ☐
access_codes/update   | ☐      | ☐       | ☐       | ☐     | ☐
connected_accounts/* | ☐      | ☐       | ☐       | ☐     | ☐
events/*             | ☐      | ☐       | ☐       | ☐     | ☐
```

---

## Priority Endpoints (Quick Wins)

If you want quick wins, implement in this order:

1. **devices/get** (5 min)
2. **locks/get** (5 min)
3. **locks/list** (5 min)
4. **access_codes/get** (5 min)
5. **connected_accounts/list** (10 min)
6. **events/list** (15 min)

**Total:** ~1 day for 6 new endpoints

---

## Command Structure Examples

### For GET endpoints (simple)

```bash
rseam devices get --device-id "abc123"
rseam locks get --device-id "abc123"
rseam access-codes get --access-code-id "ac_123"
rseam events list --limit 50
```

### For POST/UPDATE endpoints (complex)

```bash
rseam access-codes update \
  --access-code-id "ac_123" \
  --code "5678" \
  --name "Updated Code" \
  --ends-at "2025-12-31T23:59:59Z"

rseam connected-accounts delete \
  --connected-account-id "ca_123"
```

---

## Metrics

| Phase | Endpoints | Effort | LOC Est. | Tests |
|-------|-----------|--------|----------|-------|
| v0.1.0 (Current) | 9 | Done | 756 | 30 |
| v0.2.0 (Phase 1) | +12 | 3-4d | +400 | +15 |
| v0.3.0 (Phase 2) | +7 | 2-3d | +250 | +10 |
| v0.4.0 (Phase 3) | +11 | 3-4d | +350 | +15 |
| v0.5.0 (Phase 4) | +19 | 4-5d | +450 | +20 |
| v0.6.0+ (Phase 5) | +164 | +20d | +1000+ | +50+ |

**Total to Full API Coverage: ~30-35 days of development**

---

## Documentation

After each phase, update:
- [ ] API_COVERAGE.md (completion percentages)
- [ ] README.md (supported commands)
- [ ] ENDPOINT_TEST_MATRIX.md (test status)

---

## Recommendations

**For MVP / Production Use:**
- Focus on Phase 1 + Phase 2
- That gives you ~28 endpoints (12.6% coverage)
- Covers 95% of typical use cases

**For Exhaustive Coverage:**
- Complete all 5 phases
- Adds 222 endpoints total
- Takes ~1-2 months of dedicated effort

**For Community Contribution:**
- Implement endpoints as contributions
- Each endpoint = ~1-2 hours of work
- Good starter issues for contributors
