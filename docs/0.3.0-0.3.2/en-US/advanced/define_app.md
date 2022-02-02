# The `define_app` Macro in Detail

Many users of Perseus will be perfectly content to leave the inner workings of their app to `define_app`, but some may be curious as to what this macro actually does, and that's what this section will explain.

Before we begin on the details of this, it's important to understand one thing: **your code does not make your app**. The Perseus engine (the stuff in `.perseus/`) makes your app, and it _imports_ your code to create the specifics of your app, but the actual Wasm entrypoint in in `.perseus/src/lib.rs`. This architecture can be a little unintuitive at first, but it allows Perseus to abstract a huge amount of work behind the scenes, minimizing the amount of boilerplate code that you need to write.

The issue that arises from this architecture is making your app interface with the engine, and that's where the `define_app!` macro comes in. It defines a number of functions that are then imported by the engine and called to get information about your app. The very first version of Perseus didn't even have a CLI, and all this interfacing had to be done manually! Today though, the process is _much_ easier.

Before we get on to exactly what the macro defines, it's worth mentioning that using Perseus without the `define_app!` macro is possible, but is not recommended, even for experiences users. The main reasons are twofold: you will be writing _a lot_ of boilerplate code (e.g. you have to define a dummy translations manager even if you're not using i18n) and your app may break with new minor versions, because Perseus considers changes to the engine and the internals of the `define_app!` macro to be non-breaking. If you're still determined to persevere with going macro-less, you should regularly review the Perseus [`CHANGELOG`](https://github.com/arctic-hen7/perseus/blob/main/CHANGELOG.md) to make any changes that are necessary for minor versions.

## Functions Defined

Now that we've got all that out of the way, let's really dig into the weeds of this thing! The `define_app!` macro is defined in `packages/perseus/src/macros.rs` in the repository, and that should be your reference while trying to understand the inner workings of it.

There are two versions of the macro, one that takes i18n options and one that doesn't. This is just syntactic sugar to make things more convenient for the user, and it doesn't affect anything more. Either way, here are the functions that are defined. (Note that a lot of these are defined with secondary internal macros.)

- `get_plugins` -- this returns an instance of `perseus::Plugins`, either an empty one if no plugins are provided, or whatever the user provides
- `APP_ROOT` (a `static` `&str`) -- this is the HTML `id` of the element to run Perseus in, which is `root` unless something else is provided by the user
- `get_immutable_store` -- this returns an instance of `perseus::stores::ImmutableStore` with either `./dist` or the user-provided distribution directory as the root (whatever is provided here is relative to `.perseus/`)
- `get_mutable_store` -- this returns an instance of `perseus::stores::FsMutableStore` with `./dist/mutable` as the root (relative to `.perseus`), or a user-given mutable store
- `get_translations_manager` -- see below
- `get_locales`
  - With i18n -- this returns an instance of `perseus::internal::i18n::Locales`, literally constructed with the given default locale, the other locales, and with `using_i18n` set to `true`
  - Without i18n -- this does the same as with i18n, but sets `using_i18n` to `false`, provides no `other` locales, and sets the default to `xx-XX` (the dummy locale expected throughout Perseus if the user isn't using i18n, anything else here if you're not using i18n will result in runtime errors!)
- `get_static_aliases` -- this creates a `HashMap` of your static aliases, from URL to resource location
- `get_templates_map` -- this creates a `HashMap` out of your templates, mapping the result of `template.get_path()` (what you provide to `Template::new()`) to the templates themselves (wrapped in `Rc`s to avoid literal lifetime hell)
- `get_templates_map_atomic` -- exactly the same as `get_templates_map`, but uses `Arc`s instead of `Rc`s (needed for multithreading on the server)
- `get_error_pages` -- this one's simple, it just returns the instance of `ErrorPages` that you provide to the macro

Most of these are pretty straightforward, they're just very boilerplate-heavy, which is why Perseus does them for you! However, the translations manager is a little less straightforward, because it does different things if Perseus has been deployed to a server (in which case the `standalone` feature will be enabled on Perseus).

### `get_translations_manager`

This function is `async`, and it returns something that implements `perseus::internal::i18n::TranslationsManager`. There are four cases of what the user can provide to the macro, and they'll be gone through individually.

#### No i18n

We provide a `perseus::internal::i18n::DummyTranslationsManager`, which is designed for this exact purpose. Perseus always needs a translations manager, so this one provides an API interface and no actual functionality.

#### A custom translations manager

We just return whatever the user provided. This is technically two cases, because i18n could be either enabled or disabled (though why someone would provide a custom dummy translations manager is a bit of a mystery).

#### I18n

If no custom translations manager is provided, we create a `perseus::internal::i18n::FsTranslationsManager` for them, the `::new()` method for which takes three arguments: a directory to expect translation files in, a vector of the locales to cache, and the file extension of translation files (which will always be named as `[locale].[extension]`).

The first argument is a little challenging, because it will usually be `../translations/` (relative to `.perseus/`), in the root directory of your project. However, if Perseus has been deployed as a standalone server binary, this directory will be in the same folder as the binary, so we use `./translations/` instead. In the macro, this is controlled by the `standalone` feature flag, but that isn't provided to your app, so the best thing to do here is up to you (you might depend on an environment variable that you remember to provide when you deploy).

The second argument is probably a little weird to you. Caching translations? Well, they're actually the most requested things for the Perseus server, so the `FsTranslationsManager` caches locales when it's started by default. This uses more memory on the server, but makes requests faster in the longer-term (we do the same thing with your `index.html` file). By default, Perseus runs the `.get_all()` function on the instance of `Locales` generated by the macro's own `get_locales()` function to get all your locales, and then it tells the manager to cache everything. This is customizable in the macro by allowing the user to provide a custom instance of `FsTranslationsManager`.

The final argument is blissfully simple, because it's defined internally in Perseus at `perseus::internal::i18n::TRANSLATOR_FILE_EXT`. The reason this isn't hardcoded is because it's dependent on the `Translator` being used, which is controlled by feature flags.

Finally, the reason this whole `get_translations_manager()` function is `async` is because it has to `await` that `FsTranslationsManager::new()` call, because translations managers are fully `async` (in case they need to be working with DBs or the like).

## Conclusion

If, after all that, you still want to use Perseus without the `define_app!` macro, there's an example on its way! That said, it is _much_ easier to leave things to the macro, or you'll end up writing a huge amount of boilerplate. In fact, all this is just the tip of the iceberg, and there's more transformation that's done on all this in the engine!
