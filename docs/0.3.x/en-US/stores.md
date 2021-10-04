# Stores

Perseus has a very unique system of managing data as far as frameworks go, because it sometimes needs to change files it generated at build-time. This would be fine on an old-school server where you control the filesystem, but many modern hosting providers have read-only filesystems, which makes working with Perseus problematic.

As a solution to this, Perseus divides its data storage into two types: *mutable* (possibly changing at runtime) and *immutable* (never changing after build-time). These correspond to two types in Perseus: `ImmutableStore` (a `struct`) and `MutableStore` (a `trait`).

## Immutable Stores

An immutable store is used for all data that won't be changed after it's initially created, like for data about pages that are pre-rendered at build-time that don't revalidate. Because it's read-only after the build process, it can be used on a hosting provider with a read-only filesystem without problems, and so immutable stores always work on the filesystem. The only customizable part of them is the path they write to, which can be set with the `dist_path` parameter in the `define_app!` macro (by default it's `dist/`, relative to `.perseus/`).

## Mutable Stores

There are two classes of data that need to be modified at runtime in Perseus: data about pages that can revalidate, and pages cached after incremental generation. There are many ways to deploy a Perseus app, and some involve a read-only filesystem, in which case you'll likely want to use an external database or the like for mutable data. Perseus makes this as easy as possible by making `MutableStore` a `trait` with two simple methods: `read` and `write`. You can see more details in the [API docs](https://docs.rs/perseus).

By default, Perseus will use `FsMutableStore`, an implementation of `MutableStore` that uses the filesystem at the given path, which is set to `.perseus/dist/mutable/` by default. On hosting providers where you can write to the filesystem and have your changes reliably persist, you can leave this as is. But, if you're using a provider like Netlify, which imposes the restriction of a read-only filesystem, you'll need to implement the `MutableStore` `trait` yourself for a database or the like.

### Performance of External Mutable Stores

There are significant downsides to using a non-filesystem mutable store in terms of performance, especially if that store is on another server. Remember that every request to an incrementally-generated page or a page that revalidates will use this external store, which means the request has to travel to the server, to the store, from the store, and from the server, twice as many trips as if the store was on the filesystem. For this reason, Perseus splits data storage into mutable and immutable stores, allowing you to incur these performance costs from the smallest amount of data possible. In previous versions, these stores were combined together, which was problematic for large-scale deployments.
