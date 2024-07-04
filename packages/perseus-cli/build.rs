#[cfg(feature = "completions")]
mod completions {
    #[path = "../src/parse.rs"]
    mod parse;

    use clap::{CommandFactory, ValueEnum};
    use clap_complete::{generate_to, shells::Shell};
    use clap_complete_nushell::Nushell;
    use std::io::Error;

    pub fn generate_completions() -> Result<(), Error> {
        let out_dir = "completions";
        let bin_name = "perseus";
        // on windows, bin_name is "perseus.exe"
        #[cfg(windows)]
        let bin_name = "perseus.exe";

        let mut cmd = parse::Opts::command();

        for shell in Shell::value_variants() {
            generate_to(*shell, &mut cmd, bin_name, out_dir)?;
        }
        generate_to(Nushell, &mut cmd, bin_name, out_dir)?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "completions")]
    completions::generate_completions()?;
    Ok(())
}
