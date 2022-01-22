# Functional Actions

The first type of action that a Perseus plugin can take is a functional action, and a single functional action can be taken by many plugins. These are the more common type of Perseus action, and are extremely versatile in extending the capabilities of the Perseus engine. However, they don't have the ability to replace critical functionality on their own.

## List of Functional Actions

Here's a list of all the functional actions currently supported by Perseus, which will likely grow over time. You can see these in [this file](https://github.com/arctic-hen7/perseus/blob/main/packages/perseus/src/plugins/functional.rs) in the Perseus repository.

If you'd like to request that a new action, functional or control, be added, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose).

-   `tinker` -- see [this section](:reference/plugins/tinker)
-   `settings_actions` -- actions that can alter the settings provided by the user with [`define_app!`](:reference/define-app)
    -   `add_static_aliases` -- adds extra static aliases to the user's app (e.g. a [TailwindCSS](https://tailwindcss.com) stylesheet)
    -   `add_templates` -- adds extra templates to the user's app (e.g. a prebuilt documentation system)
    -   `add_error_pages` -- adds extra [error pages](:reference/error-pages) to the user's app (e.g. a prebuilt 404 page)
-   `build_actions` -- actions that'll be run when the user runs `perseus build` or `perseus serve` as part of the build process (these will not be run in [static exporting](:reference/exporting))
    -   `before_build` -- runs arbitrary code just before the build process starts (e.g. to run a CSS preprocessor)
    -   `after_successful_build` -- runs arbitrary code after the build process has completed, if it was successful (e.g. copying custom files into `.perseus/dist/`)
    -   `after_failed_build` -- runs arbitrary code after the build process has completed, if it failed (e.g. to report the failed build to a server crash management system)
    -   `after_failed_global_state_creation` -- runs arbitrary code after if the build process failed to generate global state
-   `export_actions` -- actions that'll be run when the user runs `perseus export`
    -   `before_export` -- runs arbitrary code just before the export process starts (e.g. to run a CSS preprocessor)
    -   `after_successful_build` -- runs arbitrary code after the build process has completed (inside the export process), if it was successful (e.g. copying custom files into `.perseus/dist/`)
    -   `after_failed_build` -- runs arbitrary code after the build process has completed (inside the export process), if it failed (e.g. to report the failed export to a server crash management system)
    -   `after_failed_export` -- runs arbitrary code after the export process has completed, if it failed (e.g. to report the failed export to a server crash management system)
    -   `after_failed_static_copy` -- runs arbitrary code if the export process fails to copy the `static` directory (e.g. to report the failed export to a server crash management system)
    -   `after_failed_static_alias_dir_copy` -- runs arbitrary code if the export process fails to copy a static alias that was a directory (e.g. to report the failed export to a server crash management system)
    -   `after_failed_static_alias_file_copy` -- runs arbitrary code if the export process fails to copy a static alias that was a file (e.g. to report the failed export to a server crash management system)
    -   `after_successful_export` -- runs arbitrary code after the export process has completed, if it was successful (e.g. copying custom files into `.perseus/dist/`)
    -   `after_failed_global_state_creation` -- runs arbitrary code if the export process failed to generate global state
-   `export_error_page_actions` --- actions that'll be run when exporting an error page - `before_export_error_page` --- runs arbitrary code before this process has started (providing the error code to be exported for and the output file)
    -   `after_successful_export_error_page` -- runs arbitrary code after this process has completed, if it was successful
    -   `after_failed_write` -- runs arbitrary code after this process has completed, if it couldn't write to the target output file
-   `server_actions` -- actions that'll be run as part of the Perseus server when the user runs `perseus serve` (or when a [serverful production deployment](:reference/deploying/serverful) runs)
    -   `before_serve` -- runs arbitrary code before the server starts (e.g. to spawn an API server)
-   `client_actions` -- actions that'll run in the browser when the user's app is accessed
    -   `start` -- runs arbitrary code when the Wasm delivered to the browser is executed (e.g. to ping an analytics service)
