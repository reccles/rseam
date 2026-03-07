# Phase 1: Complete Device Interaction Suite

## Focus
**All device-related operations**: reads, writes, control, and code management

## Current Status
- ✅ devices/list
- ✅ devices/get
- ✅ locks/unlock_door
- ✅ locks/lock_door
- ✅ access_codes/create
- ✅ access_codes/list
- ✅ access_codes/delete

**Partially Complete:** 7/13 required endpoints

---

## Phase 1 Endpoints (Priority Order)

### Section 1: Device Core (ESSENTIAL)
Mandatory for basic device scripting

#### 1.1 - devices/get (DONE ✅)
- Status: Implemented
- Tests: CLI parsing, parameter validation
- Effort: 0 min (complete)

#### 1.2 - devices/list (DONE ✅)
- Status: Implemented
- Tests: CLI parsing, filtering
- Effort: 0 min (complete)

#### 1.3 - devices/update (TODO)
- Purpose: Update device properties (name, properties)
- Complexity: Medium
- Effort: 15-20 min
- Parameters: device_id, name, properties
- Tests: Unit test (parameter validation)

#### 1.4 - devices/delete (TODO)
- Purpose: Remove device from workspace
- Complexity: Low
- Effort: 10 min
- Parameters: device_id
- Tests: Unit test (parameter validation)

---

### Section 2: Lock Control (ESSENTIAL)
Core door/lock operations

#### 2.1 - locks/unlock_door (DONE ✅)
- Status: Implemented
- Tests: CLI parsing, parameter validation
- Effort: 0 min (complete)

#### 2.2 - locks/lock_door (DONE ✅)
- Status: Implemented
- Tests: CLI parsing, parameter validation
- Effort: 0 min (complete)

#### 2.3 - locks/get (TODO)
- Purpose: Get lock status and properties
- Complexity: Low
- Effort: 10 min
- Parameters: device_id
- Tests: Unit test (mock response validation)

#### 2.4 - locks/list (TODO)
- Purpose: List all locks in workspace
- Complexity: Low
- Effort: 10 min
- Parameters: (optional filters)
- Tests: Unit test (filtering, pagination)

---

### Section 3: Access Code Management (CORE)
Create, read, update, delete codes

#### 3.1 - access_codes/create (DONE ✅)
- Status: Implemented
- Tests: CLI parsing, code generation
- Effort: 0 min (complete)

#### 3.2 - access_codes/list (DONE ✅)
- Status: Implemented
- Tests: CLI parsing, filtering
- Effort: 0 min (complete)

#### 3.3 - access_codes/delete (DONE ✅)
- Status: Implemented
- Tests: CLI parsing, parameter validation
- Effort: 0 min (complete)

#### 3.4 - access_codes/get (TODO)
- Purpose: Get single access code details
- Complexity: Low
- Effort: 10 min
- Parameters: access_code_id
- Tests: Unit test (mock response)

#### 3.5 - access_codes/update (TODO)
- Purpose: Modify code (enable/disable, time bounds, name)
- Complexity: High (many optional params)
- Effort: 30 min
- Parameters: access_code_id, code, name, starts_at, ends_at, etc.
- Tests: Unit test (parameter combinations)

#### 3.6 - access_codes/generate_code (TODO)
- Purpose: Auto-generate code instead of providing one
- Complexity: Medium
- Effort: 15 min
- Parameters: device_id, name, length
- Tests: Unit test (parameter validation)

---

### Section 4: Advanced Code Operations (NICE TO HAVE)
Specialized access code features

#### 4.1 - access_codes/create_multiple (TODO)
- Purpose: Batch create multiple codes
- Complexity: Medium
- Effort: 20 min
- Parameters: codes array
- Tests: Unit test

#### 4.2 - access_codes/update_multiple (TODO)
- Purpose: Batch update multiple codes
- Complexity: Medium
- Effort: 20 min
- Parameters: updates array
- Tests: Unit test

---

### Section 5: Code Scheduling (NICE TO HAVE)
Time-bound access codes

#### 5.1 - access_codes/get_timeline (TODO)
- Purpose: Get historical timeline of code changes
- Complexity: Low
- Effort: 10 min
- Parameters: access_code_id
- Tests: Unit test

