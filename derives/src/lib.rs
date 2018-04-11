extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Ident};

#[proc_macro_derive(Cursor)]
pub fn cursor(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    if let Data::Struct(data) = ast.data {
        if data.fields
            .iter()
            .any(|f| f.ident == Some(Ident::from("cursor")))
        {
            let code = quote! {
                impl Cursor for #name {
                    fn with_cursor(mut self, cursor: &str) -> #name {
                        self.cursor = Some(cursor.to_string());
                        self
                    }

                    fn cursor(&self) -> Option<&str> {
                        // Take the cursor as a ref, then map the string to a slice. Needs
                        // two derefs to deref to String then to str then return reference
                        self.cursor.as_ref().map(|s| &**s)
                    }
                }
            };
            code.into()
        } else {
            panic!(
                "#[derive(Cursor)] is only valid for structs that define `cursor: Option<String>`"
            )
        }
    } else {
        panic!("#[derive(Cursor)] is only valid for structs")
    }
}

#[proc_macro_derive(Limit)]
pub fn limit(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    if let Data::Struct(data) = ast.data {
        if data.fields
            .iter()
            .any(|f| f.ident == Some(Ident::from("limit")))
        {
            let code = quote! {
                impl Limit for #name {
                    fn with_limit(mut self, limit: u32) -> #name {
                        self.limit = Some(limit);
                        self
                    }

                    fn limit(&self) -> Option<u32> {
                        self.limit
                    }
                }
            };
            code.into()
        } else {
            panic!("#[derive(Limit)] is only valid for structs that define `limit: Option<u32>`")
        }
    } else {
        panic!("#[derive(Limit)] is only valid for structs")
    }
}
