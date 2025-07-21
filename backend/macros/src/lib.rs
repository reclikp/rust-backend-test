use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn require_role(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let args_stream = proc_macro2::TokenStream::from(args);
    dbg!(&args_stream);

    if args_stream.is_empty() {
        return syn::Error::new_spanned(
            &input_fn,
            "require_role expects a role argument"
        )
            .to_compile_error()
            .into();
    }

    let required_role = args_stream.to_string().trim_matches('"').to_string();

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_outputs = &input_fn.sig.output;
    let fn_generics = &input_fn.sig.generics;
    let fn_body = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    let expanded_fn = quote! {
        #(#fn_attrs)*
        #fn_vis async fn #fn_name #fn_generics(
            auth_user: backend::auth::AuthenticatedUser,
            #fn_inputs
        ) #fn_outputs {

            if  !auth_user.has_role(#required_role) {
                return Err(ErrorResponse::new(
                    Status::Forbidden,
                    "Insufficient permissions",
                ));
            }

            async move #fn_body.await
        }
    };

    TokenStream::from(expanded_fn)

    // let expand = quote! {
    //     #input_fn
    // };
    //
    // TokenStream::from(expand)
}
