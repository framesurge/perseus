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

    // The initial text should be "Hello World!"
    let mut greeting = c.find(Locator::Css("p")).await?;
    assert_eq!(greeting.text().await?, "Hello World!");
    // Now type some text in, and it should be reactively reflected straight away
    c.find(Locator::Css("input"))
        .await?
        .send_keys(" Extra text.")
        .await?;
    assert_eq!(greeting.text().await?, "Hello World! Extra text.");

    // Go to the about page and make sure the changed greeting is reflected there too
    // This tests that the global state is accessible from all pages
    c.find(Locator::Id("about-link")).await?.click().await?;
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080/about"));
    wait_for_checkpoint!("initial_state_not_present", 0, c);
    wait_for_checkpoint!("page_visible", 1, c);

    let greeting = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(greeting, "Hello World! Extra text.");

    Ok(())
}
