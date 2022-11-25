use fantoccini::{Client, Locator};
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn main(c: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    async fn test_build_path(
        page: &str,
        c: &mut Client,
    ) -> Result<(), fantoccini::error::CmdError> {
        c.goto(&format!("http://localhost:8080{}", page))
            .await?;
        wait_for_checkpoint!("initial_state_present", 0, c);
        // There should be a heading with the slug
        let heading = c.find(Locator::Css("h1")).await?.text().await?;
        assert!(heading.contains(&format!("{}", page)));
        // The helper state should be the same on every page
        let text = c.find(Locator::Css("p")).await?.text().await?;
        assert!(text.contains("Extra state: extra helper state!"));

        Ok(())
    }
    test_build_path("", c).await?;
    test_build_path("/test", c).await?;
    test_build_path("/blah/test/blah", c).await?;

    Ok(())
}
