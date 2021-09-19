# Templates

The most central part of Perseus is the definition of templates, which control how pages are built.

## Usage

An extremely simple template would look like this:

```rust
use perseus::Template;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(AboutPage<G>)]
pub fn about_page() -> SycamoreTemplate<G> {
    template! {
        p { "About." }
    }
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|_| {
        template! {
            AboutPage()
        }
    })
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("about").template(template_fn())
}
```

First, we define the *component function*, which is done with Sycamore. This is the page itself, and it contains its actual markup, in Sycamore's templating syntax (you can read more about that [here]()). Next is the *template function*, which simply defines a function that will actually render the page. We break this simple closure out into a function to get it because we'll reuse it in the routing process later. If your page takes a state, it will be passed to this closure **as a string**. You must then deserialize it, and it is safe to `.unwrap()` here (barring a horrific logic failure). The final function we define is the *page function*, which just creates the actual template for the page.

## Template Definition

You can define a template with the `Template::new()` method, which takes the template's path as an argument (with no leading or trailing slashes). In the above example, `about` renders only one page, which would be hosted at `/about`.

The only mandatory builder function after that is `.template()`, which defines your template function (the closure inside `template_fn()` in the above example). There are a number of other functions available to customize how the template renders, all of which are documented [here](./strategies/intro.md).
