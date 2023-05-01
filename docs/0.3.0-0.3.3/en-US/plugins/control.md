# Control Actions

Control actions in Perseus can only be taken by one plugin, unlike [functional actions](:plugins/functional), because, if multiple plugins took them, Perseus wouldn't know what to do. For example, if more than one plugin tried to replace the [immutable store](:stores), Perseus wouldn't know which alternative to use.

Control actions can be considered more powerful than functional actions because they allow a plugin to not only extend, but to replace engine functionality.

## List of Control Actions

Here's a list of all the control actions currently supported by Perseus, which will likely grow over time. You can see these in [this file](https://github.com/framesurge/perseus/blob/main/packages/perseus/src/plugins/control.rs) in the Perseus repository.

If you'd like to request that a new action, functional or control, be added, please [open an issue](https://github.com/framesurge/perseus/issues/new/choose).

_Note: there are currently very few control actions, and this list will be expanded over time._

-   `settings_actions` -- actions that can alter the settings provided by the user with [`define_app!`](:define-app)
    -   `set_immutable_store` -- sets an alternative [immutable store](:stores) (e.g. to store data somewhere other than the filesystem for some reason)
    -   `set_locales` -- sets the app's locales (e.g. to fetch locales from a database in a more convenient way)
    -   `set_app_root` -- sets the HTML `id` of the `div` in which to render Perseus (e.g. to fetch the app root from some other service)
-   `build_actions` -- actions that'll be run when the user runs `perseus build` or `perseus serve` as part of the build process (these will not be run in [static exporting](:exporting))
-   `export_actions` -- actions that'll be run when the user runs `perseus export`
-   `server_actions` -- actions that'll be run as part of the Perseus server when the user runs `perseus serve` (or when a [serverful production deployment](:deploying/serverful) runs)
-   `client_actions` -- actions that'll run in the browser when the user's app is accessed
