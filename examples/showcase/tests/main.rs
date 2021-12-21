use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn index(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/en-US").await?;
    wait_for_checkpoint!("initial_state_present", 0, c);
    // This greeting is passed in as a build state prop
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "Hello World!");

    Ok(())
}

// We don't bother testing `/about` because everything there is hardcoded

// This page tests that we can define templates with nested root path domains
#[perseus::test]
async fn new_post(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/en-US/post/new").await?;
    wait_for_checkpoint!("initial_state_present", 0, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "New post creator.");

    Ok(())
}

// This tests build state, build paths, and incremental generation simultaneously
#[perseus::test]
async fn post(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    async fn test_post_page(page: &str, c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
        c.goto(&format!("http://localhost:8080/en-US/post{}", page))
            .await?;
        wait_for_checkpoint!("initial_state_present", 0, c);
        // There should be a heading with the slug
        let text = c.find(Locator::Css("h1")).await?.text().await?;
        assert_eq!(text, format!("post{}", page));

        Ok(())
    }
    // First, test the build paths pages
    test_post_page("/test", c).await?;
    test_post_page("/blah/test/blah", c).await?;
    // Now test some incremental generation routes
    test_post_page("/this/is/a/route/that/wasnt/prerendered/it/was/generated/on/the/server/dynamically/at/request/time", c).await?;
    test_post_page("", c).await?;
    // Finally, test an illegal URL
    c.goto("http://localhost:8080/en-US/post/tests").await?;
    wait_for_checkpoint!("initial_state_error", 0, c);
    // There should be an error page
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("not found"));

    Ok(())
}

// This page tests using both build and request state in one page
#[perseus::test]
async fn amalgamation(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/en-US/amalgamation").await?;
    wait_for_checkpoint!("initial_state_present", 0, c);
    // This page naively combines build and request states into a single message
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert_eq!(text, "The message is: 'Hello from the amalgamation! (Build says: 'Hello from the build process!', server says: 'Hello from the server!'.)'");

    Ok(())
}

// This page tests request state
#[perseus::test]
async fn ip(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/en-US/ip").await?;
    wait_for_checkpoint!("initial_state_present", 0, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    // Unfortunately, we can't easily make the headless browser set the necessary headers to allow Perseus to actually get the IP address
    assert!(text.contains("hidden from view"));

    Ok(())
}

// This page tests revalidation
#[perseus::test]
async fn time_root(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/en-US/time").await?;
    wait_for_checkpoint!("initial_state_present", 0, c);
    let text = c.find(Locator::Css("p")).await?.text().await?;
    // We'll wait for five seconds, then reload the page and expect the content to be different
    std::thread::sleep(std::time::Duration::from_secs(5));
    c.refresh().await?;
    wait_for_checkpoint!("router_entry", 0, c);
    let new_text = c.find(Locator::Css("p")).await?.text().await?;
    assert_ne!(text, new_text);

    Ok(())
}

// This tests revalidation and incremental generation simultaneously
#[perseus::test]
async fn time(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    async fn test_time_page(page: &str, c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
        c.goto(&format!("http://localhost:8080/en-US/timeisr{}", page))
            .await?;
        wait_for_checkpoint!("initial_state_present", 0, c);
        let text = c.find(Locator::Css("p")).await?.text().await?;
        // We'll wait for five seconds, then reload the page and expect the content to be different
        std::thread::sleep(std::time::Duration::from_secs(5));
        c.refresh().await?;
        wait_for_checkpoint!("router_entry", 0, c);
        let new_text = c.find(Locator::Css("p")).await?.text().await?;
        assert_ne!(text, new_text);

        Ok(())
    }
    // First, test the build paths pages
    test_time_page("/test", c).await?;
    // Now test some incremental generation routes
    test_time_page("/isr", c).await?;
    test_time_page("", c).await?;
    // Finally, test an illegal URL
    c.goto("http://localhost:8080/en-US/timeisr/tests").await?;
    wait_for_checkpoint!("initial_state_error", 0, c);
    // There should be an error page
    let text = c.find(Locator::Css("p")).await?.text().await?;
    assert!(text.contains("not found"));

    Ok(())
}
