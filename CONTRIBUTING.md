# Contributing

First off, thanks so much for taking the time to contribute to Perseus, it's greatly appreciated!

## I just want to propose something

If you just want to let us know about a bug, or propose a new feature, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose) on this repository. We'll take a look as soon as possible!

## I want to help

That's fantastic! You can check out the [roadmap](./README.md#Roadmap) or the [issues](https://github.com/arctic-hen7/perseus/issues) to see what's currently needing to be done. If you want to work on something that's not on there, please [file an issue](https://github.com/arctic-hen7/perseus/issues/new/choose) and we'll take a look it as soon as possible!

## How do I contribute?

Contributing to a project on Github is pretty straight forward. If this is you're first time contributing to a project, all you need to do is fork this repository to your own GitHub account and then change the code you want to (usually on your local machine, you'd pull your fork down). Commit your changes as necessary, and when you're done, submit a pull request on this repository and we'll review it as soon as possible!

Make sure your code doesn't break anything existing, that all tests pass, and, if necessary, add tests so your code can be confirmed to work automatically.

After you've submitted a pull request, a maintainer will review your changes. Unfortunately, not every pull request will be merged, but we'll try to request changes so that your pull request can best be integrated into the project.

## Building

Perseus uses [Bonnie](https://github.com/arctic-hen7/bonnie) for command aliasing (you can install it with `cargo install bonnie`), and most of the project testing is done in the `examples` directory. You can run `bonnie help` to see all available commands, but this is the one you'll use the most:

-   `bonnie dev example showcase serve` -- serves the `showcase` example to <http://localhost:8080>

Before you do anything though, you should run `bonnie setup`, which will do a few thigns to prepare your local development. This includes running `cargo build`, so it will takea a little while. Note that, without running this command, most other actions you try to take will cause errors.

## Testing

Nearly all Perseus' tests are end-to-end, and run using the Perseus test macro for each example (under `examples`). You can run all tests with `bonnie test`, provided that you're running a WebDriver instance at <http://localhost:4444>.

If you're new to WebDriver, install `geckodriver` and Firefox, and then run `geckodriver` in another terminal. Then all Perseus tests will run fine.

You can also run a full check on all your code with `bonnie check`, which is the same as what's performed on CI.

## Documentation

If the code you write needs to be documented in, the README, the book, or elsewhere, please do so! Also, **please ensure your code is commented**, it makes everything so much easier.

All the Perseus documentation is stored inside `docs/`, which is then split into a folder for each version of the documentation (e.g. `0.1.x`, `0.2.x`, `0.3.x`), with the additional special folder `next`, which is rolling release. There's also a `manifest.json` file that defines which versions are outdated, stable, or in beta, as well as the points in the Git history that they correspond to. In each version folder, there are folders for each language of the docs, and contributions in the area of internationalization are very welcome!

The docs are rendered [here](https://arctic-hen7/.github.io/perseus/en-US/docs/intro), with a sidebar that acts as a table of contents. That sidebar is rendered from the special file `SUMMARY.md`, whcih links to pages in the docs as `/docs/path/to/file`, a locale and version will be inserted automatically at build time. For adding to the documentation, you should add to this file with an entry for each file you've added.

Documentation files are written in Markdown, and will be served on the website at their filenames (without the `.md` extension though). All documentation files must start with `# Title Here`, which will be used as the document's title in the browser. If you want to link to other pages in the docs from your page, use the special linking syntax `:path/to/file`, and the appropriate locale and version will be inserted automatically.

All code examples in the docs must come from real-world files, which can be done with the special syntax `{{#include path/to/file/relative/to/current/file}}`. If you want to include particular lines, use `#include_lines` instead and provide the lines as `relative/path:start:end` (where `start` and `end` are the starting and ending lines you want, 1-indexed). Code examples for languages other than Rust (e.g. `Dockerfile`s) may be directly written into the document, but they'll need to be kept up-to-date. Note that any time you use the current version of Perseus anywhere in your files, it will be updated as the Perseus version is updated (with a find-and-replace that affects everything except for the `CHANGELOG.md`).

Note that if you're updating the docs to fix a typo, you may need to change the typo in multiple versions. If you're adding new documentation, you should add it to both `next` and the latest applicable version (which could be a beta version).

Finally, you can see the documentation you've written by running `bonnie site run` (assuming you've already run `bonnie setup`), whcih requires no prerequisites other than [Bonnie](https://github.com/arctic-hen7/bonnie) and [TailwindCSS](https://tailwindcss.com) (`npm i -g tailwindcss`). Note that this will use the local, bleeding-edge, unreleased version of the Perseus CLI, not the one on `crates.io`, so you don't have to install the CLI. You'll be able to see the website at <http://localhost:8080>.

## Branches

Perseus uses a relatively intuitive branching system:

-   `main` -- the rolling-release version of the project, which should not be committed to directly
-   `stable` -- the stable version of the project, which should reflect released features (should be in line with latest tag)

A separate branch is created for new features/fixes, which are then merged into `main` with pull requests. Note that new releases can only be authored from the `stable` branch (checked by Bonnie).

## Committing

We use the Conventional Commits system, but you can commit however you want. Your pull request will be squashed and merged into a single compliant commit, so don't worry about this!
