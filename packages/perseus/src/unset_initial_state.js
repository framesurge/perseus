// Currently, Rust's Wasm toolkit can't set global variables, which is all this function does
export function unset_initial_state() {
    window.__PERSEUS_INITIAL_STATE = undefined;
}
