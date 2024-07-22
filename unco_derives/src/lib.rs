use proc_macro::TokenStream;
mod schema;

#[proc_macro]
pub fn schema(input: TokenStream) -> TokenStream {
    schema::derive(input)
}
