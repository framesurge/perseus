# Suspended state

The vast majority of state generation is handled on the engine-side in Perseus apps, but there's a way to do this kind of thing on the client-side as well, called *suspended state*. This is basically where you tell Perseus to generate a default for one or more of the fields of your state type, but to modify this reactively with an asynchronous function once the page is ready on the client-side. This could be used to, say, render content that is client-specific, but that would be too onerous to render on the engine-side. Generally, unless you're accessing browser-specific parameters, there should be no difference between the capabilities of suspended state and [request-time state](:state/request), except that the former can be faster if it takes a while to fetch the state in question (because the page is still rendered, just not all of it).

If you want to render entire sections of content in a delayed fashion, check out [delayed widgets], which are a superior solution to that particular problem.

## How is this different from Sycamore's `Suspense`?

Sycamore has a component called `Suspense` that allows you to perform asynchronous rendering, for example to fetch some data before you render. This is very conceptually similar to Perseus' suspended state system, except it's less tightly integrated with the state platform, and it actually proves totally incompatible with the Perseus build process at present. In short, anything you might do with `Suspense` can be done with suspended state instead in a way that is more Perseus-ey.

## Understanding suspended state

Suspended state has no effect on the engine-side, that's the first thing to clear up, and it also works on a field-by-field basis. You'll set it up using the `#[rx(suspense = "my_function")]` derive macro helper, which you can use to annotate a field of any state type that derives `ReactiveState` (but not `UnreactiveState`: you'll soon see why). The `my_function` in that is the name of a function that will be called, once your page is ready on the client-side, to replace whatever value was generated as a default on the engine with something more fitting. This means you still have to render *something* for these suspended fields on the engine-side, and that will be used as a fallback while the 'real' state is being fetched on the client-side.

What `my_function` will then do is be given a copy of the reactive version of *just that field*, and it will be expected to `.set()` it to whatever value it likes. This means you can't use `UnreactiveState` with suspense.

## Suspended state types

You might think you can just whack `#[rx(suspense = "my_function")]` on a field and you're done, but unfortunately it's not that simple: you need to make sure that field is compatible first. Because any kind of asynchronous suspense logic only has access to the one field it's working on, it has no way to directly modify the view. This means that, if an error occurs, it has no way to report it. Hence, Perseus mandates that any suspended fields must be wrapped in a `Result<T, E>`, where `E` is some error type. If you're certain your suspense can't fail, you can use [`SerdeInfallible`](=prelude/struct.SerdeInfallible@perseus) as the error type (which is a version of `std::convert::Infallible` that can be serialized and deserialized, not that it ever will be). This means you also have to handle any errors directly in your view logic, which enforces correct, and infinitely flexible, error handling of suspended state issues.

If you're using nested suspended state, you should use [`RxResult`](prelude/struct.RxResult@perseus) instead, which is a version of `Result` that's integrated with Perseus' reactive state system. In essence, its reactive version is an `RcSignal<Result<RcSignal<T>, E>>`, which means you can reactively set it to be an error, and you can also reactively set its `Ok` variant. Its reactive version is `RxResultRx`.

Note that you can use suspended state on nested fields without a problem, but you can't do something like have the `nested` field be suspended, as well as having the `nested.foo` field be suspended, because then you could have conflicting settings of `nested.foo`. Attempting to do this will simply not work.

## Suspended state handlers

The handler functions provided to the derive helper macro should have a signature like this:

```rust
fn my_function<'a>(cx: Scope<'a>, suspended_field: &'a MySuspendedFieldTy) -> Result<(), E>
```

Notice how this function returns a `Result<(), E>`. This is essentially a convenience: any errors returned from this will be `.set()` on the field provided, since it's guaranteed to be a result. This might seem a bit magical, and you don't have to use it if you don't want to, but it can lead to better ergonomics on occasion, especially with the `?` operator.

The `MySuspendedFieldTy` type is, given some type `T` that you set on the original field (ignoring the result wrapping it), either `RcSignal<Result<T, E>>` is your field is non-nested, or `RxResult<T, E>` if it is.

## Example

With all that over, here's an example. It may seem very intimidating at first, but that's just because there are three suspended state handlers to show you how this works with nested state. It's heavily commented, and it's recommended to read through this carefully to understand how suspended state works. This is probably the most complicated part of Perseus to use, because understanding how the state flows through it is a bit tricky (we like to think of it as being borrowed from the main system by your handler and returned with a different value, through `.set()`), so feel free to [open a GitHub discussion] or [ask on Discord] if you're having trouble understanding or using this (or any other) feature.

```rust
{{#include ../../../examples/core/suspense/src/templates/index.rs}}
```

Note `#[browser_only_fn]` here, which is the browser equivalent of `#[engine_only_fn]`.
