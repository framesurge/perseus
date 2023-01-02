# Contributing

First off, thanks so much for taking the time to contribute to Perseus, it's greatly appreciated!

Perseus uses [Tribble](https://github.com/arctic-hen7/tribble) for managing contributions, see [here](https://framesurge.sh/perseus/tribble/workflow/perseus/) for an interactive system that will guide you through making contributions to this repository. Please note that this system is still in beta, so please just open a GitHub issue as usual if something's not working properly there. Otherwise, issues should be created through Tribble (this allows you to basically triage your issue yourself, making things move more quickly).

The rest of this document is dedicated to explaining how to get a local copy of Perseus and open a pull request, and is primarily targeted toward those contributing to an open-source project for the first time.

## Getting a copy of Perseus

Before you can make the changes you want to Perseus, you'll need to grab a local copy of the repository to work on. You can do this by *forking* the repository to your GitHub account and then by *cloning* your fork.

1. Press the *Fork* button at the top-right of this page (next to the star counter). If necessary, choose to fork Perseus to your own personal account (or somewhere else if you'd like).
2. Clone the created repository in your preferred way. You can do this with the GitHub CLI by running `gh clone <your-username>/perseus` in the directory in which you want Perseus to be (it will be created as a `perseus/` directory). You can also use regular `git` with `git clone git@github.com:<your-username>/perseus.git`.

You should now have a local copy of Perseus to work with! Before you start making changes though, you should install [Rust](https://rust-lang.org/tools/install) and [Bonnie](https://github.com/arctic-hen7/bonnie) (`cargo install bonnie`), and then you should run `bonnie setup` in the project directory. This will prepare Perseus for development, and you'll need to do this for any changes that involve compiling the code. If you're just making some simple changes to the documentation, you can skip these steps.

We also ask you to sign your commits with GPG, it makes things more secure for everyone.

## Contributing your changes

Once you've made your changes to your local copy and committed them with Git, you can commit them back to Perseus through a *pull request*.

1. Push your changes to your fork on GitHub with `git push origin main` (if you're working on a different branch, change `main` to the name of that branch).
2. Go to your fork's GitHub page, press *Contribute*, and then press *Open pull request*.
3. Describe your PR in the GitHub interface and submit it! If it's not quite ready yet, you can mark it as a draft.

Once you've submitted your PR, we'll try to get to reviewing it as quickly as possible! Unfortunately, not every pull request can be merged, but we'll do our best to request changes so that yours can be best integrated into the project.

## Old Details

This section pre-dates the usage of Tribble, but it will very soon be made obsolete. During the transition, this information will still be available.

<details>
<summary>Click here to see the old details.</summary>

## Building

Perseus uses [Bonnie](https://github.com/arctic-hen7/bonnie) for command aliasing (you can install it with `cargo install bonnie`), and most of the project testing is done in the `examples` directory. You can run `bonnie help` to see all available commands, but this is the one you'll use the most:

-   `bonnie dev example showcase serve` -- serves the `showcase` example to <http://localhost:8080>

Before you do anything though, you should run `bonnie setup`, which will do few things to prepare your local development. This includes running `cargo build`, so it will take a little while. Note that, without running this command, most other actions you try to take will cause errors.

## Testing

Nearly all Perseus' tests are end-to-end, and run using the Perseus test macro for each example (under `examples`). You can run all tests with `bonnie test`, provided that you're running a WebDriver instance at <http://localhost:4444>.

If you're new to WebDriver, install `geckodriver` and Firefox, and then run `geckodriver` in another terminal. Then all Perseus tests will run fine.

You can also run a full check on all your code with `bonnie check`, which is the same as what's performed on CI.

## Documentation

If the code you write needs to be documented in, the README, the book, or elsewhere, please do so! Also, **please ensure your code is commented**, it makes everything so much easier.

All the Perseus documentation is stored inside `docs/`, which is then split into a folder for each version of the documentation (e.g. `0.1.x`, `0.2.x`, `0.3.x`), with the additional special folder `next`, which is rolling release. There's also a `manifest.json` file that defines which versions are outdated, stable, or in beta, as well as the points in the Git history that they correspond to. In each version folder, there are folders for each language of the docs, and contributions in the area of internationalization are very welcome!

The docs are rendered [here](https://framesurge.sh/perseus/en-US/docs/intro), with a sidebar that acts as a table of contents. That sidebar is rendered from the special file `SUMMARY.md`, which links to pages in the docs as `/docs/path/to/file`, a locale and version will be inserted automatically at build time. For adding to the documentation, you should add to this file with an entry for each file you've added.

Documentation files are written in Markdown, and will be served on the website at their filenames (without the `.md` extension though). All documentation files must start with `# Title Here`, which will be used as the document's title in the browser. If you want to link to other pages in the docs from your page, use the special linking syntax `:path/to/file`, and the appropriate locale and version will be inserted automatically.

All code examples in the docs must come from real-world files, which can be done with the special syntax `{{#include path/to/file/relative/to/current/file}}`. If you want to include particular lines, use `#include_lines` instead and provide the lines as `relative/path:start:end` (where `start` and `end` are the starting and ending lines you want, 1-indexed). Code examples for languages other than Rust (e.g. `Dockerfile`s) may be directly written into the document, but they'll need to be kept up-to-date. Note that any time you use the current version of Perseus anywhere in your files, it will be updated as the Perseus version is updated (with a find-and-replace that affects everything except for the `CHANGELOG.md`).

Note that if you're updating the docs to fix a typo, you may need to change the typo in multiple versions. If you're adding new documentation, you should add it to both `next` and the latest applicable version (which could be a beta version).

Finally, you can see the documentation you've written by running `bonnie site` (assuming you've already run `bonnie setup`), which requires [Bonnie](https://github.com/arctic-hen7/bonnie), [TailwindCSS](https://tailwindcss.com) (`npm i -g tailwindcss`), and `concurrently` (`npm i -g concurrently`). Note that this will use the local, bleeding-edge, unreleased version of the Perseus CLI, not the one on `crates.io`, so you don't have to install the CLI. You'll be able to see the website at <http://localhost:8080>.

## Branches

Perseus uses a relatively intuitive branching system:

-   `main` -- the rolling-release version of the project, which should not be committed to directly
-   `stable` -- the stable version of the project, which should reflect released features (should be in line with latest tag)

A separate branch is created for new features/fixes, which are then merged into `main` with pull requests. Note that new releases can only be authored from the `stable` branch (checked by Bonnie).

## Committing

We use the Conventional Commits system, but you can commit however you want. Your pull request will be squashed and merged into a single compliant commit, so don't worry about this!

We do request though that you make sure your commits are signed with GPG, it helps verify the integrity of your code, and is good practice generally.

</details>
