use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    let fn_name = &fn_sig.ident;

    let is_async = fn_sig.asyncness.is_some();
    let fn_name_str = fn_name.to_string();

    let timed_block = if is_async {
        quote! {
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
    } else {
        quote! {
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

    let original_block = if is_async {
        quote! { async { #fn_block }.await }
    } else {
        quote! { #fn_block }
    };

    let generated_code = quote! {
        #fn_vis #fn_sig {
            if std::env::var("ENABLE_FUNCTION_TIMER").map_or(true, |s| s.eq_ignore_ascii_case("true") || s == "1") {
                #timed_block
            } else {
                #original_block
            }
        }
    };

    generated_code.into()
}
