# State Generation Example

This examples shows all the ways of generating template state in Perseus, with each file representing a different way of generating state. Though this example shows many concepts, it's practical to group them all together due to their fundamental connectedness.

Note that all the state generation functions in Perseus, as well as several others, such as those responsible for generatint a page's `<head>` and settings its headers, can be either *fallible* or *infallible*. In other words, they can either return `T` (whatever their return type may be, e.g. `BuildPaths`, or perhaps `MyPageState`), or `Result<T, E>`, where `E` is some arbitrary error type. Throughout the Perseus examples, since errors are rarely shown, the infallible versions are shown, although this example uses the fallible versions to showcase `BlamedError`, a special type of error that can 'blame' either the client (if they caused the error), or the server.

Functions that can either be fallible or infallible do not have to be specified with different functions on `Template` (unlike, say, `.template()` and `.template_with_state()`).
