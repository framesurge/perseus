use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

// Most of this has already been tested under `basic`, so we only test the
// impacts of plugins
#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));

    wait_for_checkpoint!("page_interactive", 0, c);
    let greeting = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(greeting, "Hello World!");
    // For some reason, retrieving the inner HTML or text of a `<title>` doesn't
    // work
    let title = c.find(Locator::Css("title")).await?.html(false).await?;
    assert!(title.contains("Index Page"));

    // BUG Right now, this is downloaded by the browser...
    // // Check that the static alias to `Cargo.toml` worked (added by a plugin)
    // c.goto("http://localhost:8080/Cargo.toml").await?;
    // let text = c.find(Locator::Css("body")).await?.text().await?;
    // assert!(text.starts_with("[package]"));

    Ok(())
}
