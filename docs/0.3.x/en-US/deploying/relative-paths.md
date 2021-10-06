# Deploying to Relative Paths

There are many instances where you'll want to deploy a Perseus website not to the root of a domain (e.g. <https://arctic-hen7.github.io>) but to a relative path under it (e.g. <https://arctic-hen7.github.io/perseus>). This is difficult because Perseus expects all its internal assets (under the URL `/.perseus`) to be at the root of the domain. However, this is easily solved with the `PERSEUS_BASE_PATH` environment variable, which you should set to be the full URL you intend to deploy your app at.

For example, if we wanted to deploy an existing app to the URL <https://arctic-hen7.github.io/perseus> (where you're reading this right now), we'd set `PERSEUS_BASE_PATH=https://arctic-hen7.github.io/perseus` before running `perseus export` (note that relative path prefixing is designed for exported apps, though it could be used for apps run with `perseus serve` as well in theory). This will tell Perseus where to expect things to be, and it will also automatically set your apps _base URI_ with the HTML `<base>` tag (if you're familiar with this, don't worry about trailing slashes, Perseus does this for you).

## Code Changes

If you want to deploy a Perseus app to a relative path, the only code changes you need to make are to your links, which should be made _relative_ rather than _absolute_. For example, you linked to `/about` before, now you would link to `about`. Don't worry about doing this, because the HTML `<base>` tag is designed to prepend your base path to this automatically, effectively turning your relative path into an absolute one. You can read more about this [on MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base).
