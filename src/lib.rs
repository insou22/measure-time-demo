use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn measure_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block }
        = parse_macro_input!(item as ItemFn);

    let fn_name = sig.ident.to_string();
    let start = quote!(__measure_time_start_instant);

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            let #start = ::std::time::Instant::now();
            let ret = #block;
            
            ::std::println!("{} took: {:#?}", #fn_name, ::std::time::Instant::now().duration_since(#start));
            
            ret
        }
    };

    TokenStream::from(expanded)
}