# Phase 2: Everything Else (Lower Priority)

> "If we get here, great. If not, we'll revisit."

## Strategy

Implement all remaining 200+ endpoints after Phase 1 device suite is complete.

## Organized by Category

### Account Management (7 endpoints)

**connected_accounts**
- list
- get
- delete
- update
- sync
- simulate/disconnect

**workspaces**
- list
- get
- create
- update
- find_anything
- reset_sandbox

### Event & Logging (2 endpoints)

**events**
- list (with filters)
- get

### User & Identity Management (20 endpoints)

**user_identities**
- create, get, list, update, delete
- grant_access_to_device
- revoke_access_to_device
- enrollment_automations/* (4 endpoints)
- add_acs_user, remove_acs_user
- unmanaged/* (3 endpoints)

### Session Management (7 endpoints)

**client_sessions**
- create, get, list
- get_or_create
- grant_access
- revoke
- delete

### ACS Systems (51 endpoints)

Large subsystem for enterprise access control.

**Subsystems:**
- acs/systems (4)
- acs/entrances (4)
- acs/users (13)
- acs/access_groups (9)
- acs/credentials (12)
- acs/encoders (8)
- acs/credential_pools (1)

### Climate Control (25 endpoints)

**thermostats**
- get, list
- heat, cool, heat_cool, off
- set_hvac_mode, set_fan_mode
- climate_presets/* (5)
- schedules/* (5)
- daily_programs/* (3)
- simulate/* (2)

### Monitoring (7 endpoints)

**noise_sensors**
- list
- noise_thresholds/* (6)
- simulate/

### Other (30+ endpoints)

- Phones (4)
- Spaces (10)
- Webhooks (5)
- Instant Keys (3)
- Action Attempts (2)
- Bridges (2)
- Access Grants (10)
- Access Methods (7)
- Custom/Advanced features

## Effort Estimate

Total: ~100-150 hours (2-3 months dedicated)

Per entity effort varies widely:
- Simple LIST/GET: 10 min each
- CRUD endpoints: 15-20 min each
- Complex (ACS, thermostats): 30-60 min each

## When to Start

Only after Phase 1 is **100% complete and deployed**.

## If Time/Priority Allows

1. **Events** (2 endpoints) - High value for debugging
2. **Workspaces** (basic CRUD) - Foundation for multi-org
3. **User Identities** (CRUD only) - Basic user management
4. **ACS Systems** (systems/list, systems/get) - Enterprise support

Then everything else based on customer demand.

## No Commitment

This is "nice to have". Phase 1 is the real MVP.
