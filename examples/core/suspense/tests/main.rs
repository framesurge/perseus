use std::time::Duration;

use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));

    // Get each of the greetings
    let mut first = c.find(Locator::Id("first")).await?;
    let mut second = c.find(Locator::Id("second")).await?;
    let mut third = c.find(Locator::Id("third")).await?;

    // Now assert initial fallbacks
    assert_eq!(first.text().await?, "Hello from the server!",);
    assert_eq!(second.text().await?, "Hello again from the server!",);
    assert_eq!(third.text().await?, "Hello again again from the server!",);

    // Wait (all handlers in this example have a delay on them for this!)
    wait_for_checkpoint!("page_interactive", 0, c);
    std::thread::sleep(Duration::from_secs(1));

    // And now assert that the handlers have all completed
    assert_eq!(first.text().await?, "Hello from the handler!",);
    assert_eq!(second.text().await?, "Hello again from the handler!",);
    assert_eq!(third.text().await?, "Hello again again from the handler!",);

    Ok(())
}
