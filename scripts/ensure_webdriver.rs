#!/usr/bin/env rust-script
//! This script ensures WebDriver (geckodriver) is installed and running.
//! It's designed to be called before running integration tests.

use std::net::TcpStream;
use std::process::{Command, Stdio, Child};
use std::thread;
use std::time::Duration;

const WEBDRIVER_PORT: u16 = 4444;

fn is_port_open(port: u16) -> bool {
    TcpStream::connect_timeout(
        &format!("127.0.0.1:{}", port)
            .parse()
            .expect("Invalid address"),
        Duration::from_millis(100),
    )
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

fn start_geckodriver() -> Result<Child, String> {
    println!("🚀 Starting geckodriver on port {}...", WEBDRIVER_PORT);

    let child = Command::new("geckodriver")
        .arg("--port")
        .arg(WEBDRIVER_PORT.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start geckodriver: {}", e))?;

    // Wait for geckodriver to be ready by polling the port
    let timeout = Duration::from_secs(10);
    let start = std::time::Instant::now();
    let poll_interval = Duration::from_millis(100);

    while start.elapsed() < timeout {
        if is_port_open(WEBDRIVER_PORT) {
            println!("✅ geckodriver is running on port {}", WEBDRIVER_PORT);
            return Ok(child);
        }
        thread::sleep(poll_interval);
    }

    Err("Timeout waiting for geckodriver to start on port".to_string())
}

fn main() {
    println!("🔍 Checking WebDriver setup...");

    // Check if geckodriver is already running
    if is_port_open(WEBDRIVER_PORT) {
        println!("✅ geckodriver is already running on port {}", WEBDRIVER_PORT);
        return;
    }

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
            std::process::exit(1);
        }

        // Verify installation worked
        if !is_geckodriver_installed() {
            eprintln!("❌ geckodriver installation completed but executable not found in PATH");
            std::process::exit(1);
        }
    } else {
        println!("✅ geckodriver is installed");
    }

    // Start geckodriver
    match start_geckodriver() {
        Ok(child) => {
            // Detach the process so it continues running
            std::mem::forget(child);
        }
        Err(e) => {
            eprintln!("❌ Failed to start geckodriver: {}", e);
            eprintln!("\nYou can try starting it manually:");
            eprintln!("  geckodriver --port {}", WEBDRIVER_PORT);
            std::process::exit(1);
        }
    }
}
