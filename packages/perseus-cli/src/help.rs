use crate::PERSEUS_VERSION;

pub fn help(output: &mut impl std::io::Write) {
    writeln!(
        output,
        "Perseus v{version} help page:
-------------------------

This is the CLI for Perseus, a super-fast WebAssembly frontend development framework! For the full reference, please see the documentation at https://arctic-hen7.github.io/perseus.

-h, --help			prints this help page
-v, --version			prints the current version of the CLI

build				builds your app
serve				serves your app (accepts $PORT and $HOST env vars, --no-build to serve pre-built files)

Please note that watching for file changes is not yet inbuilt, but can be achieved with a tool like 'entr' in the meantime.
Further information can be found at https://arctic-hen7.github.io/perseus.
        ",
        version = PERSEUS_VERSION
    )
    .expect("Failed to write help page.")
}
