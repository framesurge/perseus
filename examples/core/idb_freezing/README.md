# IndexedDB Freezing Example

This example shows how Perseus can support freezing state to IndexedDB and retrieving it from there later, which is the mechanism of state freezing that many apps will use. This is also the basis of Perseus' HSR system.

Notably, this requires the `wasm-bindgen-futures` package and the `idb-freezing` feature enabled on the `perseus` package.
