# Freezing to IndexedDB

One of the most common places to store frozen state is inside the browser, which can be done with Perseus' inbuilt `IdbFrozenStateStore` system, which uses [IndexedDB]() to store as many frozen states as you want, allowing you to revert to not just the previous state, but the one before that, the one before that, etc.

To use this system, you'll need to enable the `idb-freezing` feature flag, and then you can use the system as per the below example.

*Further documentation on this system will be written after it's been built, which it will be by v0.3.3. Sorry for the wait!*
