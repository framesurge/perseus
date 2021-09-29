# Server Deployment

If your app uses rendering strategies that need a server, you won't be able to export your app to purely static files, and so you'll need to host the Perseus server itself.

You can prepare your production server by running `perseus deploy`, which will create a new directory called `pkg/`, which will contain the standalone binary and everything needed to run it. You should then upload this file to your server and set the `PERSEUS_STANDALONE` environment variable to `true` so that Perseus expects a standalone binary configuration. Note that this process will vary depending on your hosting provider.

## Hosting Providers

Perseus is quite unique because it **modifies its build artifacts at runtime**, which makes a number of modern hosting providers unsuitable for deployment, as they'll impose the restriction that the filesystem of your 'server' can't be written to, only read from (understandable in most cases). Again, if you aren't using any request-time strategies (so if you're only using *build state* and/or *build paths*), then you should be using [static exporting](../exporting.md) instead to avoid this entire category of problems.

It's this quirk of Perseus that makes it incredibly powerful, but it can also make it annoying to select a hosting provider. If you've come from using [NextJS](https://nextjs.org), you may be surprised at this, as [Vercel](https://vercel.com) exists to serve apps made with that framework, which also has this issue. However, this scenario is mainly due to the fact that NextJS made a company out of hosting their apps, which is currently not a plan for Perseus!

Essentially, you'll be able to host a Perseus server on any platform that supports writing to a filesystem, which is most old-school server solutions, or any where you get an actual virtual machine to work with.

<details>
<summary>Why can't I run Perseus without writing to the filesystem?</summary>

In theory, you can. This is not for the faint of of heart though, as your app may well start experiencing very strange issues. There are three strategies that run at runtime in Perseus:

- *Request state* -- doesn't write to the filesystem
- *Revalidation* -- uses the filesystem extensively for noting times for next revalidation
- *Incremental generation* -- caches pages built on demand on the filesystem for better performance in future

Based on that, if you app doesn't use revalidation, you can actually run the Perseus server on a provider that doesn't let you mutate the filesystem, but any incrementally generated pages won't be cached properly, and they'll be effectively rendered again for every user, which will make your site slower on those pages (reduces your time to first byte, to be specific).

If your app does use revalidation, running on a read-only filesystem will lead to pages that revalidate being re-rendered every single time they're visited, because the new revalidation times won't be written properly. This can be disastrous for performance.

Also, some hosting providers may clear any writes to the filesystem after a certain period (common strategy), in which case you could run a Perseus server that doesn't use revalidation and incur the performance hit of incremental generation not working properly. However, if the provider doesn't permit writing to the filesystem at all (as in throws an error if you try it), Perseus will basically blow up in your face, and any time it tries to run revalidation or incremental generation, it will throw an error (which it can't recover from).

So basically, be careful with your hosting provider, and make sure you're not using revalidation if you choose to attempt this course of action!

</details>

## Avoiding Filesystem Writes

You may have noticed earlier on in the book [this section](../config-managers.md) on config managers, which allow you to store configuration anywhere you like, for example in a database, rather than on an immutable filesystem. You can definitely use these to make Perseus work on a read-only filesystem, but you will incur *significant* performance hits from having to fetch files from an external system so frequently.

With just a client and a server, you have to send every request to the server, and then back to the client. There are 2^1 requests. But with configuration managers and an external database, every request also has to go to and from the database, which means you have 2^2 requests. In other words, everything is literally twice as slow. Hence, this approach is not recommended.

<details>
<summary>So why do configuration managers exist then?</summary>

Mostly so you can use alternative setups of Perseus, they aren't really intended for avoiding writing to the filesystem altogether. The reason they can do so is mostly a legacy thing from v0.1.x.

</details>
