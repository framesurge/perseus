#!/usr/bin/env rust-script
//! This script manages WebDriver (geckodriver) for running tests.
//! It checks if geckodriver is installed, installs it if needed,
//! starts it if not running, and then runs the tests.

use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::process::{Command, Stdio, ExitCode};
use std::thread;
use std::time::Duration;

const WEBDRIVER_PORT: u16 = 4444;

fn is_port_open(port: u16) -> bool {
    TcpStream::connect(format!("127.0.0.1:{}", port))
        .is_ok()
}

fn is_geckodriver_installed() -> bool {
    Command::new("geckodriver")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn install_geckodriver() -> Result<(), String> {
    println!("📦 geckodriver not found. Attempting to install...");

    #[cfg(target_os = "linux")]
    {
        // Try to detect the package manager and install
        if Command::new("apt").arg("--version").output().is_ok() {
            println!("   Using apt to install firefox-geckodriver...");
            let status = Command::new("sudo")
                .args(["apt", "install", "-y", "firefox-geckodriver"])
                .status()
                .map_err(|e| format!("Failed to run apt: {}", e))?;

            if !status.success() {
                return Err("apt install failed".to_string());
            }
        } else if Command::new("dnf").arg("--version").output().is_ok() {
            println!("   Using dnf to install geckodriver...");
            let status = Command::new("sudo")
                .args(["dnf", "install", "-y", "geckodriver"])
                .status()
                .map_err(|e| format!("Failed to run dnf: {}", e))?;

            if !status.success() {
                return Err("dnf install failed".to_string());
            }
        } else if Command::new("pacman").arg("--version").output().is_ok() {
            println!("   Using pacman to install geckodriver...");
            let status = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "geckodriver"])
                .status()
                .map_err(|e| format!("Failed to run pacman: {}", e))?;

            if !status.success() {
                return Err("pacman install failed".to_string());
            }
        } else {
            return Err("No supported package manager found. Please install geckodriver manually from: https://github.com/mozilla/geckodriver/releases".to_string());
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("   Using brew to install geckodriver...");
        let status = Command::new("brew")
            .args(["install", "geckodriver"])
            .status()
            .map_err(|e| format!("Failed to run brew: {}", e))?;

        if !status.success() {
            return Err("brew install failed. Please install homebrew or install geckodriver manually from: https://github.com/mozilla/geckodriver/releases".to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        println!("   Using choco to install geckodriver...");
        let status = Command::new("choco")
            .args(["install", "geckodriver", "-y"])
            .status()
            .map_err(|e| format!("Failed to run choco: {}", e))?;

        if !status.success() {
            return Err("choco install failed. Please install chocolatey or install geckodriver manually from: https://github.com/mozilla/geckodriver/releases".to_string());
        }
    }

    println!("✅ geckodriver installed successfully!");
    Ok(())
}

fn start_geckodriver() -> Result<(), String> {
    println!("🚀 Starting geckodriver on port {}...", WEBDRIVER_PORT);

    let mut child = Command::new("geckodriver")
        .arg("--port")
        .arg(WEBDRIVER_PORT.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start geckodriver: {}", e))?;

    // Wait for geckodriver to be ready by monitoring its output
    let stderr = child.stderr.take().unwrap();
    let reader = BufReader::new(stderr);

    let mut ready = false;
    let timeout = Duration::from_secs(10);
    let start = std::time::Instant::now();

    for line in reader.lines() {
        if start.elapsed() > timeout {
            let _ = child.kill();
            return Err("Timeout waiting for geckodriver to start".to_string());
        }

        if let Ok(line) = line {
            if line.contains("Listening on") {
                ready = true;
                break;
            }
        }
    }

    if !ready {
        let _ = child.kill();
        return Err("geckodriver did not start properly".to_string());
    }

    // Detach the process so it continues running
    // We don't store the child handle, letting it run in the background
    std::mem::forget(child);

    // Give it a moment to fully initialize
    thread::sleep(Duration::from_millis(500));

    println!("✅ geckodriver is running on port {}", WEBDRIVER_PORT);
    Ok(())
}

fn run_tests(test_args: Vec<String>) -> ExitCode {
    println!("🧪 Running tests...\n");

    let child = Command::new("cargo")
        .args(&test_args)
        .spawn()
        .expect("Failed to run tests");

    let output = child
        .wait_with_output()
        .expect("Failed to wait on test process");

    if output.status.success() {
        println!("\n✅ Tests passed!");
        ExitCode::SUCCESS
    } else {
        println!("\n❌ Tests failed!");
        ExitCode::FAILURE
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: test_with_webdriver.rs <test_command> [args...]");
        eprintln!("Example: test_with_webdriver.rs test --test integration_test");
        return ExitCode::FAILURE;
    }

    println!("🔍 Checking WebDriver setup...\n");

    // Check if geckodriver is already running
    if is_port_open(WEBDRIVER_PORT) {
        println!("✅ geckodriver is already running on port {}", WEBDRIVER_PORT);
    } else {
        // Check if geckodriver is installed
        if !is_geckodriver_installed() {
            if let Err(e) = install_geckodriver() {
                eprintln!("❌ Failed to install geckodriver: {}", e);
                eprintln!("\nPlease install geckodriver manually:");
                eprintln!("  - Download from: https://github.com/mozilla/geckodriver/releases");
                eprintln!("  - Or use your package manager:");
                eprintln!("    Linux (apt): sudo apt install firefox-geckodriver");
                eprintln!("    Linux (dnf): sudo dnf install geckodriver");
                eprintln!("    macOS: brew install geckodriver");
                eprintln!("    Windows: choco install geckodriver");
                return ExitCode::FAILURE;
            }

            // Verify installation worked
            if !is_geckodriver_installed() {
                eprintln!("❌ geckodriver installation completed but executable not found in PATH");
                return ExitCode::FAILURE;
            }
        } else {
            println!("✅ geckodriver is installed");
        }

        // Start geckodriver
        if let Err(e) = start_geckodriver() {
            eprintln!("❌ Failed to start geckodriver: {}", e);
            eprintln!("\nYou can try starting it manually:");
            eprintln!("  geckodriver --port {}", WEBDRIVER_PORT);
            return ExitCode::FAILURE;
        }
    }

    println!();

    // Run the tests
    run_tests(args)
}
