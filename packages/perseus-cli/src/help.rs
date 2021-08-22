use crate::PERSEUS_VERSION;

pub fn help(output: &mut impl std::io::Write) {
    writeln!(
        output,
        "Perseus v{version} help page:
-------------------------

This is the CLI for Perseus, a super-fast WebAssembly frontend development framework! For the full reference, please see the documentation at https://arctic-hen7.github.io/perseus.

-h, --help			prints this help page
-v, --version			prints the current version of the CLI

build				builds your app (-p/--prod for production, -w/--watch to watch files)
serve				serves your app (accepts $PORT and $HOST env vars)

Further information can be found at https://arctic-hen7.github.io/perseus.
        ",
        version = PERSEUS_VERSION
    )
    .expect("Failed to write help page.")
}