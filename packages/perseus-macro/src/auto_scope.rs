use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, FnArg, Ident, Item, ItemFn, PatType, Result, ReturnType, Type, TypeReference,
    Visibility,
};

/// A function that can be wrapped in the Perseus test sub-harness.
pub struct TemplateFn {
    /// The body of the function.
    pub block: Box<Block>,
    /// The arguments for custom properties and a global state, both of which
    /// are optional. (But global state needs custom properties, which can be a
    /// dummy `struct`.)
    pub args: Vec<FnArg>,
    /// The visibility of the function.
    pub vis: Visibility,
    /// Any attributes the function uses.
    pub attrs: Vec<Attribute>,
    /// The actual name of the function.
    pub name: Ident,
    /// The return type of the function.
    pub return_type: Box<Type>,
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
                // We can have 2 arguments only (scope, state), or 3 if it's
                // a capsule
                // Any other kind of template doesn't need this macro
                if args.len() != 2 && args.len() != 3 {
                    return Err(syn::Error::new_spanned(&sig.inputs, "`#[auto_scope]` is only useful if you're using reactive state (which requires two arguments)"));
                }

                Ok(Self {
                    block,
                    args,
                    vis,
                    attrs,
                    name: sig.ident,
                    return_type,
                })
            }
            item => Err(syn::Error::new_spanned(
                item,
                "only functions can be used as templates",
            )),
        }
    }
}

pub fn template_impl(input: TemplateFn) -> TokenStream {
    let TemplateFn {
        block,
        // We know that these are all typed (none are `self`)
        args: fn_args,
        vis,
        attrs,
        name,
        return_type,
    } = input;

    let arg = &fn_args[1];
    let (state_pat, state_arg) = match arg {
        FnArg::Typed(PatType { ty, pat, .. }) => match &**ty {
            Type::Reference(TypeReference { elem, .. }) => (pat, elem),
            _ => return syn::Error::new_spanned(arg, "the state argument must be a reference (e.g. `&MyStateTypeRx`); if you're using unreactive state (i.e. you're deriving `UnreactiveState` instead of `ReactiveState`), you don't need this macro!").to_compile_error()
        },
        FnArg::Receiver(_) => unreachable!(),
    };
    let props_arg = match fn_args.get(2) {
        Some(arg) => quote!( #arg ),
        None => quote!(),
    };
    quote! {
        // All we do is set up the lifetimes correctly
        #(#attrs)*
        #vis fn #name<'__page, G: ::sycamore::prelude::Html>(
            cx: ::sycamore::prelude::BoundedScope<'_, '__page>,
            #state_pat: &'__page #state_arg,
            // Capsules have another argument for properties
            #props_arg
        ) -> #return_type {
            #block
        }
    }
}
