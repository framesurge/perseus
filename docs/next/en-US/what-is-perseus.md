# What is Perseus?

Perseus is a **web development framework** for the **Rust** programming language that focuses on the **state** of your app. Since there are three main ways you might approach Perseus, we'll break down each one individually here.

## You're familiar with Rust

We can obviously agree that Rust is much better than JavaScript: it's way faster, strongly-typed, has a great compiler, and a fantastic package management system. In the browser, it runs *amazingly*. This is because of [WebAssembly] (abbreviated *Wasm*), which is basically an assembly language for programs like Chrome, Firefox, etc. With it, you can compile your Rust code to run in the browser, and even access browser APIs, allowing you to display content to the user. In the past, Rust has been used with Wasm to perform things like heavy cryptography, but Perseus lets you exile JS completely, and run your whole site with Rust only.

Now, you might have come across other web development libraries and frameworks for Rust before, but there's a big difference between those two terms, so let's sort that out first. A *library* is a piece of code that you use to help you build your site. A *framework* is a mammoth of code that uses your code to build your site. Think of it like the difference between `futures::executor::block_on` and `#[tokio::main]`: one is being used by you to handle a bit of `async`, and the other is using your code to handle *all* the `async`. In the same way, a library is a great choice for when you want to build a small site, or when you want to replace just part of a site with Rust. For these kinds of things, we absolutely recommend [Sycamore], on which Perseus is based.

However, sometimes you'll need to break out the big guns. Sometimes, you'll need to render content in advance so that your users see it straight away, rather than a blank page while your Wasm boots up. Sometimes, you'll want to have  a*stateful* app. This doesn't just mean you've got buttons and forms, etc., but that you're building your app in a special kind of pattern, which Perseus is built around. Let's say you have a simple static blog: you might have a `/post` URL, under which all your posts can be found. Fundamentally, all these posts have the same structure, just with different titles, dates, tags, and contents, so you might choose to create some kind of *template* for them, and then maybe build a Markdown parser or the like to push all that into your app to create *pages*. Essentially, **template + state = page**. In Perseus, this is all handled for you, and you can't actually create pages, you can just create templates, like `/post`, and ways to render their state.

For example, for a blog, you might create a new post template with `Template::build("post")`, and then create a function that takes in some state and plugs it into a Sycamore `view! { .. }` to render some content. You might take in a `struct` containining contents, titles, tags, etc. If you then specify a function that can list the pages that this template should create (e.g. by getting all the Markdown files in a certain directory), and then another one that takes each path and generates state for it, Perseus will string it all together and give a lightning-fast app.

Beyond this, Perseus has all sorts of extra features, like inbuilt error handling systems that allow you to gracefully display error messages if state generation fails, or if your app panics, or something else like that. All you do is match an `enum ClientError`, and Perseus shows your errors to the client. Beyond that, if you want to build an app in multiple languages, Perseus will let you do it straight away: just replace the text in your code with identifiers inside the `t!()` macro, and define a map of translation IDs to text for each language you want to support. Variable interpolation is supported out of the box, and you can unleash the full power of [Fluent] for handling pluralization rules, genders, etc.

Going even further, Perseus' state generation platform is built for even the most advanced use-cases: let's say you have not a blog, but an ecommerce site selling a thousand products. Well, a thousand would actually build very quickly, so perhaps a million. Still probably looking at less than a second, but we'll go with it. Maybe you don't want to build all that at build time. Simple! Just add `.incremental_generation()` to your template definition and then...you're done. If a user goes to a produce page that doesn't exist yet, it will passed to your state generation functions, and, if it's a page that exists, they can produce the page. For any future users, that page will be cached and returned immediately. It's like building your whole app over time, on-demand. And, if you have an index of all your products, you could automatically *revalidate* that every, say, 24 hours, to make sure users have a fairly up to date listing. Or you could logic-based revalidation that checks each time whether or not there are actually any new products, before rebuilding. You could even combine the two: only check ever few hours whether or not there are new products, and, if there are, rebuild that page.

