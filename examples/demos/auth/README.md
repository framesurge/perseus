# Authentication Example

This example demonstrates how to set up a basic authentication system in Perseus. 

*Note: the way this is implemented uses very rudimentary storage of simple identifiers in the browser's local storage. Not only is the 'token' implementation ludicrously insecure and for educational purposes only, using web storage simply will not work in some implementation of private browsing mode (e.g. Safari, which makes using this API effectively impossible in that mode). See [here](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API#private_browsing_incognito_modes) for further details.*
