use std::fs::File;
use std::io::BufReader;
use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::{Map, Value};
use syn::{LitStr, Token};

#[derive(Parse)]
pub struct IodineInput {
    filename: LitStr,
    _comma_1: Token![,],
    test_func: syn::Ident,
    _comma_2: Token![,],
    test_arg_type: syn::Ident,
}


pub fn generate_test_tokens(input: IodineInput) -> TokenStream {

    let IodineInput {filename, test_func, test_arg_type, ..} = input;

    // TODO use cargo-manifest-dir
    let cwd = std::env::current_dir().unwrap();
    let file_path = cwd.join(&filename.value());

    let file_path_str = format!("{}", &file_path.display());
    let mut output = quote! {
        // include_str! is abused here to force a recompile whenever [IodineInput.filename] changes.
        // Source: https://stackoverflow.com/a/58823695
        const UNUSED_TEST_FILE: &'static str = include_str!(#file_path_str);
    };

    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);
    let tests: Map<String, Value> = serde_json::from_reader(reader).unwrap();
    for (test_name, test) in tests {

        let test_name = format_ident!("{test_name}");
        let data_str = serde_json::to_string(&test).unwrap();

        let tokens = quote! {
            #[test]
            pub fn #test_name() {
                let data: #test_arg_type = serde_json::from_str(#data_str).unwrap();
                #test_func(data);
            }
        };

        output.extend(tokens)
    }

    output
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use super::*;

    #[test]
    fn test_generation() {
        pub struct TestInput {
            to_print: String,
        }

        pub fn test(input: TestInput) {
            println!("{}", input.to_print)
        }

        let tokens = generate_test_tokens(IodineInput {
            filename: LitStr::new("src/test.json", Span::call_site()),
            _comma_1: syn::token::Comma {
                spans: [Span::call_site()],
            },
            test_func: syn::Ident::new("test", Span::call_site()),
            _comma_2: syn::token::Comma {
                spans: [Span::call_site()],
            },
            test_arg_type: syn::Ident::new("TestInput", Span::call_site()),
        });

        println!("{tokens:?}")
    }
}
