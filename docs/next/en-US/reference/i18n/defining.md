# Defining Translations

The first part of setting up i18n in Perseus is to state that your app uses it, which is done in `PerseusApp` like so (taken from [the i18n example](https://github.com/arctic-hen7/perseus/tree/main/examples/core/i18n)):

```rust
{{#include ../../../../examples/core/i18n/src/lib.rs}}
```

There are two paremeters to the `.locales()` function: the default locale for your app, and then any other locales it supports (other than the default). Each of these locales should be specified in the form `xx-XX`, where `xx` is the language code (e.g. `en` for English, `fr` for French, `la` for Latin) and `XX` is the region code (e.g. `US` for United States, `GB` for Great Britain, `CN` for China).

## Routing

After you've enabled i18n like so, every page on your app will be rendered behind a locale. For example, `/about` will become `/en-US/about`, `/fr-FR/about`, and`/es-ES/about` in the above example. These are automatically rendered by Perseus at build-time, and they behave exactly the same as every other feature of Perseus.

Of course, it's hardly optimal to direct users to a pre-translated page if they may prefer it in another language, which is why Perseus supports _locale detection_ automatically. In other words, you can direct users to `/about`, and they'll automatically be redirected to `/<locale>/about`, where `<locale>` is their preferred locale according to `navigator.languages`. This matching is done based on [RFC 4647](https://www.rfc-editor.org/rfc/rfc4647.txt), which defines how locale detection should be done.

## Adding Translations

After you've added those definitions to `PerseusApp`, if you try to run your app, you'll find that ever page throws an error because it can't find any of the translations files. These must be defined under `translations/` (which should be NEXT to `/src`, not in it!), though this can be customized (explained later). They must also adhere to the naming format `xx-XX.ftl` (e.g. `en-US.ftl`). `.ftl` is the file extension that [Fluent](https://projectfluent.org) files use, which is the default translations system of Perseus. If you'd like to use a different system, this will be explained later.

Here's an example of a translations file (taken from [here](https://github.com/arctic-hen7/perseus/blob/main/examples/core/i18n/translations/en-US.ftl)):

```fluent
{{#include ../../../../examples/core/i18n/translations/en-US.ftl}}
```

You can read more about Fluent's syntax [here](https://projectfluent.org) (it's _very_ powerful).
