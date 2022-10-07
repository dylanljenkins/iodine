use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{LitStr, Token};

#[derive(Parse)]
pub struct IodineInput {
    filename: LitStr,
    _comma_1: Token![,],
    test: syn::Ident,
    _comma_2: Token![,],
    input_type: syn::Ident,
}

pub fn generate_test_tokens(input: IodineInput) -> TokenStream {
    let filename = input.filename;
    let test = input.test;
    let test_input_type = input.input_type;

    let output = quote! {
        #[test]
        pub fn test1() {
            println!(#filename);
            #test()
        }
    };

    output
}

#[cfg(test)]
mod tests {
    use crate::{generate_test_tokens, IodineInput};
    use proc_macro2::Span;
    use syn::LitStr;

    #[test]
    fn test_generation() {
        pub struct TestInput {
            to_print: String,
        }

        pub fn test(input: TestInput) {
            println!("{}", input.to_print)
        }

        let tokens = generate_test_tokens(IodineInput {
            filename: LitStr::new("test", Span::call_site()),
            _comma_1: syn::token::Comma {
                spans: [Span::call_site()],
            },
            test: syn::Ident::new("test", Span::call_site()),
            _comma_2: syn::token::Comma {
                spans: [Span::call_site()],
            },
            input_type: syn::Ident::new("TestInput", Span::call_site()),
        });

        println!("{tokens:?}")
    }
}
