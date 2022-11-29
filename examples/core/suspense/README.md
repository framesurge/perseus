# Suspense Example

Where Sycamore has the `Suspense` component, designed to allow asynchronous rendering of a view, Perseus applies the same principle to its state platform, allowing you to attach an asynchronous handler to any field of your reactive state that will be called to modify it on the browser-side. This is best used for performing network requests, and any other particular asynchronous work that you want to perform immediately upon loading a page.

Note that Sycamore's `Suspense` system is incompatible with the Perseus build system, and the use of this system with the state platform is highly recommended as an alternative.
