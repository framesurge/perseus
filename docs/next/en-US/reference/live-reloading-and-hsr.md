# Live Reloading and HSR

When you develop with Perseus, you can add the `-w` flag to either `perseus serve` or `perseus export` to automatically rebuild your app whenever you change any code in your project. When you do, any browsers connected to the development version of your app will also be automatically reloaded, which allows for a more rapid development cycle. (If you want faster compile times, use the nightly channel of Rust.)

This also involves using *hot state reloading* (HSR), a world first in the non-JavaScript world pioneered by Perseus. This is very similar to *hot module reloading* (HMR) in JavaScript frameworks, which only changes the bare minimum amount of code necessary to let you preview your changes, meaning the state of your app is kept.

But what does that actually mean? Well, let's take a simple example. Imagine you're working on a form page that has twelve inputs that all need to be filled out. With HMR, most changes to your code will lead to small substitutions in the browser because of the way JS can be chunked into many small files --- your inputs into the form are preserved even across code changes, which is extremely helpful!

As you may know, Perseus has the concept of state freezing and thawing inbuilt, which allows you to turn the entire state of your app into a string and then restore your app to a single point of interaction from that, which would allow you to take a user back to exactly where they were after they logged back into your app, for example.

In development, this system is applied automatically to save your app's state to a string in your browser's storage automatically just after it's rebuilt, and this is then restored after the reload, meaning you're taken back to exactly where you were before you made the code change!

Of course, there are some cases in which this isn't possible --- namely when you change the data model of your app. So, if you add new parameters to the current page's state, Perseus won't be able to process it, and the previous state will be dumped. If you change the data model for another page though, things will still work, until you go to that page, because of the incremental nature of thawing (something you almost never need to care about). Very occasionally, this can lead to odd behavior, which is immediately fixed by simply reloading the page.

So, in summary, because Wasm can't be chunked, HMR can't be implemented for Wasm projects, including Perseus ones, so we invented a new way of achieving the same results grounded in the state-based architecture of Perseus, meaning you can easily develop complex flows in your app without losing state every time you change some code.

*Note: if you're a developer using another Wasm framework, and you'd like to implement HSR yourself, hop over to our [Discord channel on the Sycamore server](https://discord.com/invite/GNqWYWNTdp) if you want to discuss implementation details. All of this is open-source, and we'd be thrilled if HSR were more widely adopted in the Wasm community, to improve the developer experience of all!*
