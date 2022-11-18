# The State Platform

One of the features Perseus proclaims most is its advanced *state platform*, which isn't the simplest concept to explain, but it forms the 'secret sauce' that makes Perseus a powerful framework. As discussed on the [core principles page](:core-principles), Perseus uses a template/page model, such that a page is the product of state going into a template, and you can generate that state in all sorts of ways. Now, let's dive into the specifics on this.

One of the most powerful features of Perseus' state platform is that it spans both the engine-side and the browser-side: you can generate state in all sorts of ways on the engine-side (see the [state generation page](:reference/state-generation)), and then, when that state gets to your pages, it's 'reactive'. But what does this actually mean? Well, let's take an example state that a page in a music app might use:

```rust
struct Song {
    title: String,
    artist: Artist,
    feats: Vec<Artist>,
    year: u32,
    album: Album,
}
struct Album {
    title: String,
    artist: Artist,
    year: u32,
    ty: AlbumType,
    cover_art_url: String,
}
enum AlbumType {
    Single,
    EP,
    Album,
}
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

Now, what matters behind the scenes is that we can turn the unreactive state you gave to Perseus into reactive state. Since we're making all the fields of the `Song` `struct` reactive, in the above example, this will involve a macro: `#[derive(ReactiveState)]`. Now, this macro is more complex than most: it takes in the `struct` you give it, and derives the `MakeRx` trait on it, which means it can be converted into some reactive type. Then, it actually *creates* a whole new `struct` called `SongPerseusRxIntermediate` (which you should never have to touch) that has all its fields wrapped in `RcSignal`s. The reason we don't just go straight to a `Signal` is because, as we mentioned earlier, Perseus caches all reactive state at the application-level, which means it has to outlive all your templates, so, for lifetime simplicity, we use `RcSignal`s.

Now, if you've worked with lifetimes long enough in Sycamore (no problem if you haven't), you'll know that this will lead to some really poor ergonomics: using `RcSignal`s, we would have to `.clone()` almost everything we want to use inside `view!`. But, this is where that macro comes to the rescue again! It creates *another* `struct` called `SongPerseusRxRef` (which you shouldn't have to touch by that name, we'll get to naming), which has all the fields of the original `Song` wrapped in `&'cx RcSignal`, where `cx` is the lifetime of the page the state is being used in. Basically, you can imagine it like this: we take unreactive state, make it reactive at the application-level, and then register it as a reference on each page it's used in when we need to, to get the best ergonomics possible.

Importantly, especially if you ever need to implement all this without the macro (e.g. if your page's state is an `enum` rather than a `struct`), the intermediate reactive type (the one with pure `RcSignal`s) implements three traits: `MakeUnrx` (which allows it to be turned back into a `Song`), `MakeRxRef` (which allows it to be turned into the final type using references), and `Freeze` (we'll get to this). The original `Song` just implements `MakeRx`, and the final reference `struct` implements `RxRef`, a simple linking trait that has no methods, but that just defines the `RxNonRef` associated type to be the intermediate type. By linking the three types together like this, Perseus can take in whichever is most ergonomically convenient and work with it! For instance, there are plenty of internal methods that have access to the intermediate type, but that need to go back to the original, and they easily can with this mechanism.

So, in the `#[template]` macro, Perseus takes in your generated, unreactive state, and checks if a reactive version has already been cached (e.g. the user has already been to this page). If there is, it'll use that, and, otherwise, it'll make the unreactive thing it was given reactive, cache that for the first time for future use, and then give a reference version to your code! Since this code is basically the same for every template, we do it with a macro to minimise the overhead.

*Note: there are plans currently to remove the `#[template]` macro entirely, eventually, though this will involve significant alterations to the Perseus core.*

Of course, you probably don't want to reference your reactive type using something like `<<Song as ::perseus::state::MakeRx>::Rx as ::perseus::state::MakeRxRef>::RxRef<'__derived_rx>;`, so you can use the `#[rx(alias = "SongRx")]` helper macro to define an alias for the final reactive reference `struct`, which takes the same lifetime as the Sycamore `Scope` of the page it's being used in.

## Freezing

Earlier, we mentioned a `Freeze` trait that the intermediate rective type implements
