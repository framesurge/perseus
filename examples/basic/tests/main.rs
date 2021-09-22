use fantoccini::{ClientBuilder, Locator};

static WEBDRIVER: &str = "http://localhost:4444";

#[tokio::test]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut c = ClientBuilder::native().connect(WEBDRIVER).await.expect("failed to connect to WebDriver");

    c.goto("http://localhost:8080").await?;
    c.wait().for_element(Locator::Id("__perseus_checkpoint-perseus_shell_entry-0")).await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "http://localhost:8080/");
    c.find(Locator::Id("about-link")).await?.click().await?;
    c.wait().for_element(Locator::Id("__perseus_checkpoint-perseus_shell_entry-1")).await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "http://localhost:8080/about");

    c.close().await
}
