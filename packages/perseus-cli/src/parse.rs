#![allow(missing_docs)] // Prevents double-documenting some things

use crate::PERSEUS_VERSION;
use clap::{AppSettings, Clap};

// The documentation for the `Opts` struct will appear in the help page, hence the lack of puncutation and the lowercasing in places

/// The command-line interface for Perseus, a super-fast WebAssembly frontend development framework!
#[derive(Clap)]
#[clap(version = PERSEUS_VERSION)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(Clap)]
pub enum Subcommand {
    Build(BuildOpts),
    Export(ExportOpts),
    Serve(ServeOpts),
    /// Serves your app as `perseus serve` does, but puts it in testing mode
    Test(ServeOpts),
    Clean(CleanOpts),
    /// Ejects you from the CLI harness, enabling you to work with the internals of Perseus
    Eject,
    /// Prepares the `.perseus/` directory (done automatically by `build` and `serve`)
    Prep,
}
/// Builds your app
#[derive(Clap)]
pub struct BuildOpts {
    /// Build for production
    #[clap(long)]
    pub release: bool,
}
/// Exports your app to purely static files
#[derive(Clap)]
pub struct ExportOpts {
    /// Export for production
    #[clap(long)]
    pub release: bool,
}
/// Serves your app (set the `$HOST` and `$PORT` environment variables to change the location it's served at)
#[derive(Clap)]
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
#[derive(Clap)]
pub struct CleanOpts {
    /// Only remove the `.perseus/dist/` folder (use if you've ejected)
    #[clap(short, long)]
    pub dist: bool,
    /// Remove the directory, even if you've ejected (this will permanently destroy any changes you've made to `.perseus/`!)
    #[clap(short, long)]
    pub force: bool,
}
