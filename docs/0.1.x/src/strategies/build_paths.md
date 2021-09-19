# Build Paths

This strategy allows you to define the pages a template will render. For example, you might use this for a blog posts system to get all the posts from a database and return a `Vec<String>` of them. This strategy is roughly equivalent to NextJS's `get_static_paths` function.

Every element returned here will have the opportunity to create its state with the *build state* strategy, being passed the path defined here. Note that every element returned here will be built, so if you need to return more than about 10 elements, it's a better idea to only return the most used ones and leave the rest to the *incremental generation* strategy to reduce your build time.

## Usage

You can define a function for this strategy like so:

```rust
pub async fn get_build_paths() -> Result<Vec<String>, String> {
    Ok(vec![
        "test".to_string(),
        "blah/test/blah".to_string()
    ])
}
```

Paths returned from this function will be rendered under `[template-path]/[returned-path]`, and they should not have a leading or trailing `/`. If you want to return a nested path, simply do so (but make sure to handle it properly in your router). Note that any errors must be returned as `String`s, and the function must be asynchronous.

You can add this strategy to a template like so:

```rust,no_run,no_playground
template
	// ...
	.build_paths_fn(Box::new(get_static_paths))
```
