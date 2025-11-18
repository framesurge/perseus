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
    /// If specified, the type should be ignored by HSR.
    #[darling(default)]
    hsr_ignore: bool,

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
    /// This is used to mark fields that have a browser-side handler dedicated
    /// to modifying their value asynchronously. When the page is loaded, the
    /// provided modifier function will be called with an `RcSignal` of this
    /// field's type (even if this is used with `#[rx(nested)]`!).
    ///
    /// The reason handlers are only allowed to work with individual fields is
    /// to enable fine-grained error handling, by forcing users to handle
    /// the possibility that each of their handlers comes up with an error.
    ///
    /// Note that the handler function specified must be asynchronous, but it
    /// will be placed in an abortable future: when the user leaves this
    /// page, any ongoing handlers will be *immmediately* short-circuited.
    /// (You shouldn't have to worry about this unless you're doing
    /// something very advanced.)
    #[darling(default)]
    suspense: Option<Ident>,

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
    let mut intermediate_field_makers = quote!();
    let mut new_intermediate_field_makers = quote!();
    let mut unrx_field_makers = quote!();
    let mut suspense_commands = quote!();
    let mut old_types = quote!();
    for field in fields.iter() {
        let old_ty = field.ty.to_token_stream();
        let field_ident = field.ident.as_ref().unwrap(); // It's a `struct`, so this is defined
        let field_vis = &field.vis;
        let mut field_attrs = quote!();
        for attr in field.attrs.iter() {
            field_attrs.extend(attr.to_token_stream());
        }
        // Old for ::new implementation of intermediate type
        old_types.extend(quote! {
            #field_ident: #old_ty,
        });
        // Nested fields are left as-is, non-nested ones are wrapped in `RcSignal`s
        if field.nested {
            // Nested types should implement the necessary linking traits
            intermediate_fields.extend(quote! {
                #field_attrs
                #field_vis #field_ident: <#old_ty as ::perseus::state::MakeRx>::Rx,
            });
            intermediate_field_makers.extend(quote! { #field_ident: self.#field_ident.make_rx(), });
            new_intermediate_field_makers.extend(quote! { #field_ident: #field_ident.make_rx(), });
            unrx_field_makers
                .extend(quote! { #field_ident: self.#field_ident.clone().make_unrx(), });

            // Handle suspended fields
            if let Some(handler) = &field.suspense {
                // This line calls a utility function that does ergonomic error handling
                suspense_commands.extend(quote! {
                    // The `nested` part makes this expect `RxResult`
                    ::perseus::state::compute_nested_suspense(
                        cx,
                        self.#field_ident.clone(),
                        #handler(
                            cx,
                            ::sycamore::prelude::create_ref(cx, self.#field_ident.clone()),
                        ),
                    );
                });
            } else {
                // If this field is not suspended, it might have suspended children, which we
                // should be sure to compute
                suspense_commands.extend(quote! {
                    self.#field_ident.compute_suspense(cx);
                })
            }
        } else {
            intermediate_fields.extend(quote! {
                #field_attrs
                #field_vis #field_ident: ::sycamore::prelude::RcSignal<#old_ty>,
            });
            intermediate_field_makers.extend(
                quote! { #field_ident: ::sycamore::prelude::create_rc_signal(self.#field_ident), },
            );
            new_intermediate_field_makers.extend(
                quote! { #field_ident: ::sycamore::prelude::create_rc_signal(#field_ident), },
            );
            // All fields must be `Clone`
            unrx_field_makers
                .extend(quote! { #field_ident: (*self.#field_ident.get_untracked()).clone(), });

            // Handle suspended fields (we don't care if they're nested, the user can worry
            // about that (probably using `RxResult` or similar))
            if let Some(handler) = &field.suspense {
                // This line calls a utility function that does ergonomic error handling
                suspense_commands.extend(quote! {
                    // The `nested` part makes this expect `RxResult`
                    ::perseus::state::compute_suspense(
                        cx,
                        self.#field_ident.clone(),
                        #handler(
                            cx,
                            ::sycamore::prelude::create_ref(cx, self.#field_ident.clone()),
                        ),
                    );
                });
            }
        }
    }

    let ReactiveStateDeriveInput {
        ident,
        vis,
        attrs: attrs_vec,
        alias,
        hsr_ignore,
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

    // Create a type alias for the final reactive version for convenience, if the
    // user asked for one
    let ref_alias = if let Some(alias) = alias {
        // // We use the full form for a cleaner expansion in IDEs
        // quote! { #vis type #alias<'__derived_rx> = <<#ident as
        // ::perseus::state::MakeRx>::Rx as
        // ::perseus::state::MakeRxRef>::RxRef<'__derived_rx>; }
        quote! { #vis type #alias = #intermediate_ident; }
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

        impl From<#intermediate_ident> for #ident
        {
            fn from(value: #intermediate_ident) -> #ident
            {
                use ::perseus::state::MakeUnrx;
                value.make_unrx()
            }
        }

        impl From<#ident> for #intermediate_ident
        {
            fn from(value: #ident) -> #intermediate_ident
            {
                use ::perseus::state::MakeRx;
                value.make_rx()
            }
        }

        impl ::perseus::state::MakeRx for #ident {
            type Rx = #intermediate_ident;
            #[cfg(debug_assertions)]
            const HSR_IGNORE: bool = #hsr_ignore;
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
            #[cfg(client)]
            fn compute_suspense<'a>(&self, cx: ::sycamore::prelude::Scope<'a>) {
                #suspense_commands
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

        #ref_alias
    }
}
