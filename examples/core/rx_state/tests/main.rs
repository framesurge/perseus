use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));
    wait_for_checkpoint!("page_interactive", 0, c);

    // The initial greeting should be to an empty string
    let greeting = c.find(Locator::Css("p")).await?;
    assert_eq!(greeting.text().await?, "Greetings, !");
    // Now type some text in, and it should be reactively reflected straight away
    c.find(Locator::Css("input"))
        .await?
        .send_keys("Test User")
        .await?;
    assert_eq!(greeting.text().await?, "Greetings, Test User!");

    // Go to the about page and make sure the changed greeting is reflected once we
    // go back. This tests that pages can access each others' states.
    // Note: Navigation uses buttons with on:click handlers calling navigate()
    // because sycamore-router doesn't attach click handlers to dynamic views.
    c.find(Locator::Id("about-link")).await?.click().await?;
    wait_for_checkpoint!("page_interactive", 1, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080/about"));

    // Navigate back to the index page
    c.find(Locator::Id("index-link")).await?.click().await?;
    wait_for_checkpoint!("page_interactive", 2, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));

    // The greeting should still have the user's input (state persisted across navigation)
    let greeting = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(greeting, "Greetings, Test User!");

    Ok(())
}
