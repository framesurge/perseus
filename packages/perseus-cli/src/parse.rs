#![allow(missing_docs)] // Prevents double-documenting some things

use crate::PERSEUS_VERSION;
use clap::Parser;

// The documentation for the `Opts` struct will appear in the help page, hence the lack of puncutation and the lowercasing in places

/// The command-line interface for Perseus, a super-fast WebAssembly frontend development framework!
#[derive(Parser)]
#[clap(version = PERSEUS_VERSION)]
// #[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(Parser)]
pub enum Subcommand {
    Build(BuildOpts),
    Export(ExportOpts),
    Serve(ServeOpts),
    /// Serves your app as `perseus serve` does, but puts it in testing mode
    Test(ServeOpts),
    Clean(CleanOpts),
    /// Ejects you from the CLI harness, enabling you to work with the internals of Perseus
    Eject,
    Deploy(DeployOpts),
    /// Prepares the `.perseus/` directory (done automatically by `build` and `serve`)
    Prep,
    Tinker(TinkerOpts),
    /// Runs one of the underlying commands that builds your app, allowing you to see more detailed logs
    #[clap(subcommand)]
    Snoop(SnoopSubcommand),
}
/// Builds your app
#[derive(Parser)]
pub struct BuildOpts {
    /// Build for production
    #[clap(long)]
    pub release: bool,
}
/// Exports your app to purely static files
#[derive(Parser)]
pub struct ExportOpts {
    /// Export for production
    #[clap(long)]
    pub release: bool,
}
/// Serves your app (set the `$HOST` and `$PORT` environment variables to change the location it's served at)
#[derive(Parser)]
pub struct ServeOpts {
    /// Don't run the final binary, but print its location instead as the last line of output
    #[clap(long)]
    pub no_run: bool,
    /// Only build the server, and use the results of a previous `perseus build`
    #[clap(long)]
    pub no_build: bool,
    /// Build and serve for production
    #[clap(long)]
    pub release: bool,
}
/// Removes `.perseus/` entirely for updates or to fix corruptions
#[derive(Parser)]
pub struct CleanOpts {
    /// Only remove the `.perseus/dist/` folder (use if you've ejected)
    #[clap(short, long)]
    pub dist: bool,
    /// Remove the directory, even if you've ejected (this will permanently destroy any changes you've made to `.perseus/`!)
    #[clap(short, long)]
    pub force: bool,
}
/// Packages your app for deployment
#[derive(Parser)]
pub struct DeployOpts {
    /// Change the output from `pkg/` to somewhere else
    #[clap(short, long, default_value = "pkg")]
    pub output: String,
    /// Export you app to purely static files (see `export`)
    #[clap(short, long)]
    pub export_static: bool,
}
/// Runs the `tinker` action of plugins, which lets them modify the Perseus engine
#[derive(Parser)]
pub struct TinkerOpts {
    /// Don't remove and recreate the `.perseus/` directory
    #[clap(long)]
    pub no_clean: bool,
    /// Force this command to run, even if you've ejected (this may result in some or all of your changes being removed, it depends on the plugins you're using)
    #[clap(long)]
    pub force: bool,
}

#[derive(Parser)]
pub enum SnoopSubcommand {
    /// Snoops on the static generation process (this will let you see `dbg!` calls and the like)
    Build,
    /// Snoops on the Wasm building process (mostly for debugging errors)
    WasmBuild,
    /// Snoops on the server process (run `perseus build` before this)
    Serve,
}
