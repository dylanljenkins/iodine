use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

#[derive(Parse)]
pub struct IodineInput {
    pub filename: LitStr,
}

pub fn generate_test_tokens(input: IodineInput) -> TokenStream {
    let filename = input.filename;

    let output = quote! {
        #[test]
        pub fn test1() {
            println!(#filename)
        }

        #[test]
        pub fn test2() {
            println!(#filename)
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
    pub fn test_generation() {
        let tokens = generate_test_tokens(IodineInput {
            filename: LitStr::new("test", Span::call_site()),
        });

        println!("{tokens:?}")
    }
}
