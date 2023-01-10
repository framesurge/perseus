# `PerseusApp`

When you're developing with Perseus, you'll need to tell it about your templates so it can prerender them and udnerstand that they...well, exist. The way you do this is through [`PerseusApp`](=prelude/struct.PerseusAppBase@perseus), which is the central interface between the Perseus internals and your own code. Most of the time, you'll use this `struct` for defining your templates, capsules, and error views, but there are several other things it's capable of, which we'll go through here.

If you're looking for documentation about the Perseus entrypoint system, see [here](:first-app/defining).

## Template/capsule definition patterns

When you define your templates and capsules, there are two ways to do this. The first is the one you'll see most frequently with templates, which involves creating a `get_template()` function or similar that just returns the template you want, which you can then call in the `.template()` method on your `PerseusApp`. However, this doesn't apply so well to capsules, which, unlike templates, you need access to in your own code as well. In those cases, it's best to use static references, usually created with a macro like [`lazy_static!`]. However, because these will be `&'static Capsule`s, not `Capsule`s proper, you'll need to use the `.capsule_ref()` method for these, like so:

```rust
.capsule_ref(&*crate::capsules::my_capsule::MY_CAPSULE)
```

The first pattern is called the *functional definition pattern*, and the second is the *referential definition pattern*. Usually, you'll use the former with templates and the latter with capsules, although some developers like to have the consistency of using one pattern everywhere: this is usually the referential definition pattern, because it offers far better ergonomics with capsules. Perseus is indifferent to which you use, and there is no penalty for either, it's entirely up to your personal preference.

## Index views

Very often, there are a few things you want to apply throughout your app, like universal stylesheets, certain preloading directives, or certain `<meta>` tags to improve search engine optimization. In these cases, you'll want to take advantage of the index view system, which allows you to write the root view into which your app will be interpolated in Sycamore! Importantly though, nothing in this view can be reactive, since it will be rendered to HTML immediately and treated as a string thereafter. If you want to provide an actual HTML string, you can use `.index_view_str()` instead.

Here's an example of an app definition using an index view:

```
#{#include ../../../examples/core/index_view/src/main.rs}
```

Note the use of the `PerseusRoot` component, which is provided to denote the entrypoint for Perseus. Unfortunately, due to limitations in the internal rendering infrastructure, you have to use this component, as opposed to manually defining the `<div id="root"></div>` that Perseus automatically interpolates into.

## Mutable stores

If you've taken a look at the API docs for `PerseusApp`, you may have noticed that you're actually looking at `PerseusAppBase`, which is subtly different: it takes *three* generics, not just `G`, but also `M` and `T`. The `M` is for a [`MutableStore`](=stores/trait.MutableStore@perseus), which is a special internal component of Perseus.

If you take a look in the `dist/` directory that Perseus produces, you'll find all sorts of interesting things, but particularly the `static/` and `mutable/` directories. In the former is a series of HTML and JSON fragments that Perseus writes to the disk at build-time to store forever, like the prerendered HTML for pages that are static. However, some pages have contents and/or state that might change in future, so Perseus writes them to the `mutable/static`directory. Pretty intuitively, the `static/` directory is *immutable*, meaning it can't be changed after build-time, and the `mutable/` directory is *mutable*, meaning it canb be. The [`ImmutableStore`](=stores/struct.ImmutableStore@perseus) type governs the immutable store, while the [`MutableStore`](=stores/trait.MutableStore@perseus) trait governs the mutable one. The distinction between the two may seem unimportant, but it's actually critical when you look into things like serverless functions, which typically have an immutable filesystem. Perseus is designed from the ground up to maximize performance with immutable assets by storing them locally, while offering an extensible system for managing mutable assets (e.g. in a serverless environment, you would have to use something like a colocated database). Hence, `PerseusAppBase` is generic over this. By default, though, [`FsMutableStore`](stores/struct.FsMutableStore@perseus) is used, which simply uses the filesystem. In development, this is the fastest, and it's the best option for production too, if you're running in an environment that supports writing to the filesystem.

Note that exported apps don't need to care about the mutable store, since they are totally static after build-time.

## Translations management

Similarly, that `T` allows `PerseusAppBase` to be generic over a [`TranslationsManager`](=i18n/trait.TranslationsManager@perseus), which allows you to manage your translations in an arbitrary way for apps using internationalization. Usually, you'll just use the [`FsTranslationsManager`](=i18n/struct.FsTranslationsManager@perseus), which is the default, expecting your translations to be in a `translations/` directory at the root of your project. However, this can of course be changed, which may be useful for larger apps that expect their translations to come from a larger database.

Note that translations managers are expected to manage their own caching, so keep this in mind when building your own, since, in an internationalized app, translations will be the second-most-requested resource after pages (since they must be served with every initial load, and every locale switch).

## The state store

One helpful method on `PerseusApp` that may help you is the `.pss_max_size()` function, which allows you to set an arbitrary maximum number of pages that Perseus will cache. This is separate to the browser's caching system, and allows Perseus to store the absolute minimum amount of information needed to re-render a page if the user goes back to it. If you try, for instance, in these docs, going to a different page, and then coming back to this one, you'll see that the transition back is instant. The reason for this is Perseus' caching mechanism. If you're familiar with SPA routing, consider this SPA caching (which, to our knowledge, is a unique feature at the moment).

Deciding what to set this value to should be based on empirical testing of your app's performance: higher values will allow users to get a higher number of instant transitions, but at the cost of your app using more RAM, whereas lower versions will keep your app's memory profile lean, at the cost of longer wait times between old pages. The default value is `25` at the moment, which is subject to significant change (as it was effectively concocted out of a hat).

Importantly, apps with very large state (e.g. documentation sites!) may wish to set this to a lower value, lest 25 very large state objects be cached automatically. If you notice your app or browser becoming a little laggy, or your app becoming completely non-responsive, lower this value. Once the maximum number of pages has been reached, Perseus will automatically *evict* the oldest pages first to make room for new ones.

Note that capsules are cached entirely separately, and they are kept around until the last page that depends upon them is evicted. Therefore, apps whose pages use a large volume of capsules should set the PSS maximum size to a lower value. See the [`PageStateStore`](=state/struct.PageStateStore@perseus) API for further details on more fine-grained control of this part of Perseus.

*Note: an experimental system will be released soon that will allow the state store to poll its own real size in RAM and intelligently delete pages until it fits in a certain size, which will make deciding on the maximum size far easier and more logical.*

## Other methods

You can see all the methods of `PerseusApp` [here](=prelude/struct.PerseusAppBase@perseus), along with their individual API docs.
