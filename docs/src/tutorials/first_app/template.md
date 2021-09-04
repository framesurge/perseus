# Writing Your First Template

Now it's time to write your first template! Templates in Perseus are the things that create pages (e.g. you might have an about page template and a blog post template). You can read more about them [here](https://arctic-hen7.github.io/perseus/arch.html#templates-and-pages).

*Note: templates used to be called pages in Perseus, so some of the older examples may still use that term. Don't worry about it, just be aware of the inconsistency! We're cleaning it up gradually!*

## Setting up Templates

1. Create a new folder under `src/` called `templates`. This is where all your templates will be housed.
2. Add a file that declares that as a module (that's how Cargo handles subfolders) called `mod.rs` under `src/templates/`.
3. Declare the module by adding `mod templates;` to the top of `src/lib.rs` (which Cargo should've created for you).

## Writing an Index Page


Let's write the landing page of your app! Create a new file under `src/templates/` called `index.rs`, and then declare it by adding `pub mod index;` to the top of `src/templates/mod.rs` (`pub` because it should be publicly accessible by `lib.rs`).

Now add the following to that file:

```rust
use perseus::Template;
use std::sync::Arc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(IndexPage<G>)]
pub fn index_page() -> SycamoreTemplate<G> {
    template! {
        p { "Welcome to the app!" }
    }
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Arc::new(|_| {
        template! {
            IndexPage()
        }
    })
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index").template(template_fn())
}
```

This code needs a lot of explaining, so here goes!

1. Import everything we need. That's the type for a Perseus template, an [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) (magical Rust type wizardry if you're a beginner!), and some stuff from [Sycamore](https://github.com/sycamore-rs/sycamore) that lets us make an interface.
2. Define the component that will render your final page with the `index_page` function (annotated so that it becomes `IndexPage` with Sycamore's internal magic). Notice that this outputs a `SycamoreTemplate`, which is basically a Rust way of writing HTML. The syntax is a little weird, but you'll get used to it pretty quickly (and there's [some work](https://github.com/sycamore-rs/sycamore/issues/23) happening to possibly make it more HTML-like in future). All this little `template!` does is render a `p` HTML element (paragraph) and will it with the text `Welcome to the app!`.
3. Define the template function in `template_fn`. This is probably the weirdest part of Perseus, but it'll make more sense with more complex pages. Basically, the template function will be given any props your template takes (this template doesn't take any though, hence the `_`), and then it lets you render your template with them. **Props are what turn a generic template into a unique page.**
4. Finally define a utility function for using this called `get_template`. This actually defines the Perseus template, providing the root path at which it will be rendered as a page (`/index`, without the leading forward slash written). That needs to know about the template function, which we provide with `.template()`. Later, this is where you get to add cool rendering strategies like [revalidation](https://arctic-hen7.github.io/perseus/strategies/revalidation.html), [SSR](https://arctic-hen7.github.io/perseus/strategies/request_state.html), and [incremental generation](https://arctic-hen7.github.io/perseus/strategies/incremental.html)!

If you understand all that, then you understand how to write a basic template in Perseus! Well done! If not, don't worry, but you might want to go over the above explanation a few times until you're understanding the basics. If you're feeling clueless, reach out on the [Gitter support channel](https://gitter.im/perseus-framework/support) for help!