To be clear, and this is important if you aren't familiar with web development, Perseus is not a library, it's a framework. It's a giant engine into which you plug your code that will connect everything together and optimize it, producing a super-fast site that outperforms every JS framework under the sun. It might well seem like you don't need a lot of these features, and, if you don't, you can just run `perseus export` to get a series of static HTML files that you can serve to users however you like, with a simple Wasm bundle making sure whatever interactivity you have works as smoothly as possible (and it will still be unreasonably fast). If you're used to systems programming, the whole idea of a framework might seem a bit absurd, but it's very often required in web development, simply because the best experiences come from complex features, like rendering your site to HTML in advance, or caching transations, or delayable capsules that can be infinitely nested to create lazy-loaded pages, etc. Some of these are easy to implement, others are not. The point of Perseus is to let you get on with what you want to write: your app. 

If Perseus doesn't sound like your cup of tea, there are several other Rust frameworks you might like to check out: [Sycamore] is the library on which Perseus is based, if you want to keep the same sort of style; [Yew] is a very popular library/framework; and [Seed] is another. There's also [Sauron], [MoonZoon], and [Leptos], just to name a few. If you'd like to see some more in-depth comparisons between these projects, check out [the comparisons page].

## You're familiar with JavaScript, and you've know what NextJS, ReactJS, etc. mean

Alright, you're pretty familiar with what web development is, and why we tend to need frameworks to make things simple and to remove the need to write hundreds of lines of boilerplate code for features we use in every app. But you've probably got plenty of questions about Perseus.

### Why Rust?

Put simply, JS is [a bit of a mess]. It's dynamically-typed, and executed at runtime, meaning you can't really catch bugs while you're coding. Sure, an IDE helps with this by showing you squiggly red lines, but it still won't stop you from forgetting about passing a certain argument to a function. TypeScript helps with this by introducing stricter typing rules, but it's really an addition on top of already existing JavaScript, and, let's be honest, how many times have you had to search up solutions for getting your `tsconfig` to work?

[Rust], on the other hand, is generally thought of as a systems programming language, meaning it's much lower-level and closer to the hardware, letting you do things like memory management more manually. It's certainly got a much steeper learning curve, but, let's walk through a quick example. Imagine you have a variable `data` that contains a very large amount of information. Obviously, copying this is going to slow your program down, so we want to avoid that if possible. In JS, you could do something like this:

```javascript
const data = "...";
let valid = isDataValid(data);
let useful = isDataUseful(data);
```

You might not realize it, but this code could copy the whole of `data` under certain conditions, because, when you think about it, both `isDataValid()` and `isDataUseful()` need it. In fact, depending on your code's structure, JS might even implicitly copy this whole variable *twice*! This is an oversimplification, and there's a lot more going on here, but, in Rust, you have total control over this:

```rust
let data = get_data();
let valid = is_data_valid(&data);
let useful = is_data_useful(&data);
```

Here, we're passing *references* to `data` to those functions, which are like telling them where `data` can be found in memory, rather than giving them it's actual value. Again, we're oversimplifying, but the point is that Rust allows you much lower-level control over your data, and it's a compiled language, meaning you have to build your code into an executable, rather than just running it. In this stage, the compiler goes over your code with a fine-toothed comb, finding whole classes of bugs and making them impossible at runtime. And, to make things even better, *undefined behavior*, a special type of bug in C/C++/etc. (which often leads to `Segmentation fault` messages, which you might have seen before), is literally impossible in Rust, because the whole language is built on a clear boundary between *safe* code, and *unsafe* code. The latter might cause UB, and should explicitly clarify what has to be upheld for it to all work properly. Then, if code can be certain that it's upholding the necessary invariants, it can call itself safe. Basically, where the compiler can't prove that your code won't crash and burn, you explicitly have to, and there's no getting around it.

To illustrate just how powerful this model of programming is, let's take a bit of a meta-example. When we were building Perseus v0.4.0, we had to rewrite the entire Perseus core, over 12,000 lines of code. After innumerable cycles of changing some code and seeing errors pop up in the terminal, when we got all the errors fixed and the code actually compiled, the first time we ran `perseus build`, *it worked*. No logic bugs, no syntax errors, it just worked. *That* is the kind of power you get from working with Rust.

Usefully, the Rust compiler supports compiling for different *targets*, which are basically formats of machine code. Your Rust code can go into code that will run on Linux, macOS, Windows, etc. Or, it could run in the browser, through a revolutionary new technology called [WebAssembly](), abbreviated as *Wasm*. Technically, any language, like C or C++, could compile into this format, but Rust has the added guarantees of *safety*.

