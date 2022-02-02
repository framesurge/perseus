# Translations Managers

As mentioned earlier, Perseus expects your translations to be in the very specific location of `translations/<locale>.ftl`, which may not be feasible or preferable in all cases. In fact, there may indeed be cases where translations might be stored in an external database (not recommended for performance as translations are regularly requested, filesystem storage with caching is far faster).

If you'd like to change this default behavior, this section is for you! Perseus manages the locations of translations with a `TranslationsManager`, which defines a number of methods for accessing translations, and should implement caching internally. Perseus has two inbuilt managers: `FsTranslationsManager` and `DummyTranslationsManager`. The former is used by default, and the latter if i18n is disabled.

## Using a Custom Translations Manager

The `define_app!` macro accepts a property called `translations_manager` if you define `locales`, which can be used to specify a non-default translations manager.

## Using a Custom Directory

If you just want to change the directory in which translations are stored, you can still use `FsTranslationsmanager`, just initialize it with a different directory, and make sure to set up caching properly. See [here](https://github.com/arctic-hen7/perseus/blob/f7f7892fbf124a7d887b1f22a1641c79773d6246/packages/perseus/src/macros.rs#L35-L50) for how this is done internally.

## Building a Custom Translations Manager

This is more complex, and you'll need to consult [this file](https://github.com/arctic-hen7/perseus/blob/main/packages/perseus/src/translations_manager.rs) (note: the client translations manager is very different) in the Perseus source code for guidance. If you're stuck, don't hesitate to ask a question under [discussions](https://github.com/arctic-hen7/perseus/discussions/new) on GitHub!
