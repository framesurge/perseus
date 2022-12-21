use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert!(url.as_ref().starts_with("http://localhost:8080"));

    wait_for_checkpoint!("page_interactive", 0, c);
    let panic_msg = c.find(Locator::Css("#__perseus_popup_error > p")).await?.text().await?;
    assert!(panic_msg.contains("critical internal error"));

    // Try out a 404
    c.goto("http://localhost:8080/foo").await?;
    wait_for_checkpoint!("not_found", 0, c);
    let err_msg = c.find(Locator::Css("#root > p")).await?.text().await?;
    assert_eq!(err_msg, "Sorry, that page doesn't seem to exist.");

    // For some reason, retrieving the inner HTML or text of a `<title>` doesn't
    // work
    let title = c.find(Locator::Css("title")).await?.html(false).await?;
    assert!(title.contains("Page not found"));


    Ok(())
}
