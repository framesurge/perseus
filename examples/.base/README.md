# Base Example

This isn't an example per se, rather it's a template for future examples. If you want to create a new example, you should copy this directory and modify it to suit your needs, its purpsoe is just to create a universal minimal boilerplate from which examples can work.

After copying this, you'll need to change this README to describe your example, and you'll need to change the name of your example in `Cargo.toml` to `perseus-example-<name>` (right now, it's `perseus-example-base`, leaving it as this will cause a compilation error).

*Note: by default, all examples are compatible with all integrations, and will be tested with them all. If your example is only compatible with a single integration, you shouldn't use `perseus-integration`, but the specific integration crate instead, and make sure to add an empty `.integration_locked` file to the root of the example. See `examples/core/custom_server/` for more details.*

If you need some help with creating your example, feel free to pop over to our [Discord channel](https://discord.com/invite/GNqWYWNTdp)!
