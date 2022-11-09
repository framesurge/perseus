# Capsules Example

This example demonstrates Perseus' capsules system, whereby the entire template system can be used for regular Sycamore components! Imagine this scenario: you're making a blog, and you want blog posts to be able to be part of a series. You render all your blog posts from Markdown files in some folder with the templating system, and it's all working beautifully, but how do you display, inside a post, the series that it's a part of? Short of iterating over every single post and reconstructing the series *every time you build a post*, this is basically impossible with Perseus. Enter capsules. If we could create a Sycamore component that could generate state by creating a version of itself for each series, and if we could then include that on each of our blog posts as necessary, parsing through the name of the current post to be highlighted, we could achieve this in a far more ergonomic manner!

But this is just one use-case for capsules. Their greatest benefit is that, if a user navigates from one page that uses capsule A to another page that also uses capsule A, then capsule A doesn't need to change at all, meaning fewer computations, and less layout shift, along with faster page load times!

Although the capsules system is currently quite unstable, it will eventually be recommended to break Perseus pages up into capsules wherever possible for efficiency.
