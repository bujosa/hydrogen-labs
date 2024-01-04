#[macro_use]
extern crate std;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn generate_functions(input: TokenStream) -> TokenStream {
    // Parse the input as a string literal representing the JSON file path
    let json_path = parse_macro_input!(input as LitStr).value();

    // Read the JSON file content and create compile error if it fails
    let json_content = match std::fs::read_to_string(&json_path) {
        Ok(content) => content,
        Err(err) => {
            let error_message = format!("Failed to read JSON file: {}", err);
            return quote! {
                compile_error!(#error_message);
            }.into()
        }
    };

    // Parse the JSON content and create compile error if it fails
    let json_data: serde_json::Value = match serde_json::from_str(&json_content) {
        Ok(data) => data,
        Err(err) => {
            let error_message = format!("Failed to parse JSON content: {}", err);
            return quote! {
                compile_error!(#error_message);
            }.into()
        }
    };

    // Extract method names from the JSON
    let method_names = match json_data.get("methods").and_then(|methods| methods.as_array()) {
        Some(names) => names
            .iter()
            .map(|name| {
                if let Some(method_name) = name.as_str() {
                    Some(method_name.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        None => {
            return TokenStream::from(quote! {
                compile_error!("Missing 'methods' array in JSON");
            });
        }
    };

    // Generate Rust functions
    let generated_functions = method_names.iter().map(|name| {
        if let Some(ref name_str) = name {
            let name_ident = syn::Ident::new(name_str, proc_macro2::Span::call_site());
            quote! {
                pub fn #name_ident() {
                    println!("Function {} called", #name_str);
                }
            }
        } else {
            quote! {}
        }
    });

    // Combine the generated functions into a TokenStream
    let result = quote! {
        #(#generated_functions)*
    };

    result.into()
}