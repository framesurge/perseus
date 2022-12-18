use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));

    // The greeting was passed through using build state
    wait_for_checkpoint!("initial_state_present", 0, c);
    wait_for_checkpoint!("page_interactive", 0, c);
    let mut greeting = c.find(Locator::Css("p")).await?;
    assert_eq!(greeting.text().await?, "Hello World!");

    let change_msg_button = c.find(Locator::Id("change-message")).await?;
    change_msg_button.click().await?;
    assert_eq!(greeting.text().await?, "Message from JS!");

    Ok(())
}
