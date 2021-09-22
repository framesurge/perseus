/// A simple macro for listening for a checkpoint in a test.
#[macro_export]
macro_rules! wait_for_checkpoint {
    ($checkpoint:literal, $count:literal, $client:expr) => {
        $client.wait()
            .for_element(::fantoccini::Locator::Id(&format!("__perseus_checkpoint-{}-{}", $checkpoint, $count)))
            .await?;
    };
}
