use darling::{ast::Data, FromDeriveInput, FromField, ToTokens};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, Ident, Type, Visibility};

/// This is used to parse what the user gives us with `darling`.
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rx))]
pub struct ReactiveStateDeriveInput {
    /// If specified, a type alias will be created for the final reactive
    /// `struct` for ease of reference.
    #[darling(default)]
    alias: Option<Ident>,

    ident: Ident,
    vis: Visibility,
    // The first of these is only relevant if we're parsing `enum`s, which we aren't
    pub data: Data<darling::util::Ignored, ReactiveStateField>,
    attrs: Vec<Attribute>,
}

/// This is used to parse each individual field in what the user gives us.
#[derive(Debug, FromField, Clone)]
#[darling(attributes(rx))]
pub struct ReactiveStateField {
    /// Whether or not we should expect the annotated field to be able to made
    /// reactive itself, enabling nested reactivity.
    #[darling(default)]
    nested: bool,

    ident: Option<Ident>,
    vis: Visibility,
    ty: Type,
    attrs: Vec<syn::Attribute>,
}

/// The underlying implementation of the `ReactiveState` derive macro, which
/// implements the traits involved in Perseus' reactive state platform, creating
/// an intermediary reactive struct using `RcSignal`s and a final one using
/// `&'cx Signal`s, where `cx` is a Sycamore scope lifetime.
pub fn make_rx_impl(input: ReactiveStateDeriveInput) -> TokenStream {
    // Extract the fields of the `struct`
    let fields = match input.data {
        Data::Struct(fields) => fields.fields,
        Data::Enum(_) => return syn::Error::new_spanned(
            input.ident,
            "you can only make `struct`s reactive with this macro (`enum` capability will be added in a future release, for now you'll have to implement it manually)"
        ).to_compile_error(),
    };
    // Now go through them and create what we want for both the intermediate and the
    // reactive `struct`s
    let mut intermediate_fields = quote!();
    let mut ref_fields = quote!();
    let mut intermediate_field_makers = quote!();
    let mut ref_field_makers = quote!(); // These start at the intermediate
    let mut unrx_field_makers = quote!();
    for field in fields.iter() {
        let old_ty = field.ty.to_token_stream();
        let field_ident = field.ident.as_ref().unwrap(); // It's a `struct`, so this is defined
        let field_vis = &field.vis;
        let mut field_attrs = quote!();
        for attr in field.attrs.iter() {
            field_attrs.extend(attr.to_token_stream());
        }

        // Nested fields are left as-is, non-nested ones are wrapped in `RcSignal`s
        if field.nested {
            // Nested types should implement the necessary linking traits
            intermediate_fields.extend(quote! {
                #field_attrs
                #field_vis #field_ident: <#old_ty as ::perseus::state::MakeRx>::Rx,
            });
            ref_fields.extend(quote! {
                #field_attrs
                #field_vis #field_ident: <<#old_ty as ::perseus::state::MakeRx>::Rx as ::perseus::state::MakeRxRef>::RxRef<'__derived_rx>,
            });
            intermediate_field_makers.extend(quote! { #field_ident: self.#field_ident.make_rx(), });
            ref_field_makers.extend(quote! { #field_ident: self.#field_ident.to_ref_struct(cx), });
            unrx_field_makers
                .extend(quote! { #field_ident: self.#field_ident.clone().make_unrx(), });
        } else {
            intermediate_fields.extend(quote! {
                #field_attrs
                #field_vis #field_ident: ::sycamore::prelude::RcSignal<#old_ty>,
            });
            ref_fields.extend(quote! {
                #field_attrs
                #field_vis #field_ident: &'__derived_rx ::sycamore::prelude::RcSignal<#old_ty>,
            });
            intermediate_field_makers.extend(
                quote! { #field_ident: ::sycamore::prelude::create_rc_signal(self.#field_ident), },
            );
            ref_field_makers.extend(
                quote! { #field_ident: ::sycamore::prelude::create_ref(cx, self.#field_ident), },
            );
            unrx_field_makers
                .extend(quote! { #field_ident: (*self.#field_ident.get_untracked()).clone(), });
            // All fields must be `Clone`
        }
    }

    let ReactiveStateDeriveInput {
        ident,
        vis,
        attrs: attrs_vec,
        alias,
        ..
    } = input;
    let mut attrs = quote!();
    for attr in attrs_vec.iter() {
        attrs.extend(attr.to_token_stream());
    }
    let intermediate_ident = Ident::new(
        &(ident.to_string() + "PerseusRxIntermediate"),
        Span::call_site(),
    );
    let ref_ident = Ident::new(&(ident.to_string() + "PerseusRxRef"), Span::call_site());

    // Create a type alias for the final reactive version for convenience, if the
    // user asked for one
    let ref_alias = if let Some(alias) = alias {
        // We use the full form for a cleaner expansion in IDEs
        quote! { #vis type #alias<'__derived_rx> = <<#ident as ::perseus::state::MakeRx>::Rx as ::perseus::state::MakeRxRef>::RxRef<'__derived_rx>; }
    } else {
        quote!()
    };

    // TODO Generics support
    quote! {
        #attrs
        #[derive(Clone)]
        #vis struct #intermediate_ident {
            #intermediate_fields
        }

        #attrs
        #vis struct #ref_ident<'__derived_rx> {
            #ref_fields
        }

        impl ::perseus::state::MakeRx for #ident {
            type Rx = #intermediate_ident;
            fn make_rx(self) -> Self::Rx {
                use ::perseus::state::MakeRx;
                Self::Rx {
                    #intermediate_field_makers
                }
            }
        }
        impl ::perseus::state::MakeUnrx for #intermediate_ident {
            type Unrx = #ident;
            fn make_unrx(self) -> Self::Unrx {
                use ::perseus::state::MakeUnrx;
                Self::Unrx {
                    #unrx_field_makers
                }
            }
        }
        impl ::perseus::state::Freeze for #intermediate_ident {
            fn freeze(&self) -> ::std::string::String {
                use ::perseus::state::MakeUnrx;
                let unrx = self.clone().make_unrx();
                // TODO Is this `.unwrap()` safe?
                ::serde_json::to_string(&unrx).unwrap()
            }
        }
        impl ::perseus::state::MakeRxRef for #intermediate_ident {
            type RxRef<'__derived_rx> = #ref_ident<'__derived_rx>;
            fn to_ref_struct<'__derived_rx>(self, cx: ::sycamore::prelude::Scope<'__derived_rx>) -> Self::RxRef<'__derived_rx> {
                Self::RxRef {
                    #ref_field_makers
                }
            }
        }
        impl<'__derived_rx> ::perseus::state::RxRef for #ref_ident<'__derived_rx> {
            type RxNonRef = #intermediate_ident;
        }

        #ref_alias
    }

    // // Note: we create three `struct`s with this macro: the original, the new
    // one // (with references), and the new one (intermediary without
    // references, stored // in context) So that we don't have to worry
    // about unit structs or unnamed // fields, we'll just copy the struct
    // and change the parts we want to // We won't create the final `struct`
    // yet to avoid more operations than // necessary
    // // Note that we leave this as whatever visibility the original state was
    // to // avoid compiler errors (since it will be exposed as a
    // trait-linked type // through the ref struct)
    // let mut mid_struct = orig_struct.clone(); // This will use `RcSignal`s,
    // and will be stored in context let ItemStruct {
    //     ident: orig_name,
    //     generics,
    //     ..
    // } = orig_struct.clone();
    // // The name of the final reference `struct`'s type alias
    // let ref_name = helpers.name.unwrap_or_else(||
    // Ident::new(&(orig_name.to_string() + "Rx"), Span::call_site())); // The
    // intermediate struct shouldn't be easily accessible let mid_name =
    // Ident::new(     &(orig_name.to_string() + "PerseusRxIntermediary"),
    //     Span::call_site(),
    // );
    // mid_struct.ident = mid_name.clone();
    // // Look through the attributes for any that warn about nested fields
    // // These can't exist on the fields themselves because they'd be parsed
    // before // this macro, and they're technically invalid syntax (grr.)
    // When we come // across these fields, we'll run `.make_rx()` on them
    // instead of naively // wrapping them in an `RcSignal`
    // let nested_fields = mid_struct
    //     .attrs
    //     .iter()
    //     // We only care about our own attributes
    //     .filter(|attr| {
    //         attr.path.segments.len() == 2
    //             && attr.path.segments.first().unwrap().ident == "rx"
    //             && attr.path.segments.last().unwrap().ident == "nested"
    //     })
    //     // Remove any attributes that can't be parsed as a `MetaList`,
    // returning the internal list     // of what can (the 'arguments' to
    // the attribute) We need them to be two elements     // long (a field
    // name and a wrapper type)     .filter_map(|attr| match
    // attr.parse_meta() {         Ok(Meta::List(list)) if list.nested.len()
    // == 2 => Some(list.nested),         _ => None,
    //     })
    //     // Now parse the tokens within these to an `(Ident, Ident)`, the
    // first being the name of the     // field and the second being the
    // wrapper type to use     .map(|meta_list| {
    //         // Extract field name and wrapper type (we know this only has two
    // elements)         let field_name = match meta_list.first().unwrap() {
    //             NestedMeta::Lit(Lit::Str(s)) =>
    // Ident::new(s.value().as_str(), Span::call_site()),             
    // NestedMeta::Lit(val) => {                 return
    // Err(syn::Error::new_spanned(                     val,
    //                     "first argument must be string literal field name",
    //                 ))
    //             }
    //             NestedMeta::Meta(meta) => {
    //                 return Err(syn::Error::new_spanned(
    //                     meta,
    //                     "first argument must be string literal field name",
    //                 ))
    //             }
    //         };
    //         let wrapper_ty = match meta_list.last().unwrap() {
    //             // TODO Is this `.unwrap()` actually safe to use?
    //             NestedMeta::Meta(meta) =>
    // &meta.path().segments.first().unwrap().ident,             
    // NestedMeta::Lit(val) => {                 return
    // Err(syn::Error::new_spanned(                     val,
    //                     "second argument must be reactive wrapper type",
    //                 ))
    //             }
    //         };

    //         Ok::<(Ident, Ident), syn::Error>((field_name,
    // wrapper_ty.clone()))     })
    //     .collect::<Vec<Result<(Ident, Ident)>>>();
    // // Handle any errors produced by that final transformation and create a
    // map let mut nested_fields_map = HashMap::new();
    // for res in nested_fields {
    //     match res {
    //         Ok((k, v)) => nested_fields_map.insert(k, v),
    //         Err(err) => return err.to_compile_error(),
    //     };
    // }
    // // Now remove our attributes from all the `struct`s
    // let mut filtered_attrs = Vec::new();
    // for attr in orig_struct.attrs.iter() {
    //     if !(attr.path.segments.len() == 2
    //         && attr.path.segments.first().unwrap().ident == "rx"
    //         && attr.path.segments.last().unwrap().ident == "nested")
    //     {
    //         filtered_attrs.push(attr.clone());
    //     }
    // }
    // orig_struct.attrs = filtered_attrs.clone();
    // mid_struct.attrs = filtered_attrs;

    // // Now define the final `struct` that uses references
    // let mut ref_struct = mid_struct.clone();
    // ref_struct.ident = ref_name.clone();
    // // Add the `'rx` lifetime to the generics
    // // We also need a separate variable for the generics, but using an
    // anonymous // lifetime for a function's return value
    // ref_struct.generics.params.insert(
    //     0,
    //     GenericParam::Lifetime(LifetimeDef::new(Lifetime::new("'rx",
    // Span::call_site()))), );

    // match mid_struct.fields {
    //     syn::Fields::Named(ref mut fields) => {
    //         for field in fields.named.iter_mut() {
    //             let orig_ty = &field.ty;
    //             // Check if this field was registered as one to use nested
    // reactivity             let wrapper_ty =
    // nested_fields_map.get(field.ident.as_ref().unwrap());             
    // field.ty = if let Some(wrapper_ty) = wrapper_ty {                 let
    // mid_wrapper_ty = Ident::new(                     
    // &(wrapper_ty.to_string() + "PerseusRxIntermediary"),                 
    // Span::call_site(),                 );
    //                 syn::Type::Verbatim(quote!(#mid_wrapper_ty))
    //             } else {
    //                 
    // syn::Type::Verbatim(quote!(::sycamore::prelude::RcSignal<#orig_ty>))
    //             };
    //             // Remove any `serde` attributes (Serde can't be used with
    // the reactive version)             let mut new_attrs = Vec::new();
    //             for attr in field.attrs.iter() {
    //                 if !(attr.path.segments.len() == 1
    //                     && attr.path.segments.first().unwrap().ident ==
    // "serde")                 {
    //                     new_attrs.push(attr.clone());
    //                 }
    //             }
    //             field.attrs = new_attrs;
    //         }
    //     }
    //     syn::Fields::Unnamed(_) => return syn::Error::new_spanned(
    //         mid_struct,
    //         "tuple structs can't be made reactive with this macro (try using
    // named fields instead)",     )
    //     .to_compile_error(),
    //     // We may well need a unit struct for templates that use global state
    // but don't have proper     // state of their own We don't need to
    // modify any fields     syn::Fields::Unit => (),
    // };
    // match ref_struct.fields {
    //     syn::Fields::Named(ref mut fields) => {
    //         for field in fields.named.iter_mut() {
    //             let orig_ty = &field.ty;
    //             // Check if this field was registered as one to use nested
    // reactivity             let wrapper_ty =
    // nested_fields_map.get(field.ident.as_ref().unwrap());             
    // field.ty = if let Some(wrapper_ty) = wrapper_ty {                 //
    // If we don't make this a reference, nested properties have to be cloned
    // (not                 // nice for ergonomics) TODO Check back on this,
    // could bite                 // back!
    //                 syn::Type::Verbatim(quote!(&'rx #wrapper_ty<'rx>))
    //             } else {
    //                 // This is the only difference from the intermediate
    // `struct` (this lifetime is                 // declared above)
    //                 syn::Type::Verbatim(quote!(&'rx
    // ::sycamore::prelude::RcSignal<#orig_ty>))             };
    //             // Remove any `serde` attributes (Serde can't be used with
    // the reactive version)             let mut new_attrs = Vec::new();
    //             for attr in field.attrs.iter() {
    //                 if !(attr.path.segments.len() == 1
    //                     && attr.path.segments.first().unwrap().ident ==
    // "serde")                 {
    //                     new_attrs.push(attr.clone());
    //                 }
    //             }
    //             field.attrs = new_attrs;
    //         }
    //     }
    //     syn::Fields::Unnamed(_) => return syn::Error::new_spanned(
    //         mid_struct,
    //         "tuple structs can't be made reactive with this macro (try using
    // named fields instead)",     )
    //     .to_compile_error(),
    //     // We may well need a unit struct for templates that use global state
    // but don't have proper     // state of their own We don't need to
    // modify any fields     syn::Fields::Unit => (),
    // };

    // // Create a list of fields for the `.make_rx()` method
    // let make_rx_fields = match mid_struct.fields {
    //     syn::Fields::Named(ref mut fields) => {
    //         let mut field_assignments = quote!();
    //         for field in fields.named.iter_mut() {
    //             // We know it has an identifier because it's a named field
    //             let field_name = field.ident.as_ref().unwrap();
    //             // Check if this field was registered as one to use nested
    // reactivity             if
    // nested_fields_map.contains_key(field.ident.as_ref().unwrap()) {
    //                 field_assignments.extend(quote! {
    //                     #field_name: self.#field_name.make_rx(),
    //                 })
    //             } else {
    //                 field_assignments.extend(quote! {
    //                     #field_name:
    // ::sycamore::prelude::create_rc_signal(self.#field_name),             
    // });             }
    //         }
    //         quote! {
    //             #mid_name {
    //                 #field_assignments
    //             }
    //         }
    //     }
    //     syn::Fields::Unit => quote!(#mid_name),
    //     // We filtered out the other types before
    //     _ => unreachable!(),
    // };
    // // Create a list of fields for turning the intermediary `struct` into one
    // using // scoped references
    // let make_ref_fields = match mid_struct.fields {
    //     syn::Fields::Named(ref mut fields) => {
    //         let mut field_assignments = quote!();
    //         for field in fields.named.iter_mut() {
    //             // We know it has an identifier because it's a named field
    //             let field_name = field.ident.as_ref().unwrap();
    //             // Check if this field was registered as one to use nested
    // reactivity             if
    // nested_fields_map.contains_key(field.ident.as_ref().unwrap()) {
    //                 field_assignments.extend(quote! {
    //                     #field_name: ::sycamore::prelude::create_ref(cx,
    // self.#field_name.to_ref_struct(cx)),                 })
    //             } else {
    //                 // This will be used in a place in which the `cx`
    // variable stores a reactive                 // scope
    //                 field_assignments.extend(quote! {
    //                     #field_name: ::sycamore::prelude::create_ref(cx,
    // self.#field_name),                 });
    //             }
    //         }
    //         quote! {
    //             #ref_name {
    //                 #field_assignments
    //             }
    //         }
    //     }
    //     syn::Fields::Unit => quote!(#ref_name),
    //     // We filtered out the other types before
    //     _ => unreachable!(),
    // };
    // let make_unrx_fields = match orig_struct.fields {
    //     syn::Fields::Named(ref mut fields) => {
    //         let mut field_assignments = quote!();
    //         for field in fields.named.iter_mut() {
    //             // We know it has an identifier because it's a named field
    //             let field_name = field.ident.as_ref().unwrap();
    //             // Check if this field was registered as one to use nested
    // reactivity             if
    // nested_fields_map.contains_key(field.ident.as_ref().unwrap()) {
    //                 field_assignments.extend(quote! {
    //                     #field_name: self.#field_name.clone().make_unrx(),
    //                 })
    //             } else {
    //                 // We can `.clone()` the field because we implement
    // `Clone` on both the new and                 // the original
    // `struct`s, meaning all fields must also be `Clone`                 
    // field_assignments.extend(quote! {                     #field_name:
    // (*self.#field_name.get_untracked()).clone(),                 });
    //             }
    //         }
    //         quote! {
    //             #orig_name {
    //                 #field_assignments
    //             }
    //         }
    //     }
    //     syn::Fields::Unit => quote!(#orig_name),
    //     // We filtered out the other types before
    //     _ => unreachable!(),
    // };

    // quote! {
    //     // We add a Serde derivation because it will always be necessary for
    // Perseus on the original `struct`, and it's really difficult and brittle
    // to filter it out     // #[derive(::serde::Serialize,
    // ::serde::Deserialize, ::std::clone::Clone)]     // #orig_struct
    //     impl #generics ::perseus::state::MakeRx for #orig_name #generics {
    //         type Rx = #mid_name #generics;
    //         fn make_rx(self) -> #mid_name #generics {
    //             use ::perseus::state::MakeRx;
    //             #make_rx_fields
    //         }
    //     }
    //     #[derive(::std::clone::Clone)]
    //     #mid_struct
    //     impl #generics ::perseus::state::MakeUnrx for #mid_name #generics {
    //         type Unrx = #orig_name #generics;
    //         fn make_unrx(self) -> #orig_name #generics {
    //             use ::perseus::state::MakeUnrx;
    //             #make_unrx_fields
    //         }
    //     }
    //     impl #generics ::perseus::state::Freeze for #mid_name #generics {
    //         fn freeze(&self) -> ::std::string::String {
    //             use ::perseus::state::MakeUnrx;
    //             let unrx = #make_unrx_fields;
    //             // TODO Is this `.unwrap()` safe?
    //             ::serde_json::to_string(&unrx).unwrap()
    //         }
    //     }
    //     // TODO Generics
    //     impl ::perseus::state::MakeRxRef for #mid_name {
    //         type RxRef<'a> = #ref_name<'a>;
    //         fn to_ref_struct<'a>(self, cx: ::sycamore::prelude::Scope<'a>) ->
    // #ref_name<'a> {             #make_ref_fields
    //         }
    //     }
    //     #[derive(::std::clone::Clone)]
    //     #ref_struct
    //     impl<'a> ::perseus::state::RxRef for #ref_name<'a> {
    //         type RxNonRef = #mid_name;
    //     }
    // }
}
