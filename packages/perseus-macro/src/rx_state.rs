use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    GenericParam, Ident, ItemStruct, Lifetime, LifetimeDef, Lit, Meta, NestedMeta, Result,
    Visibility,
};

pub fn make_rx_impl(mut orig_struct: ItemStruct, name_raw: Ident) -> TokenStream {
    // Note: we create three `struct`s with this macro: the original, the new one
    // (with references), and the new one (intermediary without references, stored
    // in context) So that we don't have to worry about unit structs or unnamed
    // fields, we'll just copy the struct and change the parts we want to
    // We won't create the final `struct` yet to avoid more operations than
    // necessary
    // Note that we leave this as whatever visibility the original state was to
    // avoid compiler errors (since it will be exposed as a trait-linked type
    // through the ref struct)
    let mut mid_struct = orig_struct.clone(); // This will use `RcSignal`s, and will be stored in context
    let ItemStruct {
        ident: orig_name,
        generics,
        ..
    } = orig_struct.clone();

    let ref_name = name_raw.clone();
    let mid_name = Ident::new(
        &(name_raw.to_string() + "PerseusRxIntermediary"),
        Span::call_site(),
    );
    mid_struct.ident = mid_name.clone();
    // Look through the attributes for any that warn about nested fields
    // These can't exist on the fields themselves because they'd be parsed before
    // this macro, and they're technically invalid syntax (grr.) When we come
    // across these fields, we'll run `.make_rx()` on them instead of naively
    // wrapping them in an `RcSignal`
    let nested_fields = mid_struct
        .attrs
        .iter()
        // We only care about our own attributes
        .filter(|attr| {
            attr.path.segments.len() == 2
                && attr.path.segments.first().unwrap().ident == "rx"
                && attr.path.segments.last().unwrap().ident == "nested"
        })
        // Remove any attributes that can't be parsed as a `MetaList`, returning the internal list
        // of what can (the 'arguments' to the attribute) We need them to be two elements
        // long (a field name and a wrapper type)
        .filter_map(|attr| match attr.parse_meta() {
            Ok(Meta::List(list)) if list.nested.len() == 2 => Some(list.nested),
            _ => None,
        })
        // Now parse the tokens within these to an `(Ident, Ident)`, the first being the name of the
        // field and the second being the wrapper type to use
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
    // Now remove our attributes from all the `struct`s
    let mut filtered_attrs = Vec::new();
    for attr in orig_struct.attrs.iter() {
        if !(attr.path.segments.len() == 2
            && attr.path.segments.first().unwrap().ident == "rx"
            && attr.path.segments.last().unwrap().ident == "nested")
        {
            filtered_attrs.push(attr.clone());
        }
    }
    orig_struct.attrs = filtered_attrs.clone();
    mid_struct.attrs = filtered_attrs;

    // Now define the final `struct` that uses references
    let mut ref_struct = mid_struct.clone();
    ref_struct.ident = ref_name.clone();
    // Add the `'rx` lifetime to the generics
    // We also need a separate variable for the generics, but using an anonymous
    // lifetime for a function's return value
    ref_struct.generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeDef::new(Lifetime::new("'rx", Span::call_site()))),
    );

    match mid_struct.fields {
        syn::Fields::Named(ref mut fields) => {
            for field in fields.named.iter_mut() {
                let orig_ty = &field.ty;
                // Check if this field was registered as one to use nested reactivity
                let wrapper_ty = nested_fields_map.get(field.ident.as_ref().unwrap());
                field.ty = if let Some(wrapper_ty) = wrapper_ty {
                    let mid_wrapper_ty = Ident::new(
                        &(wrapper_ty.to_string() + "PerseusRxIntermediary"),
                        Span::call_site(),
                    );
                    syn::Type::Verbatim(quote!(#mid_wrapper_ty))
                } else {
                    syn::Type::Verbatim(quote!(::sycamore::prelude::RcSignal<#orig_ty>))
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
            mid_struct,
            "tuple structs can't be made reactive with this macro (try using named fields instead)",
        )
        .to_compile_error(),
        // We may well need a unit struct for templates that use global state but don't have proper
        // state of their own We don't need to modify any fields
        syn::Fields::Unit => (),
    };
    match ref_struct.fields {
        syn::Fields::Named(ref mut fields) => {
            for field in fields.named.iter_mut() {
                let orig_ty = &field.ty;
                // Check if this field was registered as one to use nested reactivity
                let wrapper_ty = nested_fields_map.get(field.ident.as_ref().unwrap());
                field.ty = if let Some(wrapper_ty) = wrapper_ty {
                    // If we don't make this a reference, nested properties have to be cloned (not
                    // nice for ergonomics) TODO Check back on this, could bite
                    // back!
                    syn::Type::Verbatim(quote!(&'rx #wrapper_ty<'rx>))
                } else {
                    // This is the only difference from the intermediate `struct` (this lifetime is
                    // declared above)
                    syn::Type::Verbatim(quote!(&'rx ::sycamore::prelude::RcSignal<#orig_ty>))
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
            mid_struct,
            "tuple structs can't be made reactive with this macro (try using named fields instead)",
        )
        .to_compile_error(),
        // We may well need a unit struct for templates that use global state but don't have proper
        // state of their own We don't need to modify any fields
        syn::Fields::Unit => (),
    };

    // Create a list of fields for the `.make_rx()` method
    let make_rx_fields = match mid_struct.fields {
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
                        #field_name: ::sycamore::prelude::create_rc_signal(self.#field_name),
                    });
                }
            }
            quote! {
                #mid_name {
                    #field_assignments
                }
            }
        }
        syn::Fields::Unit => quote!(#mid_name),
        // We filtered out the other types before
        _ => unreachable!(),
    };
    // Create a list of fields for turning the intermediary `struct` into one using
    // scoped references
    let make_ref_fields = match mid_struct.fields {
        syn::Fields::Named(ref mut fields) => {
            let mut field_assignments = quote!();
            for field in fields.named.iter_mut() {
                // We know it has an identifier because it's a named field
                let field_name = field.ident.as_ref().unwrap();
                // Check if this field was registered as one to use nested reactivity
                if nested_fields_map.contains_key(field.ident.as_ref().unwrap()) {
                    field_assignments.extend(quote! {
                        #field_name: ::sycamore::prelude::create_ref(cx, self.#field_name.to_ref_struct(cx)),
                    })
                } else {
                    // This will be used in a place in which the `cx` variable stores a reactive
                    // scope
                    field_assignments.extend(quote! {
                        #field_name: ::sycamore::prelude::create_ref(cx, self.#field_name),
                    });
                }
            }
            quote! {
                #ref_name {
                    #field_assignments
                }
            }
        }
        syn::Fields::Unit => quote!(#ref_name),
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
                    // We can `.clone()` the field because we implement `Clone` on both the new and
                    // the original `struct`s, meaning all fields must also be `Clone`
                    field_assignments.extend(quote! {
                        #field_name: (*self.#field_name.get_untracked()).clone(),
                    });
                }
            }
            quote! {
                #orig_name {
                    #field_assignments
                }
            }
        }
        syn::Fields::Unit => quote!(#orig_name),
        // We filtered out the other types before
        _ => unreachable!(),
    };

    quote! {
        // We add a Serde derivation because it will always be necessary for Perseus on the original `struct`, and it's really difficult and brittle to filter it out
        #[derive(::serde::Serialize, ::serde::Deserialize, ::std::clone::Clone)]
        #orig_struct
        impl #generics ::perseus::state::MakeRx for #orig_name #generics {
            type Rx = #mid_name #generics;
            fn make_rx(self) -> #mid_name #generics {
                use ::perseus::state::MakeRx;
                #make_rx_fields
            }
        }
        #[derive(::std::clone::Clone)]
        #mid_struct
        impl #generics ::perseus::state::MakeUnrx for #mid_name #generics {
            type Unrx = #orig_name #generics;
            fn make_unrx(self) -> #orig_name #generics {
                use ::perseus::state::MakeUnrx;
                #make_unrx_fields
            }
        }
        impl #generics ::perseus::state::Freeze for #mid_name #generics {
            fn freeze(&self) -> ::std::string::String {
                use ::perseus::state::MakeUnrx;
                let unrx = #make_unrx_fields;
                // TODO Is this `.unwrap()` safe?
                ::serde_json::to_string(&unrx).unwrap()
            }
        }
        // TODO Generics
        impl ::perseus::state::MakeRxRef for #mid_name {
            type RxRef<'a> = #ref_name<'a>;
            fn to_ref_struct<'a>(self, cx: ::sycamore::prelude::Scope<'a>) -> #ref_name<'a> {
                #make_ref_fields
            }
        }
        #[derive(::std::clone::Clone)]
        #ref_struct
        impl<'a> ::perseus::state::RxRef for #ref_name<'a> {
            type RxNonRef = #mid_name;
        }
    }
}