Oh, and did we mention that Rust is [insanely fast]?

When you combine that with Wasm, a Rust site is usually >30% faster than the equivalent site built in JavaScript, in terms of runtime performance. And, when we say >30%, we mean >90% on anything modern that's not running Safari (Apple being a bastion of implementing web standards, as usual).

With all this, Rust is the perfect language to implement a next-generation web framework in, and that's exactly what Perseus is.

### Okay, but what *is* it?

As NextJS is to ReactJS, Perseus is to [Sycamore]. Sycamore is a low-level reactive library for building websites in Rust that uses *no virtual DOM*, making it [faster than Svelte] in some cases (with improvements on the horizon to get *even faster*), and Perseus builds on these foundations to create a framework designed to make your life easier by minimizing boilerplate.

Assuming you're familiar with a few terms from the usual JS jargon about frameworks, let's run through Perseus' features. It supports static site generation (building your app to HTML before it's even running), server-side rendering (building pages at request-time based on user details, like cookies), client-side rendering (fetching data in the browser to render components), using SSG and SSR *on the same page* (which, to our knowledge, no other framework in the world supports), revalidation (allowing you to rebuild a page that was built originally at build-time), incremental generation (rendering a page at request-time the first time it's requested, and then caching it for future use so it can be returned instantly next time), and *capsules* (we'll come to those).

This is all based around *state*, because that's the focus of Perseus. Unashamedly, Perseus focuses on supporting highly complex apps with many moving parts and interconnected components. Of course, if you want to build a static blog, that's a piece of cake.

Fundamentally, Perseus boils down to a state framework, and, really, the whole idea of actually displaying content to a user is secondary. As far as Perseus is concerned, your state is generated in almost any way conceivable, it gets to the user, it's made *reactive* of its own accord (meaning, if you're coming from React, that any state you generate on the server comes to you already in a `useState()` hook), and then you can work with it however you like to display it to users. If your site isn't interactive (like a static blog), you can use unreactive state instead, no problem.

Based on this, Perseus' rendering model comes down to *templates*, which are like stencils for creating pages. For example, you might have a blog post template at the `post` URL, which would have the basic structure that all blog posts share. When you plug in the data of an individual blog post called `foo`, you get out that template, filled in with that state, to produce `post/foo`, a page.

In essence, **template + state = page**, that's the fundamental equation of Perseus.

But, we went further than this. If you're familiar with [Astro], then you'll have heard of the *islands architecture*, where you split your app into components that can individually render, hydrate, etc. Now, things are a bit different over here in Wasm-world, because things are so fast here that we don't really have to care about delaying hydration, or things like that, because it all happens in literally milliseconds. Instead, our main concern is minimizing the amount of *stuff* (i.e. HTML and Wasm) that needs to be sent to the user's browser, because that's the real bottleneck for us. So, if you split out a complex ecommerce page into, say, a *widget* (Perseus' term for islands) for each product on your home page, then your home page can load as a simple skeleton waiting for some content. It's kind of like a template waiting for state, but the pieces that need to be filled in are actual mini-pages themselves. In fact, unlike any other framework ever created, Perseus has the unique concept that **capsule + state = widget**. That's right, as a template creates pages, a capsule creates widgets, meaning you can have a `product` capsule that incrementally generates product widgets as they're requested. You can use every single rendering strategy that works for pages on widgets, and you can control exactly when they're rendered too. If you want, say, the first row of products on your website's landing page to be instantly rendered, and then the rest to be lazy-loaded in parallel, you can do that by chaning `.widget()` to `.delayed_widget()`. It's that simple.

Naturally, Perseus also comes with the usual stew of extra framework features, like internationalization out of the box that just works (translator APIs etc. are all available for you, and you can pick a really powerful one using [Fluent] or a really tiny one using JSON, with more to come), and one-command deployment to a `pkg/` folder that you put literally anywhere that runs executables. And if you want a static site, you just run `perseus export`, and you're set.

