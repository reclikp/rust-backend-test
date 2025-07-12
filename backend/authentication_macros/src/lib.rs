extern crate proc_macro;

// use backend_app::
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Meta, LitStr, punctuated::Punctuated, Token, ItemFn};

#[proc_macro_attribute]
pub fn require_role(args: TokenStream, input: TokenStream) -> TokenStream {
    dbg!("JEBANKO");
    dbg!(&args);

    let role_list: Punctuated<LitStr, Token![,]> = parse_macro_input!(args with Punctuated::parse_terminated);
    let mut func = parse_macro_input!(input as ItemFn);

    let roles: Vec<String> = role_list.iter().map(|lit| lit.value()).collect();

    dbg!(role_list);
    dbg!(roles);
    dbg!(func.sig.inputs);

    let func_name = &func.sig.ident;
    let vis = &func.vis;
    let attrs = &func.attrs;
    let output_type = &func.sig.output;
    let route_attrs = attrs.iter().filter(|a| a.path().is_ident("get") || a.path().is_ident("post"));

    let new_func_name = format_ident!("__{}_inner", func_name);
    func.sig.ident = new_func_name.clone();

    let func_block = &func.block;

    let new_function_body = quote! {
        #func

        #(#route_attrs)
        #vis async fn #func_name<'r>(req: &'r rocket::Request<'_>) -> #output_type {
        // #vis async fn #func_name<'r>(_jwt: JWT) -> #output_type {
            use rocket::request::{FromRequest, Outcome};
            use rocket::http::Status;
            use your_crate::middleware::jwt::JWT;

            let jwt = match JWT::from_request(req).await {
                Outcome::Success(token) => token,
                _ => return Err(Status::Unauthorized),
            };

            let mut authorized = false;
            // #(#role_check)*

            if !authorized {
                return Err(Status::Unauthorized);
            }

            #new_func_name().await
        }
    };

    new_function_body.into()
}


// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
