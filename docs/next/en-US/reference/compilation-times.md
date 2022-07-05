# Improving Compilation Times

As you may have noticed while using Perseus, compiling even very small changes to your app takes *a long time*. This is because Perseus has a significant amount of boilerplate to make all its features work essentially seamlessly, and because Rust generally takes quite a while to compile anything.

The first step in addressing this is to figure out whether your app needs exporting or not. If not, then you should use `#[perseus::main_export]` rather than `#[perseus::main(...)]` in your `lib.rs`, and then you don't need to bring in a server integration, which should speed things up slightly. Notably, if you choose not to do this, there's no actual compile-time difference between `perseus export` and `perseus serve` (there used to be, back in the days of `.perseus/`, but those horrors are gone now). As a general rule, apps that *can* go to static files *should* go to static files, because they'll be faster. (It's also more easily possible to push static files to edge networks in deployment, which usually means faster load times for your users, depending on your hosting provider.)

The next step is pretty trivial: `rustup override set nightly`. This command will set the default toolchain in your project (you have to run that command in your proejct directory) to `nightly`, which tends to nearly cut compile times in half! In general, the nightly compiler will be much faster than the stable one, because it uses all sorts of unstable features that improve compilation times. If you want maximum assurances though, switch back to the stable compiler before running `perseus deploy` so your production app is secured against any nightly compiler bugs (which do happen) and you get the best of both worlds.

From here, we get a little more radical.