As for the Lighthouse scores, Perseus achieves 100 on desktop without even trying, and consistently about 90 on mobile. The reason for the dropoff in mobile performance is mostly because of the way mobile browsers still have to go in optimizing Wasm, but this wil improve with time, and any user on a modern smartphone will see a snappy and responsive site practically instantly. That whole idea of render-then-hydrate is baked into Perseus: your users see content straight away, and it becomes reactive a moment later.

Unfortunately, the idea of *resumability*, as pioneered by [Qwik], isn't really possible with Wasm yet, because you actually can't split a Wasm bundle into smaller pieces, you just send the whole thing to the user. While that does mean that Perseus apps are *insanely* fast when going between pages, it can mean slightly slower load times when a user first comes to your site. That said, it's still 100 on Lighthouse, so it can't be *that* bad. Even so, we're sure you've had that bad experience of loading a site and trying to press buttons that don't work, and knowing (as a developer) that it's because the site hasn't hydrated yet. Now, with Perseus, your users really won't be waiting too long for those buttons to be working, but you can enable a feature flag that holds user interactions in stasis until your app is hydrated, before automatically re-sending them, leading to a much better overall user experience. And, if you don't like it, as with most things in Perseus, you can just turn it off.

The other really cool thing about Perseus is *error handling*. A lot of JS frameworks have this concept of *error boundaries*, but still more leave all the error management to you. If JS blow up (as it frequently does), you're left to clean up on your own. In Rust, errors have to be propagated explicitly with a type called `Result`, which can either be `Ok` or `Err`. Unless a function `panic!`s, it can't rip the floor out from under you and cause everything to fail. That means Perseus can handle nearly all errors gracefully: for example, if a single widget can't render its contents properly, it will automatically render an error instead. If Perseus can'tt start up your app, but it knows the user can already see some content, it will show a popup error message instead of replacing the perfectly good static content. And, if your whole app panics, crashing and burning to the ground, Perseus gives you the opportunity to run arbitrary code (like crash analytics) as well as display a nice error message to the user. And, because Rust is strongly-typed, if you forget to explicitly handle (or not handle) a particular type of error, your app just won't compile, and you'll get a lovely error message from the compiler. Basically, it would take an alignment of cosmic rays flipping dozens of bits in your computer simultaneously, or a total browser crash, to make Perseus fail without producing an error message of some kind. We don't crash and burn a lot, but when we do, we do it in style.

## You're new to web development and Rust, welcome!

Usually, people build websites with three languages: HTML (HyperText Markup Language), CSS (Cascading Style Sheets), and JS (JavaScript). If you imagine building a bed in real life with these languages, HTML would be responsible for declaring that what you're building is a `<bed></bed>`, while you would CSS to set how rounded the corners are, what color the whole thing is, what shape, what size, etc. Finally, you would use JS to make the bed, perhaps, start playing music at a certain time in the morning to wake you up.

However, these languages are all *interpreted*, meaning the browser tries to figure out what your code does as it gets it. So, if you were to, say, make a typo in some code that you put on your website, you wouldn't know until the code just doesn't run for your users, and some part of your site breaks. Although there are ways of working around these types of errors, usually with extensions to JS like [TypeScript], they effectively bring the power of *compiled* and *typed* languages (like [Rust]) to the web, except they're just extensions, which means they don't solve a lot the underlying problems.

For example, let's say we have a variable `x` in JavaScript, which we set to be `5`. If we then change this to say the string `foo`, that's perfectly fine according to JS, but think about it: how many units of memory does it take to represent `5`? And how many to represent `foo`? The fact that these are different, and that this sort of thing is permissible in the language, means that JS has to do a whole lot of overhead work making everything work out. Sure, it can be nice to be able to set any variable to anything, and that sort of freedom can certainly be useful for rapid prototyping (one of the great appeals of conceptually similar languages, like Python), but it doesn't make for very fast code.

