# Internationalization

Internationalization (abbreviated *i18n*) is making an app available in many languages. Perseus supports this out-of-the-box with [Fluent](https://projectfluent.org).

The approach usually taken to i18n is to use translation IDs in your code instead of natural language. For example, instead of writing `format!("Hello, {}!", name.get())`, you'd write something like `t!("greeting", {"name" => name.get()})`. This ensures that your app works well for people across the world, and is crucial for any large apps.

This section will explain how i18n works in Perseus and how to use it to make lightning-fast apps that work for people across the planet.
