# Live Reloading

When you develop a Perseus app, you'll usually be using `-w` on the command you're running (e.g. `perseus serve -w`, `perseus export -sw`), which will make the CLI watch your code and rebuild your app when it changes. In addition to that, Perseus will automatically reload any browser sessions that are connected to your app, meaning you can just change your code and save the file, and then your updated app will be ready for you!

In production of course, the code of your app won't change, so Perseus disables live reloading automatically when you build for production (e.g. with `perseus deploy`).

If you find that live reloading isn't to your liking, you can disable it by adding `default-features = false` to the `perseus` dependency in your `Cargo.toml`, which will disable all default features, including live reloading. Currently, it's the only default feature (along with [HSR](:reference/state/hsr), which depends on it), so you don't need to enable any other features after doing this.

To achieve live reloading, Perseus runs a server at <http://localhost:3100>, though, if you have something else on this port, this would be problematic. You can change the port by setting the `PERSEUS_RELOAD_SERVER_PORT` environment variable (and `PERSEUS_RELOAD_SERVER_HOST` also exists if you need to change the host).
