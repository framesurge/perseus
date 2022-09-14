# Initial vs. Subsequent Loads

In a Perseus app, there are two ways for a page in your app to be loaded, and it's important to understand this if you want to work with the more advanced features of Perseus. The first way is an *initial load*, which is when a user comes onto your site from an external URL (e.g. a search engine). The second is a *subsequent load*, which is when a user moves from one page on your site to another (e.g. a link on your landing page takes them to an about page).

## Initial Loads

The main thing to understand about initial loads is that they have to send the user *everything*: HTML, the Wasm bundle, etc. After this is all in the user's browser, Perseus can take over the routing process to optimize for performance by only fetching what it needs. First, we need to get everything into the user's browser though.

At the level of HTTP, an initial load request looks as you might expect. Requesting a page at `/about` requests `/about`, and the server compiles a full HTML file with everything the user needs, sending it to them. For exported apps, these files are compiled when you build your app (which is why you can't use request-time state in exporting, there's no way to update the state in the precompiled HTML before it goes to the client).

This HTML file has in it a prerendered version of the page, meaning the user will see content straight away, even though the Wasm bundle might take a moment longer to load, after which time the page will become reactive, and you can click buttons, etc.

Once this Wasm is loaded, all other links in the app are controlled by subsequent loads. Importantly, if the user goes to an external URL and then comes back, another initial load will occur (though hopefully their browser will have cached the Wasm bundle, reducing the load time to almost zero).

One caveat to all this is if i18n is being used, in which case there's unfortunately no way for the server to reliably know which language a page should be returned in. If the user requested `/en-US/about`, no problem, but if they just gave us `/about`, we need to send them a script to figure out their locale. Specifically, this comes in a blank HTML page that includes the Wasm bundle, which will then detect the locale and move ahead with rendering.

Unfortunately, this approach does lead to a moment of having a blank screen before the Wasm bundle has loaded, something that we aim to resolve in the longer-term.

## Subsequent Loads

Once the user's browser has the Wasm bundle, every time they go to a new page, we don't need to fetch that bundle again, or a whole lot actually. We don't even need the HTML scaffold --- just the page's HTML content, its `<head>`, and its state. While you may see a transition from, say, `/` to `/about`, in reality that's just superficial, and no request to `/about` has been made. In fact, a request to somewhere in `/.perseus/` has been made, which will return a JSON object with exactly what we need, minimizing load times between pages, and meaning your browser has to do no more work. From its perspective, we haven't actually moved to a new page.

This is the approach of *single-page apps*, which aren't really just one page, but they use a routing approach like this for performance. Unfortunately, SPAs have a whole host of other problems caused by this routing, all of which Perseus ims to solve. If you find any problems with our subsequent loads system, please [open an issue](https://github.com/arctic-hen7/perseus/issues/new/choose)!

*Note: currently, scroll positions are not preserved by the subsequent load system, though this is an upstream issue in Sycamore currently being worked on.*
