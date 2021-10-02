# Build State

This strategy allows you to define the state for a page. The function you give will be provided with the path of the page being rendered from the template on which this strategy is defined.

If used without the *build paths* strategy, this will simply render one state for the only page that the template will render. If used with it, this strategy will be invoked for every page that the template renders.

Note also that this strategy will be invoked on every on-demand build if used with the *incremental* strategy.

## Usage

You can define a function for this strategy like so:

```rust
use serde::{Serialize, Deserialize};
use perseus::ErrorCause;

#[derive(Serialize, Deserialize)]
pub struct PostPageProps {
    title: String,
    content: String,
}
// ...
pub async fn get_build_state(path: String) -> Result<String, (String, ErrorCause)> {
   let title = urlencoding::decode(&path).unwrap();
    let content = format!(
        "This is a post entitled '{}'. Its original slug was '{}'.",
        title, path
    );

    Ok(serde_json::to_string(&PostPageProps {
        title: title.to_string(),
        content,
    })
    .unwrap())
}
```

This function can produce two kinds of errors, broadly: those caused by the server, and those caused by the client (if this is called for a page that doesn't exist from the *incremental generation* strategy). For that reason, you need to return a `(String, ErrorCause)` tuple, the second part of which specifies who's responsible for the error. This allows Perseus to figure out whether it should send a 400 (client error) or 500 (server error) HTTP status code in the event of an error. While returning `String` errors may seem annoying, it prevents unnecessary internal heap allocation, and does overall make things faster (if you have a better way, please [open a PR](https://github.com/arctic-hen7/perseus/pulls)!). This function must also be asynchronous and the state must be returned in a stringified format.

The path provided to the function will be provided as **whatever will end up being rendered**. For example, if you returned the element `test` from the build paths strategy (intending it to be rendered as `/post/test`), it will be passed to this function as `post/test`.

You can add this strategy to a template like so:

```rust,no_run,no_playground
template
	// ...
    .build_state_fn(Box::new(get_build_state))
```
