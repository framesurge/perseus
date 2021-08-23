import init, { run } from "./pkg/perseus_showcase_app.js";
async function main() {
    await init("/.perseus/bundle.wasm");
    run();
}
main();
