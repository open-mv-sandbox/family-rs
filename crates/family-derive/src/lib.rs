use proc_macro::{self, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

/// Derive `Member` and generate `Family` for common cases.
///
/// Implements `Family` for the following cases:
/// - Owned types, as `Type`
/// - Borrowed types with one lifetime, as `TypeF`
#[proc_macro_derive(Member)]
pub fn derive_member(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput {
        vis,
        ident,
        generics,
        ..
    } = input;

    let lifetimes = generics.lifetimes().count();
    let (family_ident, generics_gat) = match lifetimes {
        0 => (ident.clone(), quote! {}),
        1 => (
            Ident::new(&format!("{}F", ident), Span::call_site().into()),
            quote! { <'a> },
        ),
        _ => panic!("derive macro only supports 0 or 1 lifetimes"),
    };

    // Create the family type if necessary
    let family_type = if family_ident != ident {
        let docs = format!("`Family` implementation for `{}`.", ident);
        quote! {
            #[doc = #docs]
            #vis enum #family_ident {}
        }
    } else {
        quote! {}
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let output = quote! {
        #family_type

        impl #impl_generics ::family::Member<#family_ident> for #ident #ty_generics #where_clause {}

        impl ::family::Family for #family_ident {
            type Member<'a> = #ident #generics_gat;
        }
    };
    output.into()
}
