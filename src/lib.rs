#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

/// Derive an implementation of [`Hash`] that relies solely on an enum's discriminant,
/// rather than its contents.
///
/// [`Hash`]: core::hash::Hash
#[proc_macro_derive(Hash)]
pub fn derive_hash(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);

    if !matches!(item.data, Data::Enum(..)) {
        return Error::new(
            Span::call_site(),
            "Cannot derive discriminant Hash for non-enum types. Did you mean to use the built-in Hash trait?",
        ).into_compile_error().into();
    }

    let ident = item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics ::core::hash::Hash for #ident #ty_generics #where_clause {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                ::std::mem::discriminant(self).hash(state);
            }
        }
    })
}

/// Derive an implementation of [`PartialEq`] that relies solely on an enum's discriminant,
/// rather than its contents.
///
/// [`PartialEq`]: core::cmp::PartialEq
#[proc_macro_derive(PartialEq)]
pub fn derive_partial_eq(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);

    if !matches!(item.data, Data::Enum(..)) {
        return Error::new(
            Span::call_site(),
            "Cannot derive discriminant PartialEq for non-enum types. Did you mean to use the built-in PartialEq trait?",
        ).into_compile_error().into();
    }

    let ident = item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics ::core::cmp::PartialEq for #ident #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                ::std::mem::discriminant(self) == ::std::mem::discriminant(other)
            }
        }
    })
}
