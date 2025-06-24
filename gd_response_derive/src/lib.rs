use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Data, Fields, LitInt, parse_macro_input};

// #[response(key)]
struct ResponseKey(LitInt);

impl Parse for ResponseKey {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: LitInt = input.parse()?;
        Ok(ResponseKey(key))
    }
}

/// Derive macro to convert a struct to a Geometry Dash response.
#[proc_macro_derive(GDResponse, attributes(response))]
pub fn response_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let mut parts = Vec::new();

    if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                let mut field_key: Option<LitInt> = None;

                for attr in &field.attrs {
                    if attr.path().is_ident("response") {
                        match attr.parse_args::<ResponseKey>() {
                            Ok(rk) => {
                                field_key = Some(rk.0);
                            }
                            Err(e) => {
                                return TokenStream::from(e.to_compile_error());
                            }
                        }
                        break;
                    }
                }

                if let Some(key_int) = field_key {
                    let field_ident = field
                        .ident
                        .as_ref()
                        .expect("Expected named field for #[response] attribute.");

                    let key_str = key_int
                        .base10_parse::<String>()
                        .expect("Failed to parse integer key as string");

                    parts.push(quote! { #key_str.to_string() });
                    parts.push(quote! { self.#field_ident.to_string() });
                }
            }
        }
    }

    // if no fields with the attribute, return an empty string
    if parts.is_empty() {
        return quote! {
            impl #name {
                pub fn to_gd_response(&self, _delimiter: &str) -> String {
                    String::new()
                }
            }
        }
        .into();
    }

    // concatenate the full response string
    let full_response = quote! {
        vec![#(#parts),*].join(delimiter)
    };

    let expanded = quote! {
        impl #name {
            pub fn to_gd_response(&self, delimiter: &str) -> String {
                #full_response
            }
        }
    };

    expanded.into()
}
