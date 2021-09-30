# Styling

Perseus aims to make styling as easy as possible, though there are a number of things that you should definitely know about before you start to style a Perseus app!

It's very easy to import stylesheets with Perseus (be they your own, something like [TailwindCSS](https://tailwindcss.com), etc.). You just add them to the `static/` directory at the root of your project, and then they'll be available at `/.perseus/static/your-filename-here`. That's described in more detail in [this section](./static-content.md).

## Full-Page Layouts

If you've tried to create something like a stick footer, you've probably become extremely frustrated by Perseus, which puts all your content in a container `<div>` (in addition to the `<div id="root"></div>`). Unfortunately, this is necessary until Sycamore supports creating a template for an existing DOM node, and this does lead to some styling problems.

Notably, there are actually two of these `<div>`s at the moment: one for the content that the server pre-renders in [initial loads](./advanced/initial-loads.md) and another for when that content is hydrated by Perseus' client-side logic. That means that, if you only style one of these, you'll get a horrible flash of unstyled content, which nobody wants. To make this as easy as possible, Perseus provides a class `__perseus_content` that applies to both of these `<div>`s. Also, note that the `<div>` for the initial content will become `display: none;` as soon as the page is ready, which means you won't get it interfering with your layouts.

Knowing this, the main changes you'll need to make to any full-page layout code is to apply the styles to `.__perseus_content` instead of `body` or `#root`. As with CSS generally, if you expect `.__perseus_content` to take up the whole page, you'll need to make all its parents (`#root`, `body`, `html`) also take up the whole page (you can do this by setting `height: 100vh;` on `body`).

Any other issues should be solvable by inspecting the DOM with your browser's DevTools, but you're more than welcome to ask for help on the [Sycamore Discord server](https://discord.gg/PgwPn7dKEk), where Perseus has its own channel!
