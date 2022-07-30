# Fetching Example

This examples demonstrates how to interact with a server with Perseus and fetch data on both the server and in the browser. Specifically, this uses `reqwest` on the server-side (with Perseus' inbuilt caching mechanism to speed up development builds) and `reqwasm` on the client-side.

On the server-side, this will simply fetch the server's IP address, and on the client-side it will fetch the message at `/.perseus/static/message.txt`, automatically served by Perseus from `static/message.txt`. The reason for fetching a file on the same site is to prevent issues with CORS, as documented in the book.
