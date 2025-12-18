/// A simple macro for listening for a checkpoint in a test.
///
/// This uses a 120-second timeout to accommodate complex examples that may take
/// longer to initialize their WASM bundles, especially in CI environments.
#[macro_export]
macro_rules! wait_for_checkpoint {
    ($checkpoint:literal, $count:literal, $client:expr) => {
        $client
            .wait()
            .at_most(::std::time::Duration::from_secs(120))
            .for_element(::fantoccini::Locator::Id(&format!(
                "__perseus_checkpoint-{}-{}",
                $checkpoint, $count
            )))
            .await?;
    };
}
