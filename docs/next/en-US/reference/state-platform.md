# The State Platform

One of the features Perseus proclaims most is its advanced *state platform*, which isn't the simplest concept to explain, but it forms the 'secret sauce' that makes Perseus so powerful. As discussed on the [core principles page](:core-principles), Perseus uses a template/page model, such that a page is the product of state going into a template, and you can generate that state in all sorts of ways. Now, let's dive into the specifics on this.

One of the most powerful features of Perseus' state platform is that it spans both the engine-side and the browser-side: you can generate state in all sorts of ways on the engine-side (see the [state generation page](:reference/state-generation)), and then, when that state gets to your pages, it's 'reactive'. But what does this actually mean? Well, let's take an example state that a page in a music app might use:

```rust
#[derive(Serialize, Deserialize, ReactiveState)]
struct Song {
    title: String,
    #[rx(nested)]
    artist: Artist,
    year: u32,
    #[rx(nested)]
    album: Album,
}
#[derive(Serialize, Deserialize, ReactiveState)]
struct Album {
    title: String,
    #[rx(nested)]
    artist: Artist,
    year: u32,
    ty: AlbumType,
    cover_art_url: String,
}
#[derive(Serialize, Deserialize)]
enum AlbumType {
    Single,
    EP,
    Album,
}
#[derive(Serialize, Deserialize, ReactiveState)]
struct Artist {
    name: String,
    bio: String,
    profile_pic_url: String,
}
```

Now, this is pretty complex for an example, and rightly so, we're going to dive into exactly how a real app might use Perseus' reactive state platform! Note that this will be a bit of a contrived example, since you probably wouldn't need *reactive* state in a music app, but that means we can use this example for both reactive and unreactive state!

## State Generation

The first step in all this is actually getting some instances of this state, since we can't do anything with the state unless we know what it is! Above, we've defined a schema for it, but we need some actual values in there now. If we imagine there's a database of everything we need, we could use one of Perseus' many [state generation strategies](:reference/state-generation) to get that state at build-time, or even incrementally as users visit certain URLs, getting state as necessary. Since there's a [whole separate page](:reference/state-generation) on this, we'll leave it there for this, just imagine we've somehow gotten instances of our state into Perseus. Note that this stage will also involve defining all the paths under the URL `/song` that we want to create.

## Passing State to a Template

