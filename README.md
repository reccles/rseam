# rseam

**Unofficial third-party Rust CLI for the Seam API**

A command-line interface for the [Seam API](https://docs.seam.co), written in Rust. Provides non-interactive access to Seam endpoints for scripting and automation.

This is an independent implementation and is not affiliated with, endorsed by, or related to [Seam](https://seam.co) or the [official seam-cli](https://github.com/seamapi/seam-cli).

## Disclaimer

This is an early-stage project. While tested, use at your own risk. Not affiliated with Seam. Refer to the [Seam API Terms of Service](https://seam.co/terms) and ensure compliance with your account agreement.

## Building

Requirements: Rust 1.70+

```bash
git clone https://github.com/yourusername/rseam.git
cd rseam
cargo build --release
./target/release/rseam --help
```

## Running

Set your API key:

```bash
export SEAM_API_KEY="your_seam_api_key_here"
```

Then run commands:

```bash
# Health check
rseam health get-health

# List devices
rseam devices list

# Get a specific device
rseam devices get --name "Front Door"

# Create an access code
rseam access-codes create --device-id "dev_123" --code "1234" --name "Guest"

# Unlock a door
rseam locks unlock-door --device-id "dev_123"
```

## Output Formats

By default, output is pretty-printed JSON:

```bash
rseam devices list
```

For single-line JSON (useful for piping):

```bash
rseam devices list --raw
```

For ID-only output (useful in scripts):

```bash
DEVICE_ID=$(rseam devices get --name "Front Door" --id-only)
rseam locks unlock-door --device-id "$DEVICE_ID"
```

## Commands

- `devices list` - List all devices
- `devices get` - Get a specific device by ID or name
- `access-codes create` - Create an access code
- `access-codes list` - List access codes for a device
- `access-codes delete` - Delete an access code
- `locks unlock-door` - Unlock a door
- `locks lock-door` - Lock a door
- `connect-webviews create` - Create a connect webview
- `health get-health` - Check API health

## Testing

```bash
# Unit tests
cargo test

# Integration tests (requires SEAM_API_KEY)
export SEAM_API_KEY="your_key"
cargo test -- --ignored
```

## Project Structure

```
src/
  main.rs            - Entry point
  cli.rs             - CLI argument parsing
  api_client.rs      - HTTP client
  error.rs           - Error types
  types.rs           - Data structures
  commands/          - Command implementations
    mod.rs
    devices.rs
    access_codes.rs
    locks.rs
    health.rs
    connect_webviews.rs
```

## License

MIT - See LICENSE file
