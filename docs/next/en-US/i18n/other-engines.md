# Other Translation Engines

Perseus uses [Fluent](https://projectfluent.org) for i18n by default, but this isn't set in stone. Rather than providing only one instance of `Translator`, Perseus can support many through Cargo's features system. By default, Perseus will enable the `translator-fluent` feature to build a `Translator` `struct` that uses Fluent. The `translator-dflt-fluent` feature will also be enabled, which sets `perseus::Translator` to be an alias for `FluentTranslator`.

If you want to create a translator for a different system, this will need to be integrated into Perseus as a pull request, but we're more than happy to help with these efforts. Optimally, Perseus will in future support multiple translations systems, and developers will be able to pick the one they like the most

## Why Not a Trait?

It may seem like this problem could simply be solved with a `Translator` trait, as is done with translations managers, but unfortunately this isn't so simple because of the way translators are transported through the app. The feature-gating solution was chosen as the best compromise between convenience and performance.

## How Do I Make One?

If you want to make your own alternative translation engine, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose) about it, explaining the system you want to support. Provided the system is compatible with Perseus' i18n design (which it certainly should be if we've done our job correctly!), we'll be happy to help you get it into Perseus!