Now that we have our many states (one for each song), we need to pass it to our `song` template, and use it to generate a number of pages. **This entire stage is automatic, and occurs behind the scenes.** In essence, Perseus will take in all the paths you've told it about, and it will get the state for those in parallel (e.g. you read a database to tell it about all the songs, and then it fetches each one and gets its state), building all the pages you need. Now, obviously, this involves sending the state you've generated to a page (we'll focus on just one page from now on, for simplicity), so how does this happen?

Well, when you generated state, you generated an instance of `Song`, but, if we want our state to be *reactive*, then we'll have to do better. Reactive state is state that you can call `.get()` and `.set()` on. The most obvious usage of it is inside a form: let's say you're building a user interface that involves the user inputting some values, well, you could use Sycamore's `bind:value` on each of the `input` elements to store the state of each input reactively. But, rather than creating all the variables to do this inside your template, you can accept these as state, like so:

```rust
#[template]
fn my_template<'a, G: Html>(cx: Scope<'a>, state: PageStateRx<'a>) -> View<G> {
    view! { cx,
        form {
            input(bind:value = state.name, placeholder = "Name")
            input(bind:value = state.email, placeholder = "Email")
            // ...
        }
    }
}
```

See what we mean? It's much more convenient if every single one of the fields of `state` is *reactive*, meaning it's wrapped in a Sycamore `Signal`. (If you haven't read the Sycamore docs yet, now would be a good time!) Otherwise, you'd have to create all the `Signal`s you need at the start of your template function.

But this isn't just for convenience, it also serves a practical function: Perseus automatically caches all reactive state internally, meaning the changes the user makes to those inputs will be reflected inside Perseus' cache. And, when they come back to that page later, *the state will be restored from the cache*, meaning the inputs are just as they left them. This means users can navigate fearlessly around any Perseus app using reactive state, without fear of losing their place.

*(It actually gets even better than this, but keep reading!)*

Now, what matters behind the scenes is that we can turn the unreactive state you gave to Perseus into reactive state. Since we're making all the fields of the `Song` `struct` reactive, in the above example, this will involve a macro: `#[derive(ReactiveState)]` (we also derive `Serialize` and `Deserialize` from Serde, since Perseus needs to send this state over a network connection from server to browser). Now, this derive macro is more complex than most: it takes in the `struct` you give it, and derives the `MakeRx` trait on it, which means it can be converted into some reactive type. Then, it actually *creates* a whole new `struct` called `SongPerseusRxIntermediate` (which you should never have to touch) that has all its fields wrapped in `RcSignal`s. The reason we don't just go straight to a `Signal` is because, as we mentioned earlier, Perseus caches all reactive state at the application-level, which means it has to outlive all your templates, so, for the lifetimes to work out, we use `RcSignal`s.

Now, if you've worked with lifetimes long enough in Sycamore (no problem if you haven't), you'll know that this will lead to some really poor ergonomics: using `RcSignal`s, we would have to `.clone()` almost everything we want to use inside `view!`. But, this is where that macro comes to the rescue again! It creates *another* `struct` called `SongPerseusRxRef` (which you shouldn't have to touch by that name, we'll get to naming), which has all the fields of the original `Song` wrapped in `&'cx RcSignal`, where `cx` is the lifetime of the page the state is being used in. Basically, you can imagine it like this: we take unreactive state, make it reactive at the application-level, and then register it as a reference on each page it's used in when we need to, to get the best ergonomics possible.

But, if it encounters an `#[rx(nested)]` helper macro on any of your fields, it will assume the type of that field also has `ReactiveState` derived, and it will automatically use the reactive version of it. In our example above, this means we wouldn't be getting the artist of a song's name by going `song.artist.get().name`, we could use the far better `song.artist.name.get()`! This improves ergonomics substantially in complex apps (while also allowing *very* fine-grained state control).

[TODO implementation on `Vec` etc.]

Importantly, especially if you ever need to implement all this without the macro (e.g. if your page's state is an `enum` rather than a `struct`), the intermediate reactive type (the one with pure `RcSignal`s) implements three traits: `MakeUnrx` (which allows it to be turned back into a `Song`), `MakeRxRef` (which allows it to be turned into the final type using references), and `Freeze` (we'll get to this). The original `Song` just implements `MakeRx`, and the final reference `struct` implements `RxRef`, a simple linking trait that has no methods, but that just defines the `RxNonRef` associated type to be the intermediate type. By linking the three types together like this, Perseus can take in whichever is most ergonomically convenient and work with it! For instance, there are plenty of internal methods that have access to the intermediate type, but that need to go back to the original, and they easily can with this mechanism.

So, in the `#[template]` macro, Perseus takes in your generated, unreactive state, and checks if a reactive version has already been cached (e.g. the user has already been to this page). If there is, it'll use that, and, otherwise, it'll make the unreactive thing it was given reactive, cache that for the first time for future use, and then give a reference version to your code! Since this code is basically the same for every template, we do it with a macro to minimize the overhead.

*Note: there are plans currently to remove the `#[template]` macro entirely, eventually, though this will involve significant alterations to the Perseus core.*

Of course, you probably don't want to reference your reactive type using something like `<<Song as ::perseus::state::MakeRx>::Rx as ::perseus::state::MakeRxRef>::RxRef<'__derived_rx>;`, so you can use the `#[rx(alias = "SongRx")]` helper macro to define an alias for the final reactive reference `struct`, which takes the same lifetime as the Sycamore `Scope` of the page it's being used in.

## Unreactive State

The other thing we could do is have out song state be unreactive, since, after all, it's pretty unlikely that the user is going to be renaming a song inside our music app (remember that the `.set()` method simply changes the state locally, it doesn't change anything on the engine-side or in a database, unless you code that yourself).

