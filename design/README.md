# rseam Design & Planning

Design documents, architecture decisions, and implementation roadmaps.

## Core Files

### PHASE_1_DEVICES.md ⭐ START HERE
**Device-focused implementation plan for rseam MVP**

- 13 device-related endpoints
- Full CRUD for: devices, locks, access codes
- Sectioned by priority (essential → nice-to-have)
- Unit-test focused (2 offline locks, can't do live API tests)
- ~4.5 hours total effort
- Ready for beads task tracking

**Current Status:** 7/13 endpoints done

### PHASE_2_FUTURE.md
**Everything else (lower priority)**

- 200+ remaining endpoints
- Organized by category (events, users, ACS, thermostats, etc.)
- ~100-150 hours total effort
- Only start after Phase 1 is 100% complete
- No commitment to complete

---

## Reference Files (For Context)

### COMPLETE_API_COVERAGE.md
Complete breakdown of all 222 Seam API endpoints

### IMPLEMENTATION_ROADMAP.md
Original 5-phase plan (superseded by new prioritization)

### ENDPOINT_TEST_MATRIX.md
Test coverage tracking template

### API_COVERAGE.md
High-level API structure reference

---

## Quick Reference

### Phase 1: Device Suite (THIS SPRINT)

**Must Complete (MVP):**
```
Section 1: Device Core (4 endpoints, 35 min)
  ✅ devices/get
  ✅ devices/list
  ☐ devices/update
  ☐ devices/delete

Section 2: Lock Control (4 endpoints, 40 min)
  ✅ locks/unlock_door
  ✅ locks/lock_door
  ☐ locks/get
  ☐ locks/list

Section 3: Access Code Management (6 endpoints, 95 min)
  ✅ access_codes/create
  ✅ access_codes/list
  ✅ access_codes/delete
  ☐ access_codes/get
  ☐ access_codes/update
  ☐ access_codes/generate_code
```

**Nice-to-Have (if time allows):**
```
Section 4: Advanced Codes (2 endpoints, 40 min)
  ☐ access_codes/create_multiple
  ☐ access_codes/update_multiple

Section 5: Code Scheduling (2 endpoints, 50 min)
  ☐ access_codes/get_timeline
  ☐ access_codes/pull_backup_access_code
```

### Phase 2: Everything Else (LATER)

Only start after Phase 1 is shipped.

Priority order:
1. events (2) - high debug value
2. workspaces (basic CRUD) - multi-org foundation
3. user_identities (CRUD only) - user mgmt
4. ACS basic (2) - enterprise support
5. Everything else based on demand

---

## Testing Philosophy

**For Phase 1:** Unit tests only
- Mock API responses
- Test parameter validation
- Test output formatting
- Test error handling

**Why?** You have 2 offline locks. Live API testing isn't feasible.

**Result:** Sufficient for MVP quality

---

## Structure

```
design/
  ├─ README.md (this file)
  ├─ PHASE_1_DEVICES.md (action plan)
  ├─ PHASE_2_FUTURE.md (next phase)
  ├─ COMPLETE_API_COVERAGE.md (reference)
  ├─ IMPLEMENTATION_ROADMAP.md (reference)
  ├─ ENDPOINT_TEST_MATRIX.md (tracking)
  └─ API_COVERAGE.md (reference)
```

---

## Next Step

1. Read **PHASE_1_DEVICES.md** in full
2. Use beads to track the 13 endpoints
3. Check back when ready to start implementation

Then we build! 🚀
