use fantoccini::{ClientBuilder, Locator};
use std::time::Duration;

// Mimic the Perseus test macro structure exactly
#[tokio::test]
async fn test_like_perseus() {
    println!("Starting test_like_perseus...");

    // Only run if env var is set (like Perseus does)
    if std::env::var("PERSEUS_RUN_WASM_TESTS").is_ok() {
        println!("PERSEUS_RUN_WASM_TESTS is set, running test...");

        let headless = std::env::var("PERSEUS_RUN_WASM_TESTS_HEADLESS").is_ok();
        println!("Headless mode: {}", headless);

        // Set capabilities exactly like Perseus macro
        let mut capabilities = serde_json::Map::new();
        let firefox_opts;
        let chrome_opts;
        if headless {
            firefox_opts = serde_json::json!({ "args": ["--headless"] });
            chrome_opts = serde_json::json!({ "args": ["--headless=new", "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"] });
        } else {
            firefox_opts = serde_json::json!({ "args": [] });
            chrome_opts = serde_json::json!({ "args": [] });
        }
        capabilities.insert("moz:firefoxOptions".to_string(), firefox_opts);
        capabilities.insert("goog:chromeOptions".to_string(), chrome_opts);

        println!("Connecting to WebDriver at http://localhost:4444...");

        let mut client = ClientBuilder::native()
            .capabilities(capabilities)
            .connect("http://localhost:4444")
            .await
            .expect("failed to connect to WebDriver");

        println!("Connected! Navigating to http://localhost:8080...");

        // Run the inner test
        let result = inner_test(&mut client).await;

        println!("Closing client...");
        client.close().await.expect("failed to close client");

        if let Err(e) = result {
            panic!("test failed: '{}'", e);
        }

        println!("Test passed!");
    } else {
        println!("PERSEUS_RUN_WASM_TESTS not set, skipping test");
    }
}

async fn inner_test(c: &mut fantoccini::Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    println!("Page loaded!");

    // Wait for checkpoint
    println!("Waiting for begin checkpoint...");
    c.wait()
        .at_most(Duration::from_secs(30))
        .for_element(Locator::Id("__perseus_checkpoint-begin-0"))
        .await?;
    println!("Found begin checkpoint!");

    let url = c.current_url().await?;
    println!("Current URL: {}", url.as_ref());

    Ok(())
}
