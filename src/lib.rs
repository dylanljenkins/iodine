use iodine_core::IodineInput;
use syn::parse_macro_input;

#[proc_macro]
pub fn generate_tests(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: IodineInput = parse_macro_input!(input);

    let output = iodine_core::generate_test_tokens(input);

    proc_macro::TokenStream::from(output)
}
