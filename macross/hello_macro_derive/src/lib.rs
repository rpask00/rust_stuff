extern crate proc_macro;
use quote::quote;
use syn;


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast);
}

fn impl_hello_macro(ast  :&syn::DeriveInput){
    let name = &ast.ident;

}