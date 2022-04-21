use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, AttributeArgs, Block, FnArg, Generics, Ident, Item, ItemFn, NestedMeta, PatType,
    Result, ReturnType, Type, TypeTuple, Visibility,
};

/// A function that can be wrapped in the Perseus test sub-harness.
pub struct TemplateFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// The arguments for custom properties and a global state, both of which are optional. (But global state needs custom properties, which can be a dummy `struct`.)
    pub args: Vec<FnArg>,
    /// The visibility of the function.
    pub vis: Visibility,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// The actual name of the function.
    pub name: Ident,
    /// The return type of the function.
    pub return_type: Box<Type>,
    /// Any generics the function takes (should be one for the Sycamore `GenericNode`).
    pub generics: Generics,
}
impl Parse for TemplateFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => {
                let ItemFn {
                    attrs,
                    vis,
                    sig,
                    block,
                } = func;
                // Validate each part of this function to make sure it fulfills the requirements
                // Mustn't be async
                if sig.asyncness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.asyncness,
                        "templates cannot be asynchronous",
                    ));
                }
                // Can't be const
                if sig.constness.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.constness,
                        "const functions can't be used as templates",
                    ));
                }
                // Can't be external
                if sig.abi.is_some() {
                    return Err(syn::Error::new_spanned(
                        sig.abi,
                        "external functions can't be used as templates",
                    ));
                }
                // Must have an explicit return type
                let return_type = match sig.output {
                    ReturnType::Default => {
                        return Err(syn::Error::new_spanned(
                            sig,
                            "template functions can't return default/inferred type",
                        ))
                    }
                    ReturnType::Type(_, ty) => ty,
                };
                let mut args = Vec::new();
                for arg in sig.inputs.iter() {
                    // We don't care what the type is, as long as it's not `self`
                    if let FnArg::Receiver(arg) = arg {
                        return Err(syn::Error::new_spanned(arg, "templates can't take `self`"));
                    }
                    args.push(arg.clone())
                }
                // We can have anywhere between 0 and 2 arguments
                if args.len() > 2 {
                    return Err(syn::Error::new_spanned(&args[2], "template functions accept a maximum of two arguments (one for custom properties and antoher for global state, both optional)"));
                }

                Ok(Self {
                    block,
                    args,
                    vis,
                    attrs,
                    name: sig.ident,
                    return_type,
                    generics: sig.generics,
                })
            }
            item => Err(syn::Error::new_spanned(
                item,
                "only funtions can be used as templates",
            )),
        }
    }
}

/// Gets the code fragment used to support live reloading and HSR.
// This is also used by the normal `#[template(...)]` macro
pub fn get_live_reload_frag() -> TokenStream {
    #[cfg(all(feature = "hsr", debug_assertions))]
    let hsr_frag = quote! {
        ::perseus::state::hsr_freeze(render_ctx).await;
    };
    #[cfg(not(all(feature = "hsr", debug_assertions)))]
    #[allow(unused_variables)]
    let hsr_frag = quote!();

    #[cfg(all(feature = "live-reload", debug_assertions))]
    let live_reload_frag = quote! {{
        use ::sycamore::prelude::cloned; // Pending sycamore-rs/sycamore#339
        let render_ctx = ::perseus::get_render_ctx!();
        // Listen to the live reload indicator and reload when required
        let indic = render_ctx.live_reload_indicator.clone();
        let mut is_first = true;
        ::sycamore::prelude::create_effect(cloned!(indic, render_ctx => move || {
            let _ = indic.get(); // This is a flip-flop, we don't care about the value
            // This will be triggered on initialization as well, which would give us a reload loop
            if !is_first {
                // Conveniently, Perseus re-exports `wasm_bindgen_futures::spawn_local`!
                ::perseus::spawn_local(cloned!(render_ctx => async move {
                    #hsr_frag

                    ::perseus::state::force_reload();
                    // We shouldn't ever get here unless there was an error, the entire page will be fully reloaded
                }))
            } else {
                is_first = false;
            }
        }));
    }};
    #[cfg(not(all(feature = "live-reload", debug_assertions)))]
    let live_reload_frag = quote!();

    live_reload_frag
}

/// Gets the code fragment used to support HSR thawing. This MUST be prefixed by a `#[cfg(target_arch = "wasm32")]`.
pub fn get_hsr_thaw_frag() -> TokenStream {
    #[cfg(all(feature = "hsr", debug_assertions))]
    let hsr_thaw_frag = quote! {{
        use ::sycamore::prelude::cloned; // Pending sycamore-rs/sycamore#339
        let mut render_ctx = ::perseus::get_render_ctx!();
        ::perseus::spawn_local(cloned!(render_ctx => async move {
            // We need to make sure we don't run this more than once, because that would lead to a loop
            // It also shouldn't run on any pages after the initial load
            if render_ctx.is_first.get() {
                render_ctx.is_first.set(false);
                ::perseus::state::hsr_thaw(render_ctx).await;
            }
        }));
    }};
    // If HSR is disabled, there'll still be a Wasm-gate, which means we have to give it something to gate (or it'll gate the code after it, which is very bad!)
    #[cfg(not(all(feature = "hsr", debug_assertions)))]
    let hsr_thaw_frag = quote!({});

    hsr_thaw_frag
}

