use fantoccini::{error::CmdError, Client, Locator};
use perseus::wait_for_checkpoint;

// Basic capsules, properties, and wrapping with property passthrough
#[perseus::test]
async fn index(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080/about").await?;
    wait_for_checkpoint!("page_interactive", 0, c);

    async fn test(c: &mut Client) -> Result<(), CmdError> {
        // Get the greeting widget (right text and right colour, which was set by
        // properties)
        let mut greeting = c.find(Locator::Id("greeting")).await?;
        assert_eq!(greeting.text().await?, "Hello world! (I'm a widget!)");
        assert_eq!(
            greeting.attr("style").await?,
            Some("color: red;".to_string())
        );

        Ok(())
    }

    // Subsequent...
    goto_page_from_links(c, "index-link").await?;
    test(c).await?;
    // ...and initial
    c.refresh().await?;
    test(c).await?;

    Ok(())
}

// Request state and rescheduling
#[perseus::test]
async fn about(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("page_interactive", 0, c);

    async fn test(c: &mut Client) -> Result<(), CmdError> {
        let ip = c.find(Locator::Id("ip")).await?.text().await?;
        // Headless browsers...
        assert_eq!(ip, "\"hidden from view!\"");

        Ok(())
    }

    // Subsequent...
    goto_page_from_links(c, "about-link").await?;
    test(c).await?;
    // ...and initial
    c.refresh().await?;
    test(c).await?;

    Ok(())
}

// Revalidation (and more advanced rescheduling)
#[perseus::test]
async fn clock(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("page_interactive", 0, c);

    async fn test(c: &mut Client) -> Result<(), CmdError> {
        let text = c.find(Locator::Id("time")).await?.text().await?;
        // We'll wait for five seconds, then reload the page and expect the content to
        // be different
        std::thread::sleep(std::time::Duration::from_secs(5));
        c.refresh().await?;
        wait_for_checkpoint!("page_interactive", 0, c);
        let new_text = c.find(Locator::Id("time")).await?.text().await?;
        assert_ne!(text, new_text);

        Ok(())
    }

    // Subsequent...
    goto_page_from_links(c, "clock-link").await?;
    test(c).await?;
    // ...and initial
    c.refresh().await?;
    test(c).await?;

    Ok(())
}

// Incremental generation with build paths
#[perseus::test]
async fn four(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("page_interactive", 0, c);

    async fn test(c: &mut Client) -> Result<(), CmdError> {
        let four = c.find(Locator::Id("four")).await?.text().await?;
        assert_eq!(four, "The number four: 4.");

        Ok(())
    }

    // Subsequent...
    goto_page_from_links(c, "four-link").await?;
    test(c).await?;
    // ...and initial
    c.refresh().await?;
    test(c).await?;

    Ok(())
}

// Incremental generation with non-build paths (testing automatic extra
// incremental builds and widget functions)
#[perseus::test]
async fn calc(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("page_interactive", 0, c);

    async fn test(c: &mut Client) -> Result<(), CmdError> {
        let fiftysix = c.find(Locator::Id("fifty-six")).await?.text().await?;
        assert_eq!(fiftysix, "The number fifty-six: 56.");

        let mut sum = c.find(Locator::Id("sum")).await?;
        assert_eq!(sum.text().await?, "The sum of the state numbers: 42.");
        // Update the sum (this reactively fetches the new widgets, and leads them to be
        // incrementally rendered)
        c.find(Locator::Css("input"))
            .await?
            .clear() // Starts with 0
            .await?;
        c.find(Locator::Css("input")).await?.send_keys("50").await?;
        // Because we're actually fetching a separate widget, we do have to wait a
        // second
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(sum.text().await?, "The sum of the state numbers: 92.");

        // Now try another number
        c.find(Locator::Css("input"))
            .await?
            .send_keys("0") // 500 now
            .await?;
        // Because we're actually fetching a separate widget, we do have to wait a
        // second
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(sum.text().await?, "The sum of the state numbers: 542.");

        // Now go back to 50, without any delay (because of preloading, this should be
        // instant)
        c.find(Locator::Css("input")).await?.clear().await?;
        c.find(Locator::Css("input")).await?.send_keys("50").await?;
        assert_eq!(sum.text().await?, "The sum of the state numbers: 92.");

        Ok(())
    }

    // Subsequent...
    goto_page_from_links(c, "calc-link").await?;
    test(c).await?;
    // ...and initial
    c.refresh().await?;
    wait_for_checkpoint!("page_interactive", 0, c);
    test(c).await?;

    Ok(())
}

async fn goto_page_from_links(c: &mut Client, id: &str) -> Result<(), fantoccini::error::CmdError> {
    let link = c.find(Locator::Css(&format!("#links > #{}", id))).await?;
    link.click().await?;

    Ok(())
}
