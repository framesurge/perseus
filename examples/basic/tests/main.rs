use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn foo(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("begin", 0, c);
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "http://localhost:8080/");
    c.find(Locator::Id("about-link")).await?.click().await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "http://localhost:8080/about");

    Ok(())
}
