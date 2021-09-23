use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "http://localhost:8080/");

    // The greeting was passed through using build state
    wait_for_checkpoint!("initial_state_present", 0, c);
    wait_for_checkpoint!("page_visible", 0, c);
    let greeting = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(greeting, "Hello World!");
    // For some reason, retrieving the inner HTML or text of a `<title>` doens't work
    let title = c.find(Locator::Css("title")).await?.html(false).await?;
    assert_eq!(title, "<title>Index Page | Perseus Example – Basic</title>");

    // Go to `/about`
    c.find(Locator::Id("about-link")).await?.click().await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "http://localhost:8080/about");
    wait_for_checkpoint!("initial_state_not_present", 0, c);
    wait_for_checkpoint!("page_visible", 1, c);
    // Make sure the hardcoded text there exists
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "About.");
    let title = c.find(Locator::Css("title")).await?.html(false).await?;
    assert_eq!(title, "<title>About Page | Perseus Example – Basic</title>");
    // Make sure we get initial state if we refresh
    c.refresh().await?;
    wait_for_checkpoint!("initial_state_present", 0, c);

    Ok(())
}
