# Static Content Example

This example doesn't introduce any new code, it just shows how to host arbitrary static content with Perseus. By default, any content in the `static/` directory at the root of your project will be hosted at the URL `/.perseus/static/`, though you can also add aliases to content inside your project's root directory to be hosted at any URL in your app. This example shows both methods, and you'll be able to find a file at `/test.txt` and at `/.perseus/static/style.css` (named so as to indicate that you would typically put stylesheets in the `static/` directory).
