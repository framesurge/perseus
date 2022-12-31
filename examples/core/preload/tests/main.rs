use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

// IMPORTANT: These tests *do not* test the actual preloading system, they
// merely make sure that nothing explicitly fails, and that the API works.
// Actual tests of the preloading system can be found in the `core/capsules`
// example for incremental widget use in a reactive way, where preloading can be
// tested with micro-delays.
//
// These tests just test the bassic pathways of preloading (with i18n).
//
// This also tests switching locales, really.
#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    let url = c.current_url().await?;
    // Locale redirection
    assert!(url.as_ref().starts_with("http://localhost:8080/en-US"));

    let greeting = c.find(Locator::Css("p")).await?.text().await?;
    assert!(greeting.contains("Open up"));

    // Go to `/en-US/about`
    c.find(Locator::Id("about")).await?.click().await?;
    let url = c.current_url().await?;
    assert!(url
        .as_ref()
        .starts_with("http://localhost:8080/en-US/about"));
    wait_for_checkpoint!("page_interactive", 1, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("Check out"));

    // Go back to `/en-US`
    c.find(Locator::Id("index")).await?.click().await?;
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080/en-US"));
    wait_for_checkpoint!("page_interactive", 2, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("Open up"));

    // Go to `/fr-FR/about`
    c.find(Locator::Id("fr-about")).await?.click().await?;
    let url = c.current_url().await?;
    assert!(url
        .as_ref()
        .starts_with("http://localhost:8080/fr-FR/about"));
    wait_for_checkpoint!("page_interactive", 3, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("cette page"));

    // Go to `/fr-FR`
    c.find(Locator::Id("index")).await?.click().await?;
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080/fr-FR"));
    wait_for_checkpoint!("page_interactive", 3, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("ci-dessous"));

    // Now go back to `/en-US`
    c.find(Locator::Id("en-about")).await?.click().await?;
    let url = c.current_url().await?;
    assert!(url
        .as_ref()
        .starts_with("http://localhost:8080/en-US/about"));
    wait_for_checkpoint!("page_interactive", 4, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("Check out"));

    // Make sure we get initial state if we refresh
    c.refresh().await?;
    wait_for_checkpoint!("page_interactive", 0, c);

    Ok(())
}
