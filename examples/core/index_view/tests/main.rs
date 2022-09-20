use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));

    // The greeting was passed through using build state
    wait_for_checkpoint!("initial_state_present", 0, c);
    wait_for_checkpoint!("page_interactive", 0, c);
    let greeting = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(greeting, "Hello World!");
    // Check the footer, the symbol of the index view having worked
    let footer = c.find(Locator::Css("footer")).await?.text().await?;
    assert_eq!(footer, "This is a footer!");

    // Go to `/about`
    c.find(Locator::Id("about-link")).await?.click().await?;
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080/about"));
    wait_for_checkpoint!("initial_state_not_present", 0, c);
    wait_for_checkpoint!("page_interactive", 1, c);
    // Make sure the hardcoded text there exists
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "About.");
    // Check the footer, the symbol of the index view having worked
    let footer = c.find(Locator::Css("footer")).await?.text().await?;
    assert_eq!(footer, "This is a footer!");
    // Make sure we get initial state if we refresh
    c.refresh().await?;
    wait_for_checkpoint!("initial_state_present", 0, c);

    Ok(())
}
