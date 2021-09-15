import init, { run } from "/.perseus/bundle.js";
async function main() {
    await init("/.perseus/bundle.wasm");
    run();
}
main();
