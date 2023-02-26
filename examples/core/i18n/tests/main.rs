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
    // Make sure the HTML `lang` attribute has been correctly set
    let lang = c.find(Locator::Css("html")).await?.attr("lang").await?;
    assert_eq!(lang, Some("en-US".to_string()));
    // This tests translations, variable interpolation, and multiple aspects of
    // Sycamore all at once
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "Hello, \u{2068}User\u{2069}!"); // Ask Fluent about these characters

    c.find(Locator::Css("a")).await?.click().await?;
    // This tests i18n linking (locale should be auto-detected)
    let url = c.current_url().await?;
    wait_for_checkpoint!("page_interactive", 1, c);
    assert!(url
        .as_ref()
        .starts_with("http://localhost:8080/en-US/about"));

    // Switch the locale
    c.find(Locator::Id("switch-button")).await?.click().await?;
    let url = c.current_url().await?;
    wait_for_checkpoint!("page_interactive", 2, c);
    assert!(url
        .as_ref()
        .starts_with("http://localhost:8080/fr-FR/about"));

    // Make sure the HTML `lang` attribute has been correctly set
    let lang = c.find(Locator::Css("html")).await?.attr("lang").await?;
    assert_eq!(lang, Some("fr-FR".to_string()));

    Ok(())
}
