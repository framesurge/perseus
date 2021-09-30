# Writing Views

Perseus is fundamentally a high-level framework around [Sycamore](https://github.com/sycamore-rs/sycamore), which provides all the underlying reactivity and the ability to write code that turns into visible HTML elements.

It would be foolish to reproduce here all the fantastic work of Sycamore, and you can read [their docs](https://sycamore-rs.netlify.app/docs/v0.6/getting_started/installation) to understand how reactivity, variable interpolation, and all the rest of their amazing systems work.

Note that Perseus makes some sections of Sycamore's docs irrelevant (namely the sections on routing and SSR), as they're managed internally. Note that if you want to use Perseus without the CLI (*very* brave), these sections will be extremely relevant.

## Using Sycamore without Perseus

If you want to create a pure SPA without all the overhead of Perseus, you may want to use Sycamore without Perseus. Note that this won't provide as good SEO (search engine optimization), and you'll miss out on a number of additional features (like i18n, inferred routing, rendering strategies, and pre-optimized static exporting without a server), but for applications where these are unnecessary, Sycamore is perfect on its own.
