#!/bin/bash

# This script tests the given example with Bonnie
example=$1
headless=$2

# Get the path to the server executable (last line of output when we use `--no-run`)
exec=$(bonnie dev example $example test --no-run | tail -n 1)
# Now move into the correct execution context
cd examples/$example/.perseus/server
# And run the server itself in the background (making sure to pass through that we're testing)
PERSEUS_TESTING=true $exec &

# Now execute tests against that
cd ../../ # Now we're in the example's directory
if [[ $headless == "--headless" ]]; then
    PERSEUS_RUN_WASM_TESTS=true PERSEUS_RUN_WASM_TESTS_HEADLESS=true cargo test
else
    PERSEUS_RUN_WASM_TESTS=true cargo test
fi

# Now that we're done, halt the server
kill %1
