import init, { run } from "./dist/pkg/perseus_cli_builder.js";
async function main() {
    await init("/.perseus/bundle.wasm");
    run();
}
main();
