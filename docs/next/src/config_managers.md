# Config Managers

As you may have noticed, Perseus generates a considerable volume of static files to speed up serving your app and to cache various data. Storing these is trivial in development, they can just be put in a `dist` folder or the like. However, when in production, we don't always have access to such luxuries as a stateful filesystem, and we may need to access files from elsewhere, like a database or a CDN.

## Default

In development, you'll still need to specify a config manager, which allows you to test out your production config manager even in local development! The easiest one to use for typical development though is the inbuilt `FsConfigManager`.

## Writing a Config Manager

Any custom config managers have to implement the `ConfigManager` trait, which only has two functions: `read` and `write`. Here's the trait definition:

```rust,no_run,no_playground
pub trait ConfigManager {
    /// Reads data from the named asset.
    async fn read(&self, name: &str) -> Result<String>;
    /// Writes data to the named asset. This will create a new asset if one doesn't exist already.
    async fn write(&self, name: &str, content: &str) -> Result<()>;
}
```

### Errors

It's easily possible for CDNs of filesystems to throw errors when we try to interact with them, and Perseus provides a custom set of errors with [`error_chain!`]() to deal with this. Note that your implementation *must* use these, or it will not implement the trait and thus not be compatible with Perseus. The errors available to you are:

- `NotFound`, takes a `String` asset name
- `ReadFailed`, takes a `String` asset name and a `String` error (not chained because it might come back by carrier pigeon for all we know)
- `WriteFailed`, takes a `String` asset name and a `String` error (not chained because it might come back by carrier pigeon for all we know)

## Best Practices

Some storage solutions will be significantly faster in production than others, and a CDN is recommended over a database or the like. Generally speaking, go with something lightning-fast, or a local filesystem if you can. Unfortunately, Perseus must run its own logic to know what files to fetch, so direct communication between the app shell and the CDN is not possible, so speed of connection with your storage provider is essential.

We're currently working on framework support on providers like Netlify so this process is fully seamless for you, but for now it will be very inconvenient. Hence, **Perseus is presently not recommended for production use, but soon will be**! Once integrations are up and running, the project has matured some more, and setup on platforms like Vercel is done, have at it!
