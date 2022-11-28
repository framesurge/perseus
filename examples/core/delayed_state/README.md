# Delayed State Example

This example is a replica of the `state_generation` example, just using delayed state in each of the different rendering strategies, demonstrating Perseus' ability to avoid sending some state to the client to minimise page load times, instead asking the client to asynchronously fetch it after the page has been loaded. This allow syou to get very content-heavy pages to users extremely quickly.

Note that delayed state should only be used when you have an *extremely large* amount of state in a page, to the point that you can already see much slower load times. You should experiment with using delayed state to see if it actually improves the user experience, since it will likely *increase* overall loading time until the page is completely ready. Its main purpose is to simply get a skeleton of your page to the user as quickly as possible.
