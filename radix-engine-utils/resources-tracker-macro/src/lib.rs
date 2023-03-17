use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    parse::{Parse, Parser},
    parse_quote
};


#[cfg(not(feature = "resource_tracker"))]
#[proc_macro_attribute]
pub fn trace_resources(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[cfg(target_family = "unix")]
#[cfg(feature = "resource_tracker")]
#[proc_macro_attribute]
pub fn trace_resources(attr: TokenStream, input: TokenStream) -> TokenStream {
    let _arg = if let Ok(attrs) = syn::Ident::parse.parse(attr) {
        quote!{ #attrs }
    } else {
        quote!{ "" }
    };

    let output = if let Ok(mut item) = syn::Item::parse.parse(input.clone()) {
        match item {
            syn::Item::Fn(ref mut item_fn) => {
                let original_block = &mut item_fn.block;
                let fn_signature = item_fn.sig.ident.to_string();
                item_fn.block = Box::new( parse_quote! {{ 
                    QEMU_PLUGIN.with(|v| {
                        // let stack = v.borrow().get_current_stack();
                        // let spaces = [' '; 40];
                        // //let space = std::iter::repeat(' ').take(4 * stack).collect::<String>();
                        // println!("[rtrack]{}++enter: {} {} {}", spaces[], #fn_signature, stack + 1, #arg);
                        v.borrow_mut().start_counting(#fn_signature);
                    });
                    let ret = #original_block;
                    QEMU_PLUGIN.with(|v| {
                        let (stack, cnt) = v.borrow_mut().stop_counting(#fn_signature);
                        //let space = std::iter::repeat(' ').take(4 * stack).collect::<String>();
                        //println!("[rtrack]{}--exit: {} {} {} {}", space, #fn_signature, stack, cnt, #arg);
                    });
                    ret
                }} );
                item.into_token_stream()
            }
            _ => syn::Error::new_spanned(item, "#[trace] is not supported for this item")
                .to_compile_error(),
        }

    } else {
        let input2 = proc_macro2::TokenStream::from(input);
        syn::Error::new_spanned(input2, "expected one of: `fn`, `impl`, `mod`").to_compile_error()
    };

    output.into()
}
