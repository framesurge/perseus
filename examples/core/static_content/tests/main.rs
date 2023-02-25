use fantoccini::{Client, Locator};

// This will only test the existence of the static content
#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    // Check the static alias
    c.goto("http://localhost:8080/test.txt").await?;
    let url = c.current_url().await?;
    assert!(url
        .as_ref()
        .starts_with("http://localhost:8080/static/test"));
    // The browser will show this in the window as a `<pre>`
    let text = c.find(Locator::Css("pre")).await?.text().await?;
    assert_eq!(text, "This is a test file!");

    // Check the static content (no alias declared)
    c.goto("http://localhost:8080/.perseus/static/style.css")
        .await?;
    let url = c.current_url().await?;
    assert!(url
        .as_ref()
        .starts_with("http://localhost:8080/.perseus/static/style.css"));
    let text = c.find(Locator::Css("pre")).await?.text().await?;
    assert!(text.contains("background-color: red"));

    Ok(())
}
