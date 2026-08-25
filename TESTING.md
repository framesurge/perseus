# Perseus Testing Guide

## Prerequisites

The test infrastructure automatically handles geckodriver setup! You just need:

1. **rust-script** (for running test automation scripts):
   ```bash
   cargo install rust-script
   ```

The tests will automatically:
- Check if geckodriver is installed
- Install it using your system's package manager if not found
- Start geckodriver on port 4444 if not already running

### Manual Installation (Optional)

If you prefer to install geckodriver manually or the automatic installation fails:

```bash
# On Debian/Ubuntu
sudo apt install firefox-geckodriver

# On Fedora/RHEL
sudo dnf install geckodriver

# On Arch
sudo pacman -S geckodriver

# On macOS
brew install geckodriver

# On Windows
choco install geckodriver

# Or download from: https://github.com/mozilla/geckodriver/releases
```

## Running Tests Locally

### Run All Tests
```bash
bonnie test
```
This will automatically start geckodriver if needed, then run all core tests, CLI tests, and example E2E tests.

### Run Specific Example E2E Test
```bash
# Using the bonnie command (recommended - automatically starts geckodriver):
bonnie test example-all-integrations core basic

# Direct pattern (you may need to start geckodriver manually):
EXAMPLE_INTEGRATION=axum bonnie dev example <category> <example_name> test

# Examples:
bonnie test example-all-integrations core freezing_and_thawing
bonnie test example-all-integrations core idb_freezing
bonnie test example-all-integrations core i18n
bonnie test example-all-integrations core index_view
bonnie test example-all-integrations core preload
bonnie test example-all-integrations core rx_state
```

### Clean Build Before Testing
```bash
# Clean example dist directory
rm -rf examples/core/<example_name>/dist

# Then run test
EXAMPLE_INTEGRATION=axum bonnie dev example core <example_name> test
```

### Debugging Tests

1. **View full test output**:
   ```bash
   EXAMPLE_INTEGRATION=axum bonnie dev example core freezing_and_thawing test 2>&1
   ```

2. **Search for specific output**:
   ```bash
   EXAMPLE_INTEGRATION=axum bonnie dev example core freezing_and_thawing test 2>&1 | grep "DEBUG"
   ```

3. **Save output to file**:
   ```bash
   EXAMPLE_INTEGRATION=axum bonnie dev example core freezing_and_thawing test 2>&1 | tee test_output.log
   ```

### Troubleshooting

#### "Session is already started" Error
geckodriver has a stale session. Restart it:
```bash
pkill -9 geckodriver
pkill -9 firefox
sleep 1
geckodriver --port 4444
```

#### Tests Timing Out
Increase timeout or check if the server started properly on http://localhost:8080

#### WASM Compilation Issues
The WASM build uses `--cfg=client` flag automatically via Perseus CLI.
Check `packages/perseus-cli/src/build.rs` for build configuration.

## Test File Locations

- **Example E2E tests**: `examples/core/<example_name>/tests/main.rs`
- **Test fixtures**: Look for `wait_for_checkpoint!` macros in test files
- **Template code**: `examples/core/<example_name>/src/templates/`

## Understanding Test Commands

bonnie.toml defines test commands:
- `bonnie test` - Runs all tests
- `bonnie test example-all-integrations` - Runs all example E2E tests
- The `EXAMPLE_INTEGRATION=axum` sets which server integration to use (axum, actix-web, or warp)

## Cargo Check for cfg-specific Code

Check client-side code (WASM):
```bash
RUSTFLAGS="--cfg=client" CARGO_TARGET_DIR="target_wasm" cargo check --target wasm32-unknown-unknown
```

Check engine-side code:
```bash
RUSTFLAGS="--cfg=engine" CARGO_TARGET_DIR="target_engine" cargo check
```

## Current Known Issues (as of migration to Sycamore 0.9.2)

1. **freezing_and_thawing**: `freeze()` returns empty string
   - File: `examples/core/freezing_and_thawing/src/templates/about.rs`
   - Issue: `render_ctx.freeze()` returns empty, likely in `packages/perseus/src/reactor/state.rs`

2. **Navigation after thaw**: URL doesn't change after thaw operation
   - Related to freeze returning empty

3. Other failing tests: idb_freezing, i18n, index_view, preload, rx_state
   - Likely related to Sycamore 0.9.2 reactive system changes
