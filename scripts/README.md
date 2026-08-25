# Perseus Scripts

These are cross-platform scripts written in Rust, and designed to be executed with [`rust-script`](https://github.com/fornwall/rust-script). These are intended primarily for complex tasks involving manipulating files in the repository, rather than command-line tasks that could easily be defined with Bonnie. Note that Bonnie is used as a frontend over all the scripts in this directory.

Note that these are all designed to be executed from the root of the project.

## WebDriver Test Automation

### ensure_webdriver.rs

Automatically manages WebDriver (geckodriver) for running integration tests.

**Features:**
- **Automatic Detection**: Checks if geckodriver is already running on port 4444
- **Automatic Installation**: Installs geckodriver using your system's package manager if not found
- **Automatic Startup**: Starts geckodriver in the background if not already running
- **Port Polling**: Waits up to 10 seconds for geckodriver to become available

**Usage:**

This script is automatically called by `bonnie test` commands. You can also run it manually:

```bash
rust-script scripts/ensure_webdriver.rs
```

**How It Works:**

1. Port Check: Tests if port 4444 is open (geckodriver running)
2. Installation Check: Verifies geckodriver is installed
3. Auto-Install: If not installed, attempts package manager installation
4. Startup: Launches geckodriver with output redirected to null
5. Readiness: Polls port 4444 every 100ms until available or timeout
6. Detach: Allows geckodriver to run in the background

**Technical Details:**
- Uses `TcpStream::connect_timeout()` to check port availability instead of parsing geckodriver's output to avoid blocking issues
- Uses `std::mem::forget()` to detach the geckodriver process
- 10-second timeout with 100ms polling interval

### test_with_webdriver.rs

Alternative script that wraps arbitrary test commands with WebDriver setup, providing more flexibility for custom test scenarios.

**Usage:**

```bash
rust-script scripts/test_with_webdriver.rs test --test integration_test
```
