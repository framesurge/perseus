#!/bin/bash

# This script tests the given example with Bonnie
category=$1
example=$2
integration=$3 # This allows us to run all the tests with every integration
headless=$4

# Get the path to the server executable (last line of output when we use `--no-run`)
exec=$(bonnie dev example $category $example test --no-run --integration $integration | tail -n 1)
# Now move into the correct execution context
cd examples/$category/$example/.perseus/server
# And run the server itself in the background (making sure to pass through that we're testing)
PERSEUS_TESTING=true $exec &

# Now execute tests against that
cd ../../ # Now we're in the example's directory
# With `geckodriver`, we can only run one test at a time
# TODO use chromedriver instead
if [[ $headless == "--headless" ]]; then
    PERSEUS_RUN_WASM_TESTS=true PERSEUS_RUN_WASM_TESTS_HEADLESS=true cargo test -- --test-threads 1
else
    PERSEUS_RUN_WASM_TESTS=true cargo test -- --test-threads 1
    fi
code=$?

# Now that we're done, halt the server
kill %1

exit $code
