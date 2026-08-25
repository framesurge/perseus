/// A simple macro for listening for a checkpoint in a test.
///
/// This uses a 120-second timeout to accommodate complex examples that may take
/// longer to initialize their WASM bundles, especially in CI environments.
///
/// NOTE: This implementation uses manual polling with .find() instead of .wait().for_element()
/// due to compatibility issues between Fantoccini 0.22 and geckodriver 0.36.
#[macro_export]
macro_rules! wait_for_checkpoint {
    ($checkpoint:literal, $count:literal, $client:expr) => {{
        let checkpoint_id = format!("__perseus_checkpoint-{}-{}", $checkpoint, $count);
        let start = ::std::time::Instant::now();
        let timeout = ::std::time::Duration::from_secs(120);
        let poll_interval = ::std::time::Duration::from_millis(100);

        loop {
            // Try to find the element
            match $client
                .find(::fantoccini::Locator::Id(&checkpoint_id))
                .await
            {
                Ok(_) => break, // Found it!
                Err(_) => {
                    // Not found yet, check if we've timed out
                    if start.elapsed() > timeout {
                        panic!(
                            "Timeout waiting for checkpoint: {} (waited {:?})",
                            checkpoint_id,
                            start.elapsed()
                        );
                    }
                    // Sleep briefly before retrying
                    ::tokio::time::sleep(poll_interval).await;
                }
            }
        }
    }};
}
