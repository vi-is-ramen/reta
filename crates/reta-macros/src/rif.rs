use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Meta, parse_macro_input};

use crate::cfg::eval_meta;

struct RetaIfInput
{
    condition: Meta,
}

impl Parse for RetaIfInput
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let condition: Meta = input.parse()?;

        if !input.is_empty()
        {
            return Err(input.error("unexpected tokens after condition"));
        }

        Ok(RetaIfInput { condition })
    }
}

pub fn reta_if(input: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as RetaIfInput);

    match eval_meta(input.condition)
    {
        crate::cfg::CfgResult::Known(true) =>
        {
            // Condition is true at macro expansion time
            quote! { true }.into()
        },
        crate::cfg::CfgResult::Known(false) =>
        {
            // Condition is false at macro expansion time
            quote! { false }.into()
        },
        crate::cfg::CfgResult::Unknown(remaining_meta) =>
        {
            // Condition depends on compiler cfgs, delegate to compiler
            quote! { cfg!(#remaining_meta) }.into()
        },
    }
}
