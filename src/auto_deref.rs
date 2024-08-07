use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(auto_deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDerefFieldInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}

#[allow(unused)]
#[derive(Debug, FromField)]
#[darling(attributes(auto_deref))]
struct AutoDerefFieldInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDeref only works on structs");
    };
    let (fd, ty) = if let Some(field) = field {
        match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
            Some(f) => (field, &f.ty),
            None => {
                panic!("field {:?} not found in the data structure", field)
            }
        }
    } else if fields.len() == 1 {
        let f = fields.iter().next().unwrap();
        let field = f.ident.clone().unwrap();
        (field, &f.ty)
    } else {
        panic!("AutoDeref only works on structs with exactly one field or with field attributes")
    };

    // build the output tokens
    // quote! returns proc_macro2 TokenStream, so we need to convert it to TokenStream
    let mut code = vec![quote! {
        impl #generics ::std::ops::Deref for #ident #generics {
            type Target = #ty;
            fn deref(&self) -> &Self::Target {
                &self.#fd
            }
        }
    }];

    if mutable {
        code.push(quote! {
            impl #generics ::std::ops::DerefMut for #ident #generics {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#fd
                }
            }
        });
    }

    quote! {
        #( #code )*
    }
}
