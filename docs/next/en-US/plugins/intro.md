# Plugins

Perseus is extremely versatile, but there are some cases where is needs to be modified a little under the hood to do something very advanced. For example, as you'll learn [here](:deploying/size), the common need for applying size optimizations requires modifying a file in the `.perseus/` directory, which requires [ejecting](:ejecting). This is a laborious process, and makes updating difficult, so Perseus support a system of *plugins* to automatically apply common modifications under the hood!

First, a little bit of background. The `.perseus/` directory contains what's called the Perseus engine, which is basically the core of your app. The code you write is actually imported by this and used to invoke various methods from the `perseus` crate. If you had to build all this yourself, it would take a very long time! Because this directory can be automatically generated though, there's no need to check it into version control (like Git). However, this becomes problematic if you then want to change even a single file inside, because you'll then need to commit the whole directory, which can be unwieldy. More importantly, when updates come along that involve changes to that directory, you'll either have to delete it and re-apply your modifications to the updated directory, or apply the updates manually, either of which is overly tedious for simple cases.

Perseus has plugins to help with this. At various points in the engine, plugins have what are called *actions* that they can take. Those actions are then executed by the engine at the appropriate time. For example, if a plugin needed to run some code before a Perseus app initialized, it could do that by taking a particular action, and then the engine would execute that action just before the app initialized.

Perseus has two types of plugins that behave quite differently, and have different capabilities. This distinction is created due to compatibility issues. For example, if a plugin wants to replace the [immutable store](:stores/immutable) of an app with some new configuration, we can't let multiple plugins do this, otherwise we'd potentially have multiple conflicting immutable stores and no way to choose which one to use. In this case, we need a *control plugin*, which can take *control actions*, which can only be taken by one plugin at a time. Conversely, some actions can be taken by as many plugins as needed, like running some arbitrary code before the app initializes, and a *functional plugin* can be used in those cases. Both types of plugins are described in further detail, along with the actions they can take, in the following sections.