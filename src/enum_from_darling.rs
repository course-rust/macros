// #![allow(dead_code)]

use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarlingInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: Fields<EnumVariantFields>,
}
#[derive(Debug, FromField)]
struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarlingInput {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarlingInput::from_derive_input(&input).unwrap()
    else {
        panic!("EnumFromDarling only works on enums")
    };
    let from_impls = data.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let style = &variant.fields.style;
        match style {
            Style::Tuple => {
                if variant.fields.len() == 1 {
                    let field = variant.fields.iter().next().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(value: #ty) -> Self {
                                #ident::#variant_ident(value)
                            }
                        }
                    }
                } else {
                    quote! {}
                }
            }
            Style::Struct => quote! {},
            Style::Unit => quote! {},
        }
    });

    quote! {
        #( #from_impls )*
    }
}
