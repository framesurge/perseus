use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn build_state(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/build_state").await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    // This greeting is passed in as a build state prop
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "Hello World!");

    Ok(())
}

#[perseus::test]
async fn build_paths(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    async fn test_build_path(
        page: &str,
        c: &mut Client,
    ) -> Result<(), fantoccini::error::CmdError> {
        c.goto(&format!("http://localhost:8080/build_paths{}", page))
            .await?;
        wait_for_checkpoint!("page_interactive", 0, c);
        // There should be a heading with the slug
        let text = c.find(Locator::Css("h1")).await?.text().await?;
        assert!(text.contains(&format!("build_paths{}", page)));

        Ok(())
    }
    test_build_path("", c).await?;
    test_build_path("/test", c).await?;
    test_build_path("/blah/test/blah", c).await?;
    // Test an unbuilt URL
    c.goto("http://localhost:8080/build_paths/tests").await?;
    // There should be an error page
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("not found"));

    Ok(())
}

// This tests build state, build paths, and incremental generation
// simultaneously
#[perseus::test]
async fn incremental_generation(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    async fn test_incremental_generation(
        page: &str,
        c: &mut Client,
    ) -> Result<(), fantoccini::error::CmdError> {
        c.goto(&format!(
            "http://localhost:8080/incremental_generation{}",
            page
        ))
        .await?;
        wait_for_checkpoint!("page_interactive", 0, c);
        // There should be a heading with the slug
        let text = c.find(Locator::Css("h1")).await?.text().await?;
        assert!(text.contains(page));

        Ok(())
    }
    // First, test the build paths pages
    test_incremental_generation("/test", c).await?;
    test_incremental_generation("/blah/test/blah", c).await?;
    // Now test some incremental generation routes
    test_incremental_generation("/this/is/a/route/that/wasnt/prerendered/it/was/generated/on/the/server/dynamically/at/request/time", c).await?;
    test_incremental_generation("", c).await?;
    // Finally, test an illegal URL
    c.goto("http://localhost:8080/incremental_generation/tests")
        .await?;
    // This is actually very important: incremental pages that are invalidated on
    // the engine-side will appear valid to the browser, leading to a
    // `FullRouteVerdict::Found` variant. It is imperative that the `not_found`
    // checkpoint is executed somehow even in this case, otherwise
    // users are likely to find themselves with almost undiagnosable errors.
    wait_for_checkpoint!("not_found", 0, c);
    // There should be an error page
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("not found"));

    Ok(())
}

// This page tests using both build and request state in one page
#[perseus::test]
async fn amalgamation(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/amalgamation").await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    // This page naively combines build and request states into a single message
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "The message is: 'Hello from the amalgamation! (Build says: 'Hello from the build process!', server says: 'Hello from the server!'.)'");

    Ok(())
}

// This page tests request state
#[perseus::test]
async fn request_state(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/request_state").await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    // Unfortunately, we can't easily make the headless browser set the necessary
    // headers to allow Perseus to actually get the IP address
    assert!(text.contains("hidden from view"));

    Ok(())
}

// This page tests revalidation
#[perseus::test]
async fn revalidation(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/revalidation").await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    // We'll wait for five seconds, then reload the page and expect the content to
    // be different
    std::thread::sleep(std::time::Duration::from_secs(5));
    c.refresh().await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    let new_text = c.find(Locator::Css("p")).await?.text().await?;
    assert_ne!(text, new_text);

    Ok(())
}

// This tests revalidation and incremental generation simultaneously
#[perseus::test]
async fn revalidation_and_incremental_generation(
    c: &mut Client,
) -> Result<(), fantoccini::error::CmdError> {
    async fn test_time_page(page: &str, c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
        c.goto(&format!(
            "http://localhost:8080/revalidation_and_incremental_generation{}",
            page
        ))
        .await?;
        wait_for_checkpoint!("page_interactive", 0, c);
        let text = c.find(Locator::Css("p")).await?.text().await?;
        // We'll wait for five seconds, then reload the page and expect the content to
        // be different
        std::thread::sleep(std::time::Duration::from_secs(5));
        c.refresh().await?;
        wait_for_checkpoint!("page_interactive", 0, c);
        let new_text = c.find(Locator::Css("p")).await?.text().await?;
        assert_ne!(text, new_text);

        Ok(())
    }
    // First, test the build paths pages
    test_time_page("/test", c).await?;
    // Now test some incremental generation routes
    test_time_page("/isr", c).await?;
    test_time_page("", c).await?;

    Ok(())
}
