# Translations Managers

As mentioned earlier, Perseus expects your translations to be in the very specific location of `translations/<locale>.ftl`, which may not be feasible or preferable in all cases. In fact, there may indeed be cases where translations might be stored in an external database (not recommended for performance as translations are regularly requested, filesystem storage with caching is far faster).

If you'd like to change this default behavior, this section is for you! Perseus manages the locations of translations with a `TranslationsManager`, which defines a number of methods for accessing translations, and should implement caching internally. Perseus has two inbuilt managers: `FsTranslationsManager` and `DummyTranslationsManager`. The former is used by default, and the latter if i18n is disabled.

## Using a Custom Translations Manager

`PerseusApp` can be used with a custom translations manager through the `.translations_manager()` function. Note that this must be used with `PerseusAppWithTranslationsManager` rather than the usual `PerseusApp` (there's also `PerseusAppBase` if you want this and a custom mutable store). Further, translations managers all instantiate asynchronously, but we can't have asynchronous code in `PerseusApp` because of how it's called in the browser, so you should provide a future here (just don't add the `.await`), and Perseus will evaluate this when needed.

## Using a Custom Directory

If you just want to change the directory in which translations are stored, you can still use `FsTranslationsmanager`, just initialize it with a different directory, and make sure to set up caching properly. 

## Building a Custom Translations Manager

This is more complex, and you'll need to consult [this file](https://github.com/framesurge/perseus/blob/main/packages/perseus/src/i18n/translations_manager.rs) (note: the client translations manager is very different) in the Perseus source code for guidance. If you're stuck, don't hesitate to ask a question under [discussions](https://github.com/framesurge/perseus/discussions/new) on GitHub!
