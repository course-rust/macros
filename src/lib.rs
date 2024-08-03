// proc macro create
use proc_macro::TokenStream;
use quote::quote;

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);
    // get the ident
    let ident = input.ident;
    // get generics
    let generics = input.generics;

    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only supports enums"),
    };
    // for each variant, we need to get ident and field
    let from_impls = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().unwrap();
                    let ty = &field.ty;
                    // generate From impl
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(value: #ty) -> Self {
                                #ident::#variant_ident(value)
                            }
                        }
                    }
                }
            }
            syn::Fields::Named(_) => quote! {},
            syn::Fields::Unit => quote! {},
        }
    });
    // generate From impls for each variant
    // then return a new TokenStream with all From impls
    // quote! returns proc_macro2 TokenStream, so we need to convert it to TokenStream
    quote! {
        #( #from_impls )*
    }
    .into()
}
// for struct, we'd like to generate From impls for each field
