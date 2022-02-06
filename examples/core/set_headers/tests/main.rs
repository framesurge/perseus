use fantoccini::Client;

// We use the usual test harness to ensure that we can assert on the site's contents, but we'll actually use `ureq` to verify that the header we want is being set
#[perseus::test]
async fn main(_: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    let res = ureq::get("http://localhost:8080").call().unwrap();
    let header_val = res.header("x-greeting");
    assert_eq!(header_val, Some("Hello World!"));

    Ok(())
}