If, instead, you were to build your site in another programming language that's *typed* (meaning, once you set `x = 5`, it can't be anything other than a number, because the language knows exactly how much memory to allocate) and *compiled* (meaning there's a stage before code execution where your code is parsed, checked for errors, and automatically optimized, being translated from human-readable code to machine-readable instructions), it could be, at a minimum, over 30% faster than one built with JS. Also, you get much more performant continuity between platforms. For example, you can happily build your site in Rust, and your server. If you were to do that with JavaScript, then both would be *quite slow*. And, when we're talking about corporate applications, even a second slower loads can do [meaningful harm] to customer conversion.

Perseus is a framework for building complex websites and webapps in Rust, which consistently outperforms every other JS framework under the sun in benchmarks. It's based on [Sycamore], which provides underlying *reactivity* (which lets you do cool things like say "show the value of variable `x` here and update the view whenever that variable updates"), and is [faster than Svelte], one of the fastest JS frameworks, in several benchmarks. On its own, Perseus will take your code, compile it, and then add an extra stage of *building* your app, in which it looks at your code, figures out the earliest pages can be prepared for users, and prepares them. So, if you have an *about us* page that's the same for every user, and that doesn't depend on users, say, being logged in, then Perseus will automatically render that page when you build your app, meaning your users will see it more quickly when they want it.

If you're completely new to web development and Rust, explaining the rest of Perseus' features will probably not be the best thing, so we'd recommend taking a look at the [MDN] documentation for information about web dev generally, and you should read [the Rust book] (it's not too long) to get a feel for Rust. Once you've got the basics down, you should be ready to dive straight into Perseus! And, if you need some help, don't hesitate to ask on [our Discord]! Best of luck!






























Perseus is a framework for building extremely fast web apps in Rust, with a focus on the state of your app, enabling dynamic server-side state generation, request-time state alteration, time or logic-based state revalidation, and even freezing your entire app's state and thawing it later!

