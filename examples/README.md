# Examples

This folder contains examples for Perseus to be used as learning resources! If any of these don't work, please [open an issue](https://github.com/arctic-hen7/perseus/issues/choose) to let us know!

These examples are all fully self-contained, and do not serve as examples in the traditional Cargo way, they are each indepedent Perseus apps, just with relative path dependencies so that they use the bleeding edge version of Perseus in this crate. If you want the examples for a particualr version, you should navigate to the appropriate tag for that version in GitHub and then come back to this directory at that point in the commit history.

*Note: these examples used toto double as end-to-end tests for Perseus. Those have now been moved to the `tests/` directory.*

-   Showcase -- an app that demonstrates all the different features of Perseus, including SSR, SSG, and ISR (this example is actively used for testing/development)
-   Basic -- a simple app that uses the Perseus CLI
    -   This has `.perseus/` included in Git, it's where that's developed
-   i18n -- a simple app that showcases internationalization in particular
-   Tiny -- the smallest Perseus can get, the _Hello World!_ example
-   Plugins -- an example of creating and integrating plugins into Perseus
-   Fetching -- an example of fetching data at build time and in the browser with `reqwasm`
