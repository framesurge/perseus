use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, FnArg, Generics, Ident, Item, ItemFn, PatType, Result, ReturnType, Type,
    Visibility,
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

#[derive(FromMeta)]
pub struct TemplateArgs {
    /// The name of the component.
    component: Ident,
    /// The name of the type parameter to use (default to `G`).
    #[darling(default)]
    type_param: Option<Ident>,
    /// The identifier of the global state type, if this template needs it.
    #[darling(default)]
    global_state: Option<Ident>,
    /// The name of the unreactive properties, if there are any.
    #[darling(default)]
    unrx_props: Option<Ident>,
}

pub fn template_impl(input: TemplateFn, args: TemplateArgs) -> TokenStream {
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

    let component_name = &args.component;
    let type_param = match &args.type_param {
        Some(type_param) => type_param.clone(),
        None => Ident::new("G", Span::call_site()),
    };
    // This is only optional if the second argument wasn't provided
    let global_state = if fn_args.len() == 2 {
        match &args.global_state {
            Some(global_state) => global_state.clone(),
            None => return syn::Error::new_spanned(&fn_args[0], "template functions with two arguments must declare their global state type (`global_state = `)").to_compile_error()
        }
    } else {
        match &args.global_state {
            Some(global_state) => global_state.clone(),
            None => Ident::new("Dummy", Span::call_site()),
        }
    };
    // This is only optional if the first argument wasn't provided
    let unrx_props = if !fn_args.is_empty() {
        match &args.unrx_props {
            Some(unrx_props) => unrx_props.clone(),
            None => return syn::Error::new_spanned(&fn_args[0], "template functions with one argument or more must declare their unreactive properties type (`unrx_props = `)").to_compile_error()
        }
    } else {
        match &args.unrx_props {
            Some(unrx_props) => unrx_props.clone(),
            None => Ident::new("Dummy", Span::call_site()),
        }
    };

    // We create a wrapper function that can be easily provided to `.template()` that does deserialization automatically if needed
    // This is dependent on what arguments the template takes
    if fn_args.len() == 2 {
        // There's an argument for page properties that needs to have state extracted, so the wrapper will deserialize it
        // We'll also make it reactive and add it to the page state store
        let state_arg = &fn_args[0];
        // There's also a second argument for the global state, which we'll deserialize and make global if it's not already (aka. if any other pages have loaded before this one)
        // Sycamore won't let us have more than one argument to a component though, so we sneakily extract it and literally construct it as a variable (this should be fine?)
        let global_state_arg = &fn_args[1];
        let (global_state_arg_pat, global_state_rx) = match global_state_arg {
            FnArg::Typed(PatType { pat, ty, .. }) => (pat, ty),
            FnArg::Receiver(_) => unreachable!(),
        };
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                // Deserialize the global state, make it reactive, and register it with the `RenderCtx`
                // If it's already there, we'll leave it
                // This means that we can pass an `Option<String>` around safely and then deal with it at the template site
                let global_state_refcell = ::perseus::get_render_ctx!().global_state;
                let global_state = global_state_refcell.borrow();
                if (&global_state).downcast_ref::<::std::option::Option::<()>>().is_some() {
                    // We can downcast it as the type set by the core render system, so we're the first page to be loaded
                    // In that case, we'll set the global state properly
                    drop(global_state);
                    let mut global_state = global_state_refcell.borrow_mut();
                    // This will be defined if we're the first page
                    let global_state_props = &props.global_state.unwrap();
                    let new_global_state = ::serde_json::from_str::<#global_state>(global_state_props).unwrap().make_rx();
                    *global_state = ::std::boxed::Box::new(new_global_state);
                    // The component function can now access this in `RenderCtx`
                }
                // The user's function
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component(#component_name<#type_param>)]
                fn #name#generics(#state_arg) -> #return_type {
                    let #global_state_arg_pat: #global_state_rx = {
                        let global_state = ::perseus::get_render_ctx!().global_state;
                        let global_state = global_state.borrow();
                        // We can guarantee that it will downcast correctly now, because we'll only invoke the component from this function, which sets up the global state correctly
                        let global_state_ref = (&global_state).downcast_ref::<#global_state_rx>().unwrap();
                        (*global_state_ref).clone()
                    };
                    #block
                }
                ::sycamore::prelude::view! {
                    #component_name(
                        {
                            // Check if properties of the reactive type are already in the page state store
                            // If they are, we'll use them (so state persists for templates across the whole app)
                            let mut pss = ::perseus::get_render_ctx!().page_state_store;
                            match pss.get(&props.path) {
                                ::std::option::Option::Some(old_state) => old_state,
                                ::std::option::Option::None => {
                                    // If there are props, they will always be provided, the compiler just doesn't know that
                                    // If the user is using this macro, they sure should be using `#[make_rx(...)]` or similar!
                                    let rx_props = ::serde_json::from_str::<#unrx_props>(&props.state.unwrap()).unwrap().make_rx();
                                    // They aren't in there, so insert them
                                    pss.add(&props.path, rx_props.clone());
                                    rx_props
                                }
                            }
                        }
                    )
                }
            }
        }
    } else if fn_args.len() == 1 {
        // There's an argument for page properties that needs to have state extracted, so the wrapper will deserialize it
        // We'll also make it reactive and add it to the page state store
        let arg = &fn_args[0];
        quote! {
            #vis fn #name<G: ::sycamore::prelude::Html>(props: ::perseus::templates::PageProps) -> ::sycamore::prelude::View<G> {
                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component(#component_name<#type_param>)]
                fn #name#generics(#arg) -> #return_type {
                    #block
                }
                ::sycamore::prelude::view! {
                    #component_name(
                        {
                            // Check if properties of the reactive type are already in the page state store
                            // If they are, we'll use them (so state persists for templates across the whole app)
                            let mut pss = ::perseus::get_render_ctx!().page_state_store;
                            match pss.get(&props.path) {
                                ::std::option::Option::Some(old_state) => old_state,
                                ::std::option::Option::None => {
                                    // If there are props, they will always be provided, the compiler just doesn't know that
                                    // If the user is using this macro, they sure should be using `#[make_rx(...)]` or similar!
                                    let rx_props = ::serde_json::from_str::<#unrx_props>(&props.state.unwrap()).unwrap().make_rx();
                                    // They aren't in there, so insert them
                                    pss.add(&props.path, rx_props.clone());
                                    rx_props
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
                // The user's function, with Sycamore component annotations and the like preserved
                // We know this won't be async because Sycamore doesn't allow that
                #(#attrs)*
                #[::sycamore::component(#component_name<#type_param>)]
                fn #name#generics() -> #return_type {
                    #block
                }
                ::sycamore::prelude::view! {
                    #component_name()
                }
            }
        }
    } else {
        // We filtered out this possibility in the function parsing
        unreachable!()
    }
}
