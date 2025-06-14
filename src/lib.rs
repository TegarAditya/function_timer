use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;

    let is_async = fn_sig.asyncness.is_some();
    let fn_name_str = fn_name.to_string();

    if is_async {
        let generate = quote! {
            #fn_vis #fn_sig {
                let start = std::time::Instant::now();
                let result = async { #fn_block }.await;
                let duration = start.elapsed();
                tracing::info!(
                    "--- Function '{}' executed in {}ms",
                    #fn_name_str,
                    duration.as_millis()
                );
                result
            }
        };
        generate.into()
    } else {
        let generate = quote! {
            #fn_vis #fn_sig {
                let start = std::time::Instant::now();
                let result = { #fn_block };
                let duration = start.elapsed();
                tracing::info!(
                    "--- Function '{}' executed in {}ms",
                    #fn_name_str,
                    duration.as_millis()
                );
                result
            }
        };
        generate.into()
    }
}
