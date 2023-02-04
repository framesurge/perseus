# Common pitfalls and FAQs

This page is a list of common pitfalls and FAQs in Perseus, and will be updated regularly. If you're having an issue with Perseus, check through this list to see if your problem already has a solution.

## Is Perseus ready for production?

This is a really hard question to answer completely. At this very moment, Perseus v0.3.x is stable and working, and, if you're already using it without problems, that should be *reasonably safe* for production (given it's been out for a year and it seems to work excellently). v0.4.x is *much more powerful* and much faster as well, and it recommended now for all new projects (and, if you're using v0.3.x for something non-mission-critical, we strongly recommend thatyou upgrade), but it is still in beta. In the coming weeks, v0.4.x will go 'stable', which just means that we believe it is reasonably feature-complete, and that everything we've put in works, *we think*.

All that said, both Perseus and Sycamore are still in v0.x.x, meaning neither project has yet reached a 'stable' 1.0 release. Both projects strive to make sure that no breaking changes are introduced except in bumps of the v0.**X**.x number, and both projects are actively maintained with fantastic communities behind them, so any problems you're having will probably be rapidly resolved. However, if you're looking for something with set-in-stone functionality that is totally reliable, Perseus isn't quite there yet. For personal projects and internal tools, we *absolutely* recommend Perseus, it's a great choice! But, for enterprise production applications, there is a *very small* chance of something going horribly wrong. That said, to date the Perseus project has received no reports of any production failures caused by our code, so things seem to be going pretty well!

If you'd like to use Perseus in full mission-critical production though, we would recommend waiting until v1.0.0 comes out, which will denote production-safety and stability. This will be pending the release of that version for Sycamore, as well as broader stability in Perseus (there is no timeline for this at present, though we would be looking at v1.0.0 hopefully in early-to-mid 2024).

## I'm getting JSON error messages...

If an error occurs during `perseus serve`, it's very possible that you'll get error messages in JSON, which are utterly unreadable. This is because of the way the server is run, the Perseus CLI needs a JSON output so that it can figure out where the server binary is. You can access the human-readable logs by 'snooping' on the output though, which you can do by running `perseus snoop serve` (but make sure you've run `perseus build` first).

## Cargo is putting out strange errors...

If you're getting errors along the lines of not being able to find the latest Perseus version, or you have Perseus version mismatches even though you only installed it once, you've probably got some kind of Cargo corruption. Usually, this can be fixed by running `perseus clean && cargo clean`, which will delete `dist/` and `target/` and start again from scratch.

However, sometimes you'll need to purge your system's Cargo cache, which can be done safely by running the following commands:

```shell
cd ~/.cargo
mkdir old
mv git old
mv registry old
```

That will archive the `git/` and `registry/` folders in `~/.cargo/`, which should resolve any corruptions. Then, just run `cargo build` in your project (after `perseus clean && cargo clean`) and everything should work! If not and you have no idea what's going on, feel free to ask on our [Discord server](https://discord.com/invite/GNqWYWNTdp)!

## Hydration doesn't work with X

Perseus v0.4.x uses Sycamore v0.8.x, which may still have a few very minor hydration bugs (though literally dozens have been fixed since v0.7.x), so there are a few things that won't work with it yet. In fact, as a general rule, if you're getting weird layout bugs that make absolutely no logical sense, try disabling hydration, it will often fix things at the moment. This shouldn't have any major impact on user experience or performance that's appreciable, though it *may* lower your app's Lighthouse scores. Please be sure to report your problem to [Sycamore](https://github.com/sycamore-rs/sycamore) (or Perseus if you're not sure whose fault it is, and we'll probably figure it out eventually!).

## I'm getting really weird errors with a page's `<head>`...

Alright, this can mean about a million things. There is one that could be known to be Perseus' fault though: if you go to a page in your app, then reload it, then go to another page, and then navigate *back* to the original page (using a link inside your app, *not* your browser's back button), and there are problems with the `<head>` that weren't there before, then you should disable the `cache-initial-load` feature on Perseus, since Perseus is having problems figuring out how your `<head>` works. Typically, a delimiter `<meta itemprop="__perseus_head_end">` is added to the end of the `<head>`, but if you're using a plugin that's adding anything essential after this, that will be lost on transition to the new page. Any advanced manipulation of the `<head>` at runtime could also cause this. Note that disabling this feature (which is on by default) will prevent caching of the first page the user loads, and it will have to be re-requested if they go back to it, which incurs the penalty of a network request.

## I'm getting a 'mismatched render backends' error

This is a very rare kind of error that Perseus will emit if it knows that running your app in its current state will cause undefined behavior: it's a safeguard against far worse things happening. If you're using the reference pattern of managing your templates and/or capsules, where you define them in `lazy_static!`s, and then bring those into `.template_ref()`/`.capsule_ref()`, this problem is almost certainly caused by your using the incorrect *render backend generic*. In those statics, you have to specify a concrete value for that `G: Html` you see floating around the place. You might have chosen `DomNode`, or `SsrNode`, or maybe even `HydrateNode`, but each of these is only valid sometimes! Perseus internally knows when it uses each one, and it provides a clever little type alias that can handle all this for you: `PerseusNodeType`. If you use that, this error shoudl go away, adn your app should work perfectly!

Alternately, this error can occur if you try to do something very inadvisable, like putting a widget in a `view!` that you try to `render_to_string` on the browser-side. In fact, any attempt to render to a string in the browser that uses widgets is almost certain to trigger this exact error. This is because `PerseusNodeType` automatically resolves to `DomNode`/`HydrateNode` (depending on whether or not you've enabled the `hydrate` feature) on the browser-side, because Perseus doesn't need to do any server-side rendering there (unsurprisingly). That means, when you bring in a widget that's defined as a `lazy_static!` using `PerseusNodeType`, your `View` might be a `View<SsrNode>`, but the `MY_WIDGET.widget()` function will take that `SsrNode`, hold it for a moment, and check the type of itself, which it will find to be `PerseusNodeType`. Since `DomNode != SsrNode` and `HydrateNode != SsrNode`, it will find that you're trying to use a browser-side widget in a server-side rendered view, which is a type mismatch. Normally, this sort of thing could be caught by Rust at compilation-time, but Perseus uses some transmutes internally to make it safe to use `PerseusNodeType`, as long as it lines up with the actual type of the `View` being rendered. if you try to server-side render in the browser though, the types don't line up, and Perseus has the choice of either panicking or causing undefined behavior. To maintain safety, it panics.

Note that this doesn't mean it's actually impossible to server-side render a widget on the browser-side, you can use the functional pattern to do this easily. Rather than using `MY_CAPSULE.widget()`, just use `crate::path::to::my::widget::get_capsule().widget()`, because `get_capsule()` is generic over `G: Html` meaning it will just work with Rust's normal typing system.

If you're still getting this error, and none of these solutions make sense with what you're doing, then you've possibly encountered a rather serious Perseus bug, which we'd like to know about so we can fix it! Please report it [on GitHub](https://github.com/framesurge/perseus/issues/new/choose).

## Problem binding to `http://localhost:3100`

This means another instance of Perseus is already running. The reason this talks about <http://localhost:3100> rather than port 8080 is because 3100 is where the live reload server runs by default.

## I'm getting an error about not being able to modify the panic handler?

If Perseus panics, it will output an error, but sometimes, especially if you're using `-w`, Perseus will also try to reload for new code, while in a panicking state, which will lead to *another* panic where Rust complains about Perseus trying to fix things naively itself. Basically, this is just some overzealous reloading most of the time, and it can be easily fixed by reloading the page. If you want to see what the *original* panic message was, check your browser console.

## `BorrowMut` errors

These are a very sneaky kind of error in Rust that can occur at runtime, and Perseus unfortunately uses the `RefCell`s that can cause these *a lot* internally. We believe our usage of them is perfectly sound, but bugs like this have occurred in the past. Be aware that HSR can sometimes cause these in development spontaneously, just as a result of what we think are weird browser timing race conditions, and those can be fixed by reloading the page (and they shoudl never occur in production), but any persistent `BorrowMut` errors should be reported to use right away, because, chances are, they denote a bug in Perseus.
