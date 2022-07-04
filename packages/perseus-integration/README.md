# THIS CRATE IS FOR TESTING PURPOSES ONLY!

It merely collates all the currently supported integrations and re-exposes their default servers through feature flags, enabling each of the examples to bring in just one dependency and then support all integrations through feature flags on this crate, which are specified by the CI testing framework.

In other words, this is an internal convenience package used for testing.

If you've come here trying to figure out how to use an actual integration instead of this internal package, you can easily replace the `perseus-integration` crate in your `Cargo.toml` with any of the supported server integrations, like `perseus-warp`. Then, just use `perseus_warp::dflt_server` instead of `perseus_integration::dflt_server` in your `#[perseus::main(...)]` declaration.
