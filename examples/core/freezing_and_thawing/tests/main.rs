use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));
    wait_for_checkpoint!("initial_state_present", 0, c);
    wait_for_checkpoint!("page_interactive", 0, c);

    // Check the initials
    let mut page_state = c.find(Locator::Id("page_state")).await?;
    assert_eq!(page_state.text().await?, "Greetings, !");
    let mut global_state = c.find(Locator::Id("global_state")).await?;
    assert_eq!(global_state.text().await?, "Hello World!");

    // Change the page and global states
    c.find(Locator::Id("set_page_state"))
        .await?
        .send_keys("Test User")
        .await?;
    assert_eq!(page_state.text().await?, "Greetings, Test User!");
    c.find(Locator::Id("set_global_state"))
        .await?
        .send_keys(" Extra text.")
        .await?;
    assert_eq!(global_state.text().await?, "Hello World! Extra text.");

    // Switch to the about page to demonstrate route restoration as well
    c.find(Locator::Id("about-link")).await?.click().await?;
    c.current_url().await?;
    wait_for_checkpoint!("page_interactive", 1, c);
    // Now press the freeze button and get the frozen app
    c.find(Locator::Id("freeze_button")).await?.click().await?;
    let frozen_app = c.find(Locator::Id("frozen_app")).await?.text().await?;

    // Reload the app so that we can use the thawed state from scratch
    // This is a full reload, so the checkpoint counter will start over from scratch
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));
    wait_for_checkpoint!("initial_state_present", 0, c);
    wait_for_checkpoint!("page_interactive", 0, c);
    // Check that the empty initials are restored
    assert_eq!(
        c.find(Locator::Id("page_state")).await?.text().await?,
        "Greetings, !"
    );
    assert_eq!(
        c.find(Locator::Id("global_state")).await?.text().await?,
        "Hello World!"
    );

    // Fill in the frozen app to thaw from it and press the thaw button
    c.find(Locator::Id("thaw_input"))
        .await?
        .send_keys(&frozen_app)
        .await?;
    c.find(Locator::Id("thaw_button")).await?.click().await?;
    // We should now be back on the about page, with the global state restored there
    assert!(c
        .current_url()
        .await?
        .as_ref()
        .starts_with("http://localhost:8080/about"));
    assert_eq!(
        c.find(Locator::Id("global_state")).await?.text().await?,
        "Hello World! Extra text."
    );

    // And go back to the index page to check everything fully
    c.find(Locator::Id("index-link")).await?.click().await?;
    c.current_url().await?;
    wait_for_checkpoint!("page_interactive", 1, c);
    // Verify that everything has been correctly restored
    assert_eq!(
        c.find(Locator::Id("page_state")).await?.text().await?,
        "Greetings, Test User!"
    );
    assert_eq!(
        c.find(Locator::Id("global_state")).await?.text().await?,
        "Hello World! Extra text."
    );

    Ok(())
}
