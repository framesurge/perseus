# THIS CRATE IS FOR TESTING PURPOSES ONLY!

It merely collates all the currently supported integrations and re-exposes their default servers through feature flags, enabling each of the examples to bring in just one dependency and then support all integrations through feature flags on this crate, which are specified by the CI testing framework.

In other words, this is an internal convenience package used for testing.

If you've come here wondering about the weird syntax in `Cargo.toml` for one of the examples, each example in Perseus needs to be tested with each integration, so we alias `perseus-warp` to this crate in development, which allows CI to control the integration used for testing through feature flags. As should be noted in every one of these files (if you've found one without this note, please file an issue!), you should replace these lines with a usual dependency on `perseus-warp` (or whatever other integration you're using) if you want to run one of the examples outside the context of the Perseus repository (the only place the `perseus-integration` stuff is valid). We alias to `perseus-warp` so that the Rust code looks normal, which makes the code examples intelligible (which prevents people filing issues thinking all the examples are wrong, which happened for a good few months, fully justifiably).
