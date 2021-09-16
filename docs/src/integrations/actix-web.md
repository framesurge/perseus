# Actix Web Integration

If you're using [Actix Web](https://actix.rs), then Perseus can automate nearly all the boilerplate of serving your app for you!

This integration provides a configuration function, which you can use to configure an existing web server to support Perseus, so you could even run something like [Diana](https://github.com/arctic-hen7/diana) on the same server!

This integration should support almost every use case of Perseus, but there may be some extremely advanced things that you'll need to go back to basics for. If that's the case, please let us know by [opening an issue]() (we want these integrations to be as powerful as possible), and in the meantime you can use the guide [here](./serving.md) to see how to set up a server without using the integrations. If you need implementation details, check out the actual source code for the integration in the [repository](https://github.com/arctic-hen7/perseus).

## Installation

You can install the Actix Web integration by adding the following to your `Cargo.toml` under the `dependencies` section:

```toml
perseus-actix-web = "0.1"
```

Note that you will still need `actix-web`, `futures`, and `perseus` itself, even in a server repository.

All Perseus integrations follow the same version format as the core library, meaning they're all updated simultaneously. This makes version management much easier, and it means that you can just install the same version of every Perseus package and not have to worry about compatibility issues.

## Usage

This is an example of a web server that only uses Perseus, but you can call `.configure()` on any existing web server. Note though that **Perseus must be configured after all other logic**, because it adds a generic handler for all remaining pages, which will break other more specific logic that comes after it.

```rust,no_run
use perseus::{FsConfigManager, SsrNode};
use perseus_actix_web::{configurer, Options};
use perseus_showcase_app::pages;
use actix_web::{HttpServer, App};
use futures::executor::block_on;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
        App::new()
			// Other server logic here
        	.configure(
				block_on(configurer(
					Options {
						index: "../app/index.html".to_string(),
						js_bundle: "../app/pkg/bundle.js".to_string(),
						wasm_bundle: "../app/pkg/perseus_showcase_app_bg.wasm".to_string(),
						templates_map: pages::get_templates_map::<SsrNode>()
					},
					FsConfigManager::new()
				))
			)
    })
    	.bind(("localhost", 8080))?
    	.run()
    	.await
}
```

When you use the integration, you'll have to define a few options to tell it what exactly to serve. Specifically, you'll need to tell it where your `index.html` file, your JS bundle, and your Wasm bundle all are. In addition, you'll need to a provide it with a template map (which you'll often define a getter function for as above).

Also, because this plugs into an existing server, you have full control over hosting options, like the port to be used!

It's worth mentioning the blocking component of this design. The function that returns the closure that actually configures your server for Perseus is asynchronous because it needs to get your render configuration and add it as data to the server (this improves performance by reducing reads), which unfortunately is an asynchronous operation. We also can't `.await` that without causing ownership errors due to Actix Web's closure structure, which means the best solution for now is to `block_on` that configuration (which won't impact performance other than in your startup times, and all that's happening is a read from a file). If you have a better solution, [PRs are welcome](https://github.com/arctic-hen7/pulls)!