#### 5.2 - access_codes/pull_backup_access_code (TODO)
- Purpose: Get backup codes from provider
- Complexity: Medium
- Effort: 20 min
- Parameters: access_code_id
- Tests: Unit test

---

## Summary Table

| Endpoint | Status | Complexity | Effort | Tests |
|----------|--------|-----------|--------|-------|
| devices/get | ✅ DONE | Low | 0 | ✅ |
| devices/list | ✅ DONE | Low | 0 | ✅ |
| devices/update | TODO | Medium | 15 min | Unit |
| devices/delete | TODO | Low | 10 min | Unit |
| locks/unlock_door | ✅ DONE | Medium | 0 | ✅ |
| locks/lock_door | ✅ DONE | Medium | 0 | ✅ |
| locks/get | TODO | Low | 10 min | Unit |
| locks/list | TODO | Low | 10 min | Unit |
| access_codes/create | ✅ DONE | Medium | 0 | ✅ |
| access_codes/list | ✅ DONE | Low | 0 | ✅ |
| access_codes/delete | ✅ DONE | Low | 0 | ✅ |
| access_codes/get | TODO | Low | 10 min | Unit |
| access_codes/update | TODO | High | 30 min | Unit |
| access_codes/generate_code | TODO | Medium | 15 min | Unit |
| access_codes/create_multiple | TODO | Medium | 20 min | Unit |
| access_codes/update_multiple | TODO | Medium | 20 min | Unit |
| access_codes/get_timeline | TODO | Low | 10 min | Unit |
| access_codes/pull_backup_access_code | TODO | Medium | 20 min | Unit |

---

## Testing Strategy

### Since you only have 2 offline locks:

**Unit Tests Only** (sufficient for MVP)
- Mock API responses with realistic data
- Test parameter validation
- Test output formatting
- Test error handling

**No Live API Tests** (not feasible with offline devices)
- Skip integration tests
- Use recorded responses

### Per-Endpoint Test Pattern

```rust
#[test]
fn test_{endpoint}_cli_parsing() {
    // Test CLI argument parsing
}

#[test]
fn test_{endpoint}_parameter_validation() {
    // Test required params, types, constraints
}

#[test]
fn test_{endpoint}_response_formatting() {
    // Test pretty/raw/id-only output
}

#[test]
fn test_{endpoint}_error_handling() {
    // Test error messages
}
```

---

## Effort Breakdown

### Must Complete (MVP)
- Section 1 (4 endpoints): 35 min
- Section 2 (4 endpoints): 40 min
- Section 3 (6 endpoints): 95 min
- **Subtotal: 170 min (~3 hours)**

### Nice to Have
- Section 4 (2 endpoints): 40 min
- Section 5 (2 endpoints): 50 min
- **Subtotal: 90 min (~1.5 hours)**

**Total Phase 1: ~4.5 hours of implementation**

---

## Beads Tracking

Each TODO item should have a bead:

```
Section 1:
  [ ] 1.3 devices/update
  [ ] 1.4 devices/delete

Section 2:
  [ ] 2.3 locks/get
  [ ] 2.4 locks/list

Section 3:
  [ ] 3.4 access_codes/get
  [ ] 3.5 access_codes/update
  [ ] 3.6 access_codes/generate_code

Section 4:
  [ ] 4.1 access_codes/create_multiple
  [ ] 4.2 access_codes/update_multiple

Section 5:
  [ ] 5.1 access_codes/get_timeline
  [ ] 5.2 access_codes/pull_backup_access_code
```

---

## Phase 1 Completion Criteria

✅ All 13 endpoints have CLI parsing
✅ All 13 endpoints have types defined
✅ All 13 endpoints have command handlers
✅ All 13 endpoints have unit tests
✅ All tests passing (30+ new tests)
✅ README updated with examples
✅ Zero integration tests (offline devices)

---

## What This Enables

Phase 1 complete gives you:
- ✅ Full device CRUD
- ✅ Full lock control
- ✅ Full access code lifecycle
- ✅ All basic device scripting use cases

Perfect for:
- Scripting device management
- Bulk code creation/deletion
- Lock control automation
- Event monitoring (via Phase 2)

---

## Notes

- All endpoints use POST (except GET variants)
- API key in request body
- JSON request/response
- No pagination issues (device counts are low)
- No complex interdependencies

Ready to implement? Check back before starting! 🚀