To do this, we would remove all those `#[rx(nested)]` helper macros, and simply change `ReactiveState` to `UnreactiveState` in that `#[derive(...)]` call at the top. (We also wouldn't need to derive `ReactiveState` or `UnreactiveState` on anything other than `Song`). Importantly, you'll also need to change `#[template]` to `#[template(unreactive)]` in the function you're using to render `Song`s.

Now, you're probably wondering why on earth we have to specially derive `UnreactiveState`, when we're just going to get the exact same thing as we generated! Well, your type still has implement the special `Freeze` trait, and the Perseus state platform is built for storing explicitly reactive state. So, what that `UnreactiveState` derive macro actually does is basically exactly the same as the `ReactiveState` macro, except, rather than wrapping your fields in `Signal`s, it uses a special `UnreactiveState` wrapper, which basically makes your state *look* reactive to Perseus, but, when you use `#[template(unreactive)]`, it can know to get rid of those wrappers and give you the original type you generated.

## Freezing

Earlier, we mentioned a `Freeze` trait that the intermediate reactive type implements, which is the core of Perseus' unique *state freezing* system. Up to now, we have the ability with Perseus to let users go between pages and have the state of each page stored perfectly, as long as they're still on the site. Of course, once they leave the site, or reload the page, that will all be lost, but what if we could preserve it somehow?

Well, conveniently, all the state in a Perseus app has to be both `Serialize` and `Deserialize`, since it needs to be turned into a `String` to be sent from the server to the user's browser. But, what if we took all the intermediate reactive types in an app, converted them back into their unreactive versions, and serialized those to `String`s? What if we added some internal Perseus state, any global state, and the current route? Put that all together as a JSON object, and you would have a `String` representation of the *exact* state of an app, from a user's perspective.

*That* is what the `Freeze` trait enables. As explained above, to achieve this, you would need to take each intermediate reactive type, turn it into its unreactive version, and serialize it to a `String`. To allow flexibility in this, Perseus requires such intermediate types to implement `Freeze`, which just has the `.freeze()` method, which simply produces a `String` representation of that type.

If you want to see how you can freeze and entire Perseus app, check out [this example](https://github.com/framesurge/perseus/tree/main/examples/core/freezing_and_thawing). Alternately, take a look at [this one](https://github.com/framesurge/perseus/tree/main/examples/core/idb_freezing) to see how you can easily store the resulting frozen app to IndexedDB (a native in-browser storage system)!

Of course, if we can freeze, we need to be able to *thaw* too, right? Well, Perseus makes this pretty much trivial, since you can register a frozen app `String` that will be progressively unwrapped as necessary. In essence, rather than restoring the whole state of the app at once, Perseus simply restores the internal state and takes the user to whatever route they were on at the last freeze, and then restores each page's state on-demand: we leave it frozen until the user goes to that exact page. The same thing applies to the global state: it will only be thawed when it's first used. That not only mitigates the need for you to specify all your state types to a single thaw command, but it also means that any page states that are no longer valid can be silently ignored (e.g. if the app's code has changed since the freeze). This means users get the best replication of the previous state possible, even if the thaw is years later.

As a side note, this is exactly how Perseus' HSR (hot state reloading) system works! In JavaScript frameworks, you can split up all your JS files into many small *chunks*, and then, when you as a developer change some code and want to see the result, the framework just reloads the necessary chunks, meaning most things should stay the same. Since we can't chunk Wasm files (yet), Perseus just freezes your app's state to IndexedDB, and restores it after a page reload. From your perspective, the page has reloaded to exactly the same place. When you're fifteen pages into a login form and trying to realign a button, making sure you don't have to go back to beginning of that flow whenever you change some code becomes pretty useful! That said, you can easily disable HSR by turning off the default feature flag `hsr`.