To most people though, none of that will make any sense, and that's the reason these docs exist! If you're familiar with [NextJS](https://nextjs.org), Perseus is like that for Rust. If you're familiar with [SvelteKit](https://kit.svelte.dev), it's that for [Sycamore](https://github.com/sycamore-rs/sycamore). If you're still scratching your head, read on!

*Note: If you're not in the mood for a lecture, there's a TL;DR at the bottom of this page!*

### Rust web development

[Rust](https://www.rust-lang.org/) is an extremely powerful programming language, but I'll leave the introduction of it [to its developers](https://www.rust-lang.org/).

[WebAssembly](https://webassembly.org) (abbreviated Wasm) is like a low-level programming language for your browser. This is revolutionary, because it allows websites and web apps to be built in programming languages other than JavaScript. Also, it's [really fast](https://medium.com/@torch2424/webassembly-is-fast-a-real-world-benchmark-of-webassembly-vs-es6-d85a23f8e193) (usually >30% faster than JS).

But developing directly for the web with Rust using something like [`web-sys`](https://docs.rs/web-sys) isn't a great experience, it's generally agreed in the web development community that developer experience and productivity are vastly improved by having a _reactive_ framework. Let's approach this from a traditional JavaScript and HTML perspective first.

Imagine you want to create a simple counter. Here's how you might do it in a non-reactive framework (again, JS and HTML here, no Rust yet):

```html
<p id="counter">0</p>
<br />
<button
    onclick="document.getElementById('counter').innerHTML = parseInt(document.getElementById('counter').innerHTML) + 1"
>
    Increment
</button>
```

If you're unfamiliar with HTML and JS, don't worry. All this does is create a paragraph with a number inside and then increment it. But the problem is clear in terms of expression: why can't we just put a variable in the paragraph and have that re-render when we increment that variable? Well, that's reactivity!

In JS, there are frameworks like [Svelte](https://svelte.dev) and [ReactJS](https://reactjs.org) that solve this problem, but they're all bound significantly by the language itself. JavaScript is slow, dynamically typed, and [a bit of a mess](https://medium.com/netscape/javascript-is-kinda-shit-im-sorry-2e973e36fec4). Like all things to do with the web, changing things is really difficult because people have already started using them, and there will always be _someone_ still using Internet Explorer, which supports almost no modern web standards at all.

[Wasm](https://webassembly.org) solves all these problems by creating a unified format that other programming languages, like Rust, can compile into for the browser environment. This makes websites safer, faster, and development more productive. The equivalent of these reactive frameworks for Rust in particular would be projects like [Sycamore](https://sycamore-rs.netlify.app), [Seed](https://seed-rs.org), and [Yew](https://yew.rs). Sycamore is the most extensible and low-level of those options, and it's more performant because it doesn't use a [virtual DOM](https://svelte.dev/blog/virtual-dom-is-pure-overhead) (link about JS rather than Rust), and so it was chosen to be the backbone of Perseus. Here's what that counter might look like in [Sycamore](https://sycamore-rs.netlify.app) (the incrementation has been moved into a new closure for convenience):

```rust
use sycamore::prelude::*;

let counter = Signal::new(0);
let increment = cloned!((counter) => move |_| counter.set(*counter.get() + 1));

view! {
    p { (counter.get()) }
    button(on:click = increment) { "Increment" }
}
```

You can learn more about Sycamore's amazing systems [here](https://sycamore-rs.netlify.app).

### This sounds good...

But there's a catch to all this: rendering. With all these approaches in Rust so far (except for a few mentioned later), all your pages are rendered _in the user's browser_. That means your users have to download your Wasm code and run it before they see anything at all on their screens. Not only does that increase your loading time ([which can drive away users](https://medium.com/@vikigreen/impact-of-slow-page-load-time-on-website-performance-40d5c9ce568a)), it reduces your search engine rankings as well.

This can be solved through _server-side rendering_ (SSR), which means that we render pages on the server and send them to the client, which means your users see something very quickly, and then it becomes _interactive_ (usable) a moment later. This is better for user retention (shorter loading times) and SEO (search engine optimization).

The traditional approach to SSR is to wait for a request for a particular page (say `/about`), and then render it on the server and send that to the client. This is what [Seed](https://seed-rs.org) (an alternative to Perseus) does. However, this means that your website's _time to first byte_ (TTFB) is slower, because the user won't even get _anything_ from the server until it has finished rendering. In times of high load, that can drive loading times up worryingly.

The solution to this is _static site generation_ (SSG), whereby your pages are rendered _at build time_, and they can be served almost instantly on any request. This approach is fantastic, and thus far widely unimplemented in Rust. The downside to this is that you don't get as much flexibility, because you have to render everything at build time. That means you don't have access to any user credentials or anything else like that. Every page you render statically has to be the same for every user.

Perseus supports SSR _and_ SSG out of the box, along with the ability to use both on the same page, rebuild pages after a certain period of time (e.g. to update a list of blog posts every 24 hours) or based on certain conditions (e.g. if the hash of a file has changed), or even to statically build pages on demand (the first request is SSR, all the rest are SSG), meaning you can get the best of every world and faster build times.

To our knowledge, the only other framework in the world right now that supports this feature set is [NextJS](https://nextjs.org) (with growing competition from [GatsbyJS](https://www.gatsbyjs.com)), which only works with JavaScript. Perseus goes above and beyond this for Wasm by supporting whole new combinations of rendering options not previously available, allowing you to create optimized websites and web apps extremely efficiently.

## What about all that state stuff?

At the beginning, we mentioned that Perseus is state-focused, which might seem a little cryptic. In Perseus, your app's *state* is the input to a template, which creates a page. This is all Perseus-specific jargon, so we'll simplify for now: imagine you've got a documentation website, like this one, and you want to have many pages of documentation that all have the same basic layout, but that just change their content between each page. With most frameworks, you can just write this code once, and then plug in all your content from a filesystem, database, etc., and then have all your final pages just generated. Perseus can do this too, and we call the code you write a *template*, which creates *pages*. The stuff that differentiates one documentation page from another is a bit of information that contains the documentation page's title, content, author, etc. In other words, *template + state = page*. Don't worry if you're not completely getting this yet, it's a little complex, and we'll explain it in much more detail later with some real code.

Perseus focuses on that idea of *state* though, and allows you to generate it in all sorts of different ways. For instance, you might want to get all that documentation content from a database when you build your app. But, that database might change pretty often, so every 24 hours or so you might want to check if a page has an update, and then rebuild it while your app's still running. Perseus makes this a breeze. Or, you might have millions of pages of documentation that take a long time to build, so you might only want to build a page the first time it's requested, and then keep it cached for future users. Perseus makes that literally one line of code. And what if you want to support your site in many different languages? You supply the translations, we'll supply the infrastructure to integrate them seamlessly into your website with around four lines of code.

What's more, Perseus makes all state reactive on the client-side, which means you can do something like this. Let's say you've got a form on your site, and you generate some default values for that form at build-time (in a few microseconds). you can set your site up easily so that any changes to the fields of the form by the user will update that state for them, and Perseus will persist it *automatically* even if they go to another page of your site. With around ten more lines of code, you can set Perseus up to cache your entire app's state, for all pages, and store it as a string (in the user's browser, in a database, anywhere!). When a user comes back, that state can be thawed out and placed right back into their browser. So yes, a user can have their progress in a form saved automatically for months with around ten lines of code from you.

And if none of that appeals, it's all entirely optional anyway! Perseus is still lightning fast and a brilliant tool for creating fantastic websites and apps without it!

## How fast is it?

[Benchmarks show](https://rawgit.com/krausest/js-framework-benchmark/master/webdriver-ts-results/table.html) that [Sycamore](https://sycamore-rs.netlify.app) is slightly faster than [Svelte](https://svelte.dev) in places, one of the fastest JS frameworks ever. Perseus uses it and [Actix Web](https://actix.rs) or [Warp](https://github.com/seanmonstar/warp) (either is supported), some of the fastest web servers in the world. Essentially, Perseus is built on the fastest tech and is itself made to be fast.

The speed of web frameworks is often measured by [Lighthouse](https://developers.google.com/web/tools/lighthouse) scores, which are scores out of 100 (higher is better) that measure a whole host of things, like _total blocking time_, _first contentful paint_, and _time to interactive_. These are then aggregated into a final score and grouped into three brackets: 0-49 (slow), 50-89 (medium), and 90-100 (fast). This website, which is built with Perseus, using [static exporting](:exporting) and [size optimizations](:deploying/size), consistently scores a 100 on desktop and above 90 for mobile. You can see this for yourself [here](https://developers.google.com/speed/pagespeed/insights/?url=https%3A%2F%2Fframesurge.sh%2Fperseus%2Fen-US%2F&tab=desktop) on Google's PageSpeed Insights tool.

<details>
<summary>Why not 100 on mobile?</summary>

The only issue that prevents Perseus from achieving a consistent perfect score on mobile is _total blocking time_, which measures the time between when the first content appears on the page and when that content is interactive. Of course, WebAssembly code is used for this part (compiled from Rust), which isn't completely optimized for yet on many mobile devices. As mobile browsers get better at parsing WebAssembly, TBT will likely decrease further from the medium range to the green range (which we see for more powerful desktop systems).

</details>

If you're interested in seeing how Perseus compares on speed and a number of other features to other frameworks, check out the [comparisons page](comparisons).

## How convenient is it?

Perseus aims to be more convenient than any other Rust web framework by taking an approach similar to that of [ReactJS](https://reactjs.org). Perseus itself is an extremely complex system consisting of many moving parts that can all be brought together to create something amazing, but the vast majority of apps don't need all that customizability, so we built a command-line interface (CLI) that handles all that complexity for you, allowing you to focus entirely on your app's code.

Basically, here's your workflow:

1. Create a new project.
2. Define your app in around 12 lines of code.
3. Code your amazing app.
4. Run `perseus export -sw` or `perseus serve -w`.
5. Change some code and watch your app live update in the browser, restoring the previous state (if you're working on a long form, what you've typed can be saved automatically, even as you change the code).

## How stable is it?

Perseus is considered reasonably stable at this point, though it still can't be recommended for _production_ usage just yet. That said, this very website is built entirely with Perseus and Sycamore, and it works pretty well!

For now though, Perseus is perfect for anything that doesn't face the wider internet, like internal tools, personal projects, or the like. Just don't use it to run a nuclear power plant, okay?

## Summary

If all that was way too long, here's a quick summary of what Perseus does and why it's useful!

-   JS is slow and a bit of a mess, [Wasm](https://webassembly.org) lets you run most programing languages, like Rust, in the browser, and is really fast
-   Doing web development without reactivity is really annoying, so [Sycamore](https://sycamore-rs.netlify.app) is great
-   Perseus lets you render your app on the server, making the client's experience _really_ fast, and adds a ton of features to make that possible, convenient, and productive (even for really complicated apps)
- Managing complex app state is made easy with Perseus, and it supports saving state to allow users to immediately return to exactly where they were (automatically!)
