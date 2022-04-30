use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemStruct, Lifetime, LifetimeDef, Lit, Meta, NestedMeta, Result, GenericParam};

pub fn make_rx_impl(mut orig_struct: ItemStruct, name: Ident) -> TokenStream {
    // So that we don't have to worry about unit structs or unnamed fields, we'll just copy the struct and change the parts we want to
    let mut new_struct = orig_struct.clone();
    let ItemStruct {
        ident, generics, ..
    } = orig_struct.clone();

    new_struct.ident = name.clone();
    // Reset the attributes entirely (we don't want any Serde derivations in there)
    // Look through the attributes for any that warn about nested fields
    // These can't exist on the fields themselves because they'd be parsed before this macro, and tehy're technically invalid syntax (grr.)
    // When we come across these fields, we'll run `.make_rx()` on them instead of naively wrapping them in an `RcSignal`
    let nested_fields = new_struct
        .attrs
        .iter()
        // We only care about our own attributes
        .filter(|attr| {
            attr.path.segments.len() == 2
                && attr.path.segments.first().unwrap().ident == "rx"
                && attr.path.segments.last().unwrap().ident == "nested"
        })
        // Remove any attributes that can't be parsed as a `MetaList`, returning the internal list of what can (the 'arguments' to the attribute)
        // We need them to be two elements long (a field name and a wrapper type)
        .filter_map(|attr| match attr.parse_meta() {
            Ok(Meta::List(list)) if list.nested.len() == 2 => Some(list.nested),
            _ => None,
        })
        // Now parse the tokens within these to an `(Ident, Ident)`, the first being the name of the field and the second being the wrapper type to use
        .map(|meta_list| {
            // Extract field name and wrapper type (we know this only has two elements)
            let field_name = match meta_list.first().unwrap() {
                NestedMeta::Lit(Lit::Str(s)) => Ident::new(s.value().as_str(), Span::call_site()),
                NestedMeta::Lit(val) => {
                    return Err(syn::Error::new_spanned(
                        val,
                        "first argument must be string literal field name",
                    ))
                }
                NestedMeta::Meta(meta) => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "first argument must be string literal field name",
                    ))
                }
            };
            let wrapper_ty = match meta_list.last().unwrap() {
                // TODO Is this `.unwrap()` actually safe to use?
                NestedMeta::Meta(meta) => &meta.path().segments.first().unwrap().ident,
                NestedMeta::Lit(val) => {
                    return Err(syn::Error::new_spanned(
                        val,
                        "second argument must be reactive wrapper type",
                    ))
                }
            };

            Ok::<(Ident, Ident), syn::Error>((field_name, wrapper_ty.clone()))
        })
        .collect::<Vec<Result<(Ident, Ident)>>>();
    // Handle any errors produced by that final transformation and create a map
    let mut nested_fields_map = HashMap::new();
    for res in nested_fields {
        match res {
            Ok((k, v)) => nested_fields_map.insert(k, v),
            Err(err) => return err.to_compile_error(),
        };
    }
    // Now remove our attributes from both the original and the new `struct`s
    let mut filtered_attrs_orig = Vec::new();
    let mut filtered_attrs_new = Vec::new();
    for attr in orig_struct.attrs.iter() {
        if !(attr.path.segments.len() == 2
            && attr.path.segments.first().unwrap().ident == "rx"
            && attr.path.segments.last().unwrap().ident == "nested")
        {
            filtered_attrs_orig.push(attr.clone());
            filtered_attrs_new.push(attr.clone());
        }
    }
    orig_struct.attrs = filtered_attrs_orig;
    new_struct.attrs = filtered_attrs_new;
    // Now add the `'rx` lifetime to the new `struct`'s generics (we'll need it for all the `Signal`s)
    new_struct.generics.params.insert(0, GenericParam::Lifetime(LifetimeDef::new(Lifetime::new("'rx", Span::call_site()))));
    let new_generics = &new_struct.generics;

    match new_struct.fields {
        syn::Fields::Named(ref mut fields) => {
            for field in fields.named.iter_mut() {
                let orig_ty = &field.ty;
                // Check if this field was registered as one to use nested reactivity
                let wrapper_ty = nested_fields_map.get(field.ident.as_ref().unwrap());
                field.ty = if let Some(wrapper_ty) = wrapper_ty {
                    syn::Type::Verbatim(quote!(#wrapper_ty))
                } else {
                    syn::Type::Verbatim(quote!(&'rx ::sycamore::prelude::Signal<#orig_ty>))
                };
                // Remove any `serde` attributes (Serde can't be used with the reactive version)
                let mut new_attrs = Vec::new();
                for attr in field.attrs.iter() {
                    if !(attr.path.segments.len() == 1
                        && attr.path.segments.first().unwrap().ident == "serde")
                    {
                        new_attrs.push(attr.clone());
                    }
                }
                field.attrs = new_attrs;
            }
        }
        syn::Fields::Unnamed(_) => return syn::Error::new_spanned(
            new_struct,
            "tuple structs can't be made reactive with this macro (try using named fields instead)",
        )
        .to_compile_error(),
        // We may well need a unit struct for templates that use global state but don't have proper state of their own
        // We don't need to modify any fields
        syn::Fields::Unit => (),
    };

    // Create a list of fields for the `.make_rx()` method
    let make_rx_fields = match new_struct.fields {
        syn::Fields::Named(ref mut fields) => {
            let mut field_assignments = quote!();
            for field in fields.named.iter_mut() {
                // We know it has an identifier because it's a named field
                let field_name = field.ident.as_ref().unwrap();
                // Check if this field was registered as one to use nested reactivity
                if nested_fields_map.contains_key(field.ident.as_ref().unwrap()) {
                    field_assignments.extend(quote! {
                        #field_name: self.#field_name.make_rx(),
                    })
                } else {
                    field_assignments.extend(quote! {
                        // The `cx` parameter will be available where we interpolate this
                        #field_name: ::sycamore::prelude::create_signal(self.#field_name, cx),
                    });
                }
            }
            quote! {
                #name {
                    #field_assignments
                }
            }
        }
        syn::Fields::Unit => quote!(#name),
        // We filtered out the other types before
        _ => unreachable!(),
    };
    let make_unrx_fields = match orig_struct.fields {
        syn::Fields::Named(ref mut fields) => {
            let mut field_assignments = quote!();
            for field in fields.named.iter_mut() {
                // We know it has an identifier because it's a named field
                let field_name = field.ident.as_ref().unwrap();
                // Check if this field was registered as one to use nested reactivity
                if nested_fields_map.contains_key(field.ident.as_ref().unwrap()) {
                    field_assignments.extend(quote! {
                        #field_name: self.#field_name.clone().make_unrx(),
                    })
                } else {
                    // We can `.clone()` the field because we implement `Clone` on both the new and the original `struct`s, meaning all fields must also be `Clone`
                    field_assignments.extend(quote! {
                        #field_name: (*self.#field_name.get_untracked()).clone(),
                    });
                }
            }
            quote! {
                #ident {
                    #field_assignments
                }
            }
        }
        syn::Fields::Unit => quote!(#ident),
        // We filtered out the other types before
        _ => unreachable!(),
    };

    quote! {
        // We add a Serde derivation because it will always be necessary for Perseus on the original `struct`, and it's really difficult and brittle to filter it out
        #[derive(::serde::Serialize, ::serde::Deserialize, ::std::clone::Clone)]
        #orig_struct
        // BUG The lifetime parameter `'rx` is unconstrained here
        impl #generics ::perseus::state::MakeRx for #ident #generics {
            type Rx = #name #new_generics;
            fn make_rx<'rx>(self, cx: ::sycamore::prelude::Scope<'rx>) -> #name #new_generics {
                use ::perseus::state::MakeRx;
                #make_rx_fields
            }
        }
        #[derive(::std::clone::Clone)]
        #new_struct
        impl #new_generics ::perseus::state::MakeUnrx for #name #new_generics {
            type Unrx = #ident #generics;
            fn make_unrx(self) -> #ident #generics {
                use ::perseus::state::MakeUnrx;
                #make_unrx_fields
            }
        }
        impl #new_generics ::perseus::state::Freeze for #name #new_generics {
            fn freeze(&self) -> ::std::string::String {
                use ::perseus::state::MakeUnrx;
                let unrx = #make_unrx_fields;
                // TODO Is this `.unwrap()` safe?
                ::serde_json::to_string(&unrx).unwrap()
            }
        }
    }
}
