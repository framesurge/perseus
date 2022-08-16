# This will need to be updated for warning links
-latest-stable-version = 0.3.4

perseus = Perseus
sycamore = Sycamore
navlinks =
    .docs = Docs
    .comparisons = Comparisons
    .plugins = Plugins
index-intro =
    .heading = The Rust framework for the modern web
    .get-started-button = Get started now
    .github-button = GitHub
index-state-gen =
    .heading = Generate state, on your terms
    .desc = Perseus apps can generate state whenever they like and pass it to <a href="https://github.com/sycamore-rs/sycamore" class="underline">Sycamore</a>, one of the fastest web frameworks in the world.
index-i18n =
    .heading = I18n that just works
    .desc = Just add translations using <a href="https://projectfluent.org" class="underline">Fluent</a>, and your app can be set up in seconds in multiple languages, with automatic user locale detection.
index-opts =
    .heading = You want options? We got options.
    .desc = Perseus comes with a built-in plugins system and full customizability of almost every part of the system. Static exporting? Serverful deployment? Fetch data by carrier pigeon? Easy.
index-speed =
    .heading = Fast. Crazy. Fast.
    .desc-line-1 = Underlying platform? <strong>Rust.</strong>
    .desc-line-2 = Target platform? <strong>WebAssembly.</strong>
    .desc-line-3 = Lighthouse scores? <strong>💯</strong>
    .desktop-perf-label = Performance (Desktop)
    .mobile-perf-label = Performance (Mobile)
    .best-practices-label = Best Practices
index-cta =
    .heading = Get started with Perseus today!
    .docs-button = Docs
    .gh-button = GitHub
    .api-docs-button = API Docs
    .crates-io-button = Crates.io
    .matrix-button = Matrix
    .discord-button = Discord
    .comparisons-button = Comparisons

footer =
    .copyright = © <a href="https://github.com/arctic-hen7" class="underline">arctic-hen7</a> { $years }, see license <a href="https://github.com/arctic-hen7/perseus/blob/main/LICENSE" class="underline">here</a>.

comparisons-title = Comparisons to Other Frameworks
comparisons-heading = Comparisons
comparisons-subtitle = See how Perseus compares to other web development frameworks.
comparisons-extra = Is there anything we're missing here? Please <a href ="https://github.com/arctic-hen7/perseus/issues/new/choose" class="underline">open an issue</a> and let us know!
comparisons-table-header = Comparison
comparisons-table-headings =
    .name = Name
    .supports_ssg = Static generation
    .supports_ssr = Server-side rendering
    .supports_ssr_ssg_same_page = SSG & SSR in same page
    .supports_i18n = Internationalization
    .supports_incremental = Incremental generation
    .supports_revalidation = Revalidation
    .inbuilt_cli = Inbuilt CLI
    .inbuilt_routing = Routing support
    .supports_shell = App shell
    .supports_deployment = Easy deployment
    .supports_exporting = Static exporting
    .language = Language
    .homepage_lighthouse_desktop = Homepage Lighthouse Score (Desktop)
    .homepage_lighthouse_mobile = Homepage Lighthouse Score (Mobile)
comparisons-table-details =
    .language = The programming language this framework is built in. (Hint: Rust is the fastest!)
    .supports_ssg = Whether or not the framework supports pre-rendering pages at build-time.
    .supports_ssr = Whether or not the framework supports rendering pages dynamically when they're requested.
    .supports_ssr_ssg_same_page = Whether or not the framework supports SSG and SSR (see above) in the very same page.
    .supports_i18n = Whether or not the framework supports building a multilingual app. Some frameworks support this through plugins.
    .supports_incremental = Whether or not the framework supports rendering pages on-demand and then caching them for future use.
    .supports_revalidation = Whether or not the framework supports rebuilding pages that were built at build-time at request-time. Some frameworks can do this by time, or by custom logic. Perseus supports both.
    .inbuilt_cli = Whether or not the framework has a command-line interface for convenient usage. Some frameworks use third-party CLIs.
    .inbuilt_routing = Whether or not the framework supports inbuilt routing. Most frameworks have their own way of doing this if they do support it.
    .supports_shell = Whether or not this framework has an app shell, which makes switching pages much faster and cleaner.
    .supports_deployment = Whether or not this framework supports easy deployment. Perseus can deploy in one command.
    .supports_exporting = Whether or not this framework can operate without a server, as a series of purely static files.
    .homepage_lighthouse_desktop = The Lighthouse score out of 100 for desktop (higher is better). These are collected from Google's PageSpeed Insights tool. These are for the framework's website, and may not reflect the performance of all sites made with the framework.
    .homepage_lighthouse_mobile = The Lighthouse score out of 100 for mobile (higher is better). These are collected from Google's PageSpeed Insights tool. These are for the framework's website, and may not reflect the performance of all sites made with the framework.
comparisons-sycamore-heading = Perseus vs Sycamore
comparisons-sycamore-text = Perseus is a framework that uses Sycamore to write views (the things users see), so there's not much point in comparing the two as competitors. However, Sycamore can be used without Perseus, which works perfectly well, though you'll miss out on features like inbuilt internationalization, static generation (though you could build this yourself), and incremental generation. If you want to build just a bit of your site with Rust, Sycamore is the perfect tool, Perseus is a full framework designed for building entire websites.

docs-title-base = Perseus Docs
docs-status =
    .outdated = This version of the documentation is outdated, and features documented here may work differently now. You can see the latest stable version of the docs <a href="en-US/docs/{ -latest-stable-version }/intro" class="underline">here</a>.
    .beta = This version of the documentation is for a version that has only been released in beta, and is not yet stable. Features documented here may not be present in the latest stable version, and they're subject to rapid and drastic change. You can see the latest stable version of the docs <a href="en-US/docs/{ -latest-stable-version }/intro" class="underline">here</a>.
    .next = This version of the documentation is for a version that has not yet been released, and features documented here may not be present in the latest release. You can see the latest stable version of the docs <a href="en-US/docs/{ -latest-stable-version }/intro" class="underline">here</a>.
docs-version-switcher =
    .next = Next (unreleased)
    .beta = v{ $version } (beta)
    .stable = v{ $version } (stable)
    .outdated = v{ $version } (outdated)

plugins-title = Plugins
plugins-title-full = Perseus Plugins
plugin-card-author = By { $author }
plugin-details =
    .repo_link = Repository:
    .crates_link = Crate page:
    .docs_link = Docs:
    .site_link = Site:
    .metadata_heading = Metadata
    .no_link_text = N/A
plugin-search =
    .placeholder = Search
    .no_results = No results found.
plugins-desc = These are all the public plugins for Perseus! If you've made a plugin, and you'd like it to be listed here, please open an issue on our repository!
