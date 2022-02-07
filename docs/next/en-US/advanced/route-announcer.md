# Route Announcer

Perseus uses a routing system separate to the browser's, as is typical of SPA and hybrid frameworks. However, while this means we download fewer resources for each page transition, this does mean that screen readers often won't know when a page change occurs. Of course, this is catastrophic for accessibility for vision-impaired users, so Perseus follows the example set by other frameworks and uses a *route announcer*. This is essentially just a glorified `<p>` element with the ID `__perseus_route_announcer` (so that you can make modifications to it imperatively if necessary) that's updated to tell the user the title of the current page.

When a user enters a session in your app (i.e. when they open your app from a link that's not inside your app), the browser can announce the page to any screen readers as usual, so the route announcer will start empty. However, on every subsequent page load in your app (all of which will use Perseus' custom router), the route announcer will be updated to declare the title of the page as it can figure it out. It does so in this order:

1. The `<title>` element of the page.
2. The first `<h1>` element on the page.
3. The page's URL (e.g. `/en-US/about`).

This prioritization is the same as used by NextJS, and Perseus' route announcer is heavily based on NextJS'.

Notably, the route announcer is invisible to the naked eye, and it will only 'appear' through a screen reader. This is achieved through some special styling optimized for displaying this kind of text, again inspired by NextJS' router announcer (which has been proven to work very well in long-term production).
