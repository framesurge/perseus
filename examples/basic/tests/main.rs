use fantoccini::{Client, Locator};

#[perseus::test]
async fn foo(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto("http://localhost:8080").await?;
    c.wait()
        .for_element(Locator::Id("__perseus_checkpoint-shell_entry-0"))
        .await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "http://localhost:8080/");
    c.find(Locator::Id("about-link")).await?.click().await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "http://localhost:8080/about");

    Ok(())
}
