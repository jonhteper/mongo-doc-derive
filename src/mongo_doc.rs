use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn new_ident(original_ident: &Ident) -> Ident {
    let new_ident_str = format!("{original_ident}Doc");
    let new_ident = Ident::new(&new_ident_str, original_ident.span());

    new_ident
}

pub fn mongo_doc_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let original_ident = input.ident;
    let new_ident = new_ident(&original_ident);

    let fields = if let Data::Struct(data_struct) = input.data {
        data_struct.fields
    } else {
        panic!("MongoDoc only supports structs with named fields");
    };

    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        panic!("MongoDoc only supports structs with named fields");
    };

    let mut struct_fields = Vec::new();
    let mut pair_fields = Vec::new();
    for field in fields {
        let field_name = &field.ident.unwrap();

        let field_type = &field.ty;

        if &field_name.to_string() == "id" {
            let id_field = quote! {
                #[serde(rename = "_id")]
            };

            struct_fields.push(quote! {
                #id_field
                pub #field_name: #field_type,
            });
        } else {
            struct_fields.push(quote! {
                pub #field_name: #field_type,
            });
        }

        pair_fields.push(quote! {
            #field_name: values.#field_name,
        });
    }

    let impl_from = quote! {
        impl From<#new_ident> for #original_ident {
            fn from(values: #new_ident) -> Self {
                Self {
                    #(#pair_fields)*
                }
            }
        }

    };

    let expanded = quote! {
        #[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, PartialEq)]
        pub struct #new_ident {
            #(#struct_fields)*
        }

        #impl_from
    };

    TokenStream::from(expanded)
}
