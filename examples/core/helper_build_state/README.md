# Helper Build State Example

This example details the use of Perseus' *extra build state* system, which allows you to generate some arbitrary extra state when you generate the paths a template will generate. Imagine this: you're creating a blog that fetches all its pages from Markdown files in a folder, some of which are part of series of blog posts. For posts that are part of a series, you would like to list all the other posts in the series at the top of the page, but how do you do this?

In build paths, you're iterating over all the files in that posts directory, and in build state, you're reading the given file, which has only which series it's part of. You would only be able to find out all the other posts in the series by iterating over every single file in the posts directory *every time you build a single post*! This is extremely inefficient, since you're reading the same things over and over again!

With helper state, you can read all the series once in build paths, creating perhaps a `HashMap<String, Vec<String>>` to represent them, and then you can return this in the `extra` property of `BuildPaths`. Then, in each instance of build state, you can grab that state straight out of `StateGeneratorInfo` --- no more unnecessary reads needed!

This feature falls into the more advanced category of Perseus features, but it's certainly not difficult to use. Generally, it's better to use this feature when you need it, rather than trying to use it where you might not: when you do need it, it tends to be fairly obvious when you realise that the only other way is to perform the same operation over and over again.

Another nice feature of helper state is that it's engine-only, which means, no matter how large your helper state, it won't impact the size of your final binary! (Sometimes, you might even want to put the contents of every single blog post in that helper state, if you need to read each file in build paths anyway.) Note however that it will be serialized to a file and read for request state in apps using a server, so it shouldn't be *massive*, or this process might become a little slow, which would jeopardise load times (again, this is only a concern with very large state in non-exported apps).

*Note: internally, this example also functions as an important canary on the feature of Perseus that is most likely to be broken by code changes: index-level build paths. If any part of these tests fails with a 'timeout waiting on condition' error, there is almost certainly a missing `/` strip somewhere in the code. This is what we have tests for!*