pub fn template_impl(input: TemplateFn, attr_args: AttributeArgs) -> TokenStream {
    let TemplateFn {
        block,
        // We know that these are all typed (none are `self`)
        args: fn_args,
        generics,
        vis,
        attrs,
        name,
        return_type,
    } = input;

    // We want either one or two arguments
    if attr_args.len() > 1 {
        return quote!(compile_error!("this macro takes one optional argument"));
    }
    // This is optional (we'll use `G` as the default if it's not provided)
    let type_param = match &attr_args.get(0) {
        Some(NestedMeta::Meta(meta)) if meta.path().get_ident().is_some() => {
            meta.path().get_ident().unwrap().clone()
        }
        Some(nested_meta) => {
            return syn::Error::new_spanned(
                nested_meta,
                "optional second argument must be a type parameter identifier if it's provided",
            )
            .to_compile_error()
        }
        None => Ident::new("G", Span::call_site()),
    };

    // Set up a code fragment for responding to live reload events
    let live_reload_frag = get_live_reload_frag();
    let hsr_thaw_frag = get_hsr_thaw_frag();

    // We create a wrapper function that can be easily provided to `.template()` that does deserialization automatically if needed
    // This is dependent on what arguments the template takes
    if fn_args.len() == 2 {
        // There's an argument for page properties that needs to have state extracted, so the wrapper will deserialize it
        // We'll also make it reactive and add it to the page state store
        let state_arg = &fn_args[0];
        let rx_props_ty = match state_arg {
            FnArg::Typed(PatType { ty, .. }) => ty,
            FnArg::Receiver(_) => unreachable!(),
        };
        // There's also a second argument for the global state, which we'll deserialize and make global if it's not already (aka. if any other pages have loaded before this one)
        // Sycamore won't let us have more than one argument to a component though, so we sneakily extract it and literally construct it as a variable (this should be fine?)
        let global_state_arg = &fn_args[1];
        let (global_state_arg_pat, global_state_rx) = match global_state_arg {
            FnArg::Typed(PatType { pat, ty, .. }) => (pat, ty),
            FnArg::Receiver(_) => unreachable!(),
        };
        let name_string = name.to_string();
        // Handle the case in which the template is just using global state and the first argument is the unit type
        // That's represented for Syn as a typle with no elements
        match &**rx_props_ty {
            // This template takes dummy state and global state
            Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty() => quote! {
                #vis fn #name<G: ::sycamore::prelude::Html>(props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                    use ::perseus::state::MakeRx;

                    let mut render_ctx = ::perseus::get_render_ctx!();
                    // Get the frozen or active global state (the render context manages thawing preferences)
                    // This isn't completely pointless, this method mutates as well to set up the global state as appropriate
                    // If there's no active or frozen global state, then we'll fall back to the generated one from the server (which we know will be there, since if this is `None` we must be
                    // the first page to access the global state).
                    if render_ctx.get_active_or_frozen_global_state::<#global_state_rx>().is_none() {
                        // Because this came from the server, we assume it's valid
                        render_ctx.register_global_state_str::<#global_state_rx>(&props.global_state.unwrap()).unwrap();
                    }

                    #live_reload_frag

                    #[cfg(target_arch = "wasm32")]
                    #hsr_thaw_frag

                    // The user's function
                    // We know this won't be async because Sycamore doesn't allow that
                    #(#attrs)*
                    #[::sycamore::component(PerseusPage<#type_param>)]
                    fn #name #generics(#state_arg) -> #return_type {
                        let #global_state_arg_pat: #global_state_rx = {
                            let global_state = ::perseus::get_render_ctx!().global_state.0;
                            let global_state = global_state.borrow();
                            // We can guarantee that it will downcast correctly now, because we'll only invoke the component from this function, which sets up the global state correctly
                            let global_state_ref = global_state.as_any().downcast_ref::<#global_state_rx>().unwrap();
                            (*global_state_ref).clone()
                        };
                        #block
                    }
                    ::sycamore::prelude::view! {
                        PerseusPage(())
                    }
                }
            },
            // This template takes its own state and global state
            _ => quote! {
                #vis fn #name<G: ::sycamore::prelude::Html>(props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                    use ::perseus::state::MakeRx;

                    let mut render_ctx = ::perseus::get_render_ctx!();
                    // Get the frozen or active global state (the render context manages thawing preferences)
                    // This isn't completely pointless, this method mutates as well to set up the global state as appropriate
                    // If there's no active or frozen global state, then we'll fall back to the generated one from the server (which we know will be there, since if this is `None` we must be
                    // the first page to access the global state).
                    if render_ctx.get_active_or_frozen_global_state::<#global_state_rx>().is_none() {
                        // Because this came from the server, we assume it's valid
                        render_ctx.register_global_state_str::<#global_state_rx>(&props.global_state.unwrap()).unwrap();
                    }

                    #live_reload_frag

                    #[cfg(target_arch = "wasm32")]
                    #hsr_thaw_frag

                    // The user's function
                    // We know this won't be async because Sycamore doesn't allow that
                    #(#attrs)*
                    #[::sycamore::component(PerseusPage<#type_param>)]
                    fn #name #generics(#state_arg) -> #return_type {
                        let #global_state_arg_pat: #global_state_rx = {
                            let global_state = ::perseus::get_render_ctx!().global_state.0;
                            let global_state = global_state.borrow();
                            // We can guarantee that it will downcast correctly now, because we'll only invoke the component from this function, which sets up the global state correctly
                            let global_state_ref = global_state.as_any().downcast_ref::<#global_state_rx>().unwrap();
                            (*global_state_ref).clone()
                        };
                        #block
                    }
                    ::sycamore::prelude::view! {
                        PerseusPage(
                            {
                                // Check if properties of the reactive type are already in the page state store
                                // If they are, we'll use them (so state persists for templates across the whole app)
                                let mut render_ctx = ::perseus::get_render_ctx!();
                                // The render context will automatically handle prioritizing frozen or active state for us for this page as long as we have a reactive state type, which we do!
                                match render_ctx.get_active_or_frozen_page_state::<#rx_props_ty>(&props.path) {
                                    ::std::option::Option::Some(existing_state) => existing_state,
                                    // Again, frozen state has been dealt with already, so we'll fall back to generated state
                                    ::std::option::Option::None => {
                                        // Again, the render context can do the heavy lifting for us (this returns what we need, and can do type checking)
                                        // We also assume that any state we have is valid because it comes from the server
                                        // The user really should have a generation function, but if they don't then they'd get a panic, so give them a nice error message
                                        render_ctx.register_page_state_str::<#rx_props_ty>(&props.path, &props.state.unwrap_or_else(|| panic!("template `{}` takes a state, but no state generation functions were provided (please add at least one to use state)", #name_string))).unwrap()
                                    }
                                }
                            }
                        )
                    }
                }
            },
        }
    } else if fn_args.len() == 1 {
        // There's an argument for page properties that needs to have state extracted, so the wrapper will deserialize it
        // We'll also make it reactive and add it to the page state store
        let arg = &fn_args[0];
        let rx_props_ty = match arg {
            FnArg::Typed(PatType { ty, .. }) => ty,
            FnArg::Receiver(_) => unreachable!(),
        };
        let name_string = name.to_string();
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                use ::perseus::state::MakeRx;

                #[cfg(target_arch = "wasm32")]
                #hsr_thaw_frag

                #live_reload_frag

                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component(PerseusPage<#type_param>)]
                fn #name #generics(#arg) -> #return_type {
                    #block
                }
                ::sycamore::prelude::view! {
                    PerseusPage(
                        {
                            // Check if properties of the reactive type are already in the page state store
                            // If they are, we'll use them (so state persists for templates across the whole app)
                            let mut render_ctx = ::perseus::get_render_ctx!();
                            // The render context will automatically handle prioritizing frozen or active state for us for this page as long as we have a reactive state type, which we do!
                            match render_ctx.get_active_or_frozen_page_state::<#rx_props_ty>(&props.path) {
                                ::std::option::Option::Some(existing_state) => existing_state,
                                // Again, frozen state has been dealt with already, so we'll fall back to generated state
                                ::std::option::Option::None => {
                                    // Again, the render context can do the heavy lifting for us (this returns what we need, and can do type checking)
                                    // We also assume that any state we have is valid because it comes from the server
                                    // The user really should have a generation function, but if they don't then they'd get a panic, so give them a nice error message
                                    render_ctx.register_page_state_str::<#rx_props_ty>(&props.path, &props.state.unwrap_or_else(|| panic!("template `{}` takes a state, but no state generation functions were provided (please add at least one to use state)", #name_string))).unwrap()
                                }
                            }
                        }
                    )
                }
            }
        }
    } else if fn_args.is_empty() {
        // There are no arguments
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                use ::perseus::state::MakeRx;

                #[cfg(target_arch = "wasm32")]
                #hsr_thaw_frag

                #live_reload_frag

                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component(PerseusPage<#type_param>)]
                fn #name #generics() -> #return_type {
                    #block
                }
                ::sycamore::prelude::view! {
                    PerseusPage()
                }
            }
        }
    } else {
        // We filtered out this possibility in the function parsing
        unreachable!()
    }
}
