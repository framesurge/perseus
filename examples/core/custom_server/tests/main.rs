use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));

    wait_for_checkpoint!("initial_state_present", 0, c);
    wait_for_checkpoint!("page_visible", 0, c);
    let greeting = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(greeting, "Hello World!");

    // Go to `/about`
    c.find(Locator::Id("about-link")).await?.click().await?;
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080/about"));
    wait_for_checkpoint!("initial_state_not_present", 0, c);
    wait_for_checkpoint!("page_visible", 1, c);
    // Make sure the hardcoded text there exists
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "About.");
    // Make sure we get initial state if we refresh
    c.refresh().await?;
    wait_for_checkpoint!("initial_state_present", 0, c);

    // Check the API routes (we test with a server)
    // The echo route should echo back everything we give it
    c.goto("http://localhost:8080/api/echo/test").await?;
    let text = c.find(Locator::Css("pre")).await?.text().await?;
    assert_eq!(text, "test");
    c.goto("http://localhost:8080/api/echo/blah").await?;
    let text = c.find(Locator::Css("pre")).await?.text().await?;
    assert_eq!(text, "blah");

    Ok(())
}
