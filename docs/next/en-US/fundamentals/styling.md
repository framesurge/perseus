# Styling

In any kind of web development, you probably want your site to look good, and that will involve working with a language called *CSS*, short for *Cascading Style Sheets*. It's well beyond the scope of these docs to explain CSS, so we'll leave that to [this fantastic introduction](https://developer.mozilla.org/en-US/docs/Learn/Getting_started_with_the_web/CSS_basics) if you're new to it.

Right now, Perseus and Sycamore have limited inbuilt styling capabilities, and you're best off using either traditional styling (i.e. set a class `header-button` and style that in `header.css`, etc.), or a styling library like [Tailwind](https://tailwindcss.com), which provides utility classes like `rounded` and `dark:shadow-lg`.

*There is currently work ongoing on a styling framework for Sycamore/Perseus called [Jacaranda](https://github.com/framesurge/jacaranda), which will support fully typed styling!*

## Full-page layouts

A lot of websites these days are based on *full-page layouts*, which are when the entire page is taken up, usually by a header, some main content, and a footer. Getting this to work well, however, if unreasonably complicated in many cases. So, here's an example of exactly what CSS you need to get it working:

```css
{{#include ../../../examples/demos/full_page_layout/static/style.css}}
```

The comments in this file should make it fairly self-explanatory, but what it does is create a sticky header that maintains its spot when the user scrolls, while the footer will always be at the bottom of the page (but is not sticky when the content overflows the page). You can combine this with a layout component like this to get an easy way of creating full-page layouts for your sites:

```rust
{{#include ../../../examples/demos/full_page_layout/src/components/layout.rs}}
```

You can then use this like so:

```rust
{{#include ../../../examples/demos/full_page_layout/src/templates/index.rs}}
```

For more about full page layouts, see [this example](https://github.com/framesurge/perseus/tree/main/examples/demos/full_page_layout).
