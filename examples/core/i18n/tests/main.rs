use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    let url = c.current_url().await?;
    // We only test for one locale here because changing the browser's preferred
    // languages is very hard, we do unit testing on the locale detection system
    // instead
    assert!(url.as_ref().starts_with("http://localhost:8080/en-US"));
    // This tests translations, variable interpolation, and multiple aspects of
    // Sycamore all at once
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "Hello, \u{2068}User\u{2069}!"); // Ask Fluent about these characters

    c.find(Locator::Css("a")).await?.click().await?;
    // This tests i18n linking (locale should be auto-detected)
    let url = c.current_url().await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    assert!(url
        .as_ref()
        .starts_with("http://localhost:8080/en-US/about"));

    Ok(())
}
