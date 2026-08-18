use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Item, Meta, Token, parse_macro_input};

struct ChannelCondition
{
    negated: bool,
    channel: Ident,
}

impl Parse for ChannelCondition
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let mut negated = false;

        if input.peek(Token![!])
        {
            let _: Token![!] = input.parse()?;
            negated = true;
        }
        else if input.peek(Ident)
        {
            let fork = input.fork();
            if let Ok(ident) = fork.parse::<Ident>()
                && ident == "not"
            {
                let _: Ident = input.parse()?;
                let content;
                syn::parenthesized!(content in input);
                let channel: Ident = content.parse()?;
                return Ok(ChannelCondition {
                    negated: true,
                    channel,
                });
            }
        }

        let channel: Ident = input.parse()?;
        Ok(ChannelCondition { negated, channel })
    }
}

#[allow(clippy::large_enum_variant)] // I know!
pub(crate) enum CfgResult
{
    Known(bool),
    Unknown(Meta),
}

pub(crate) fn eval_meta(meta: Meta) -> CfgResult
{
    match meta
    {
        Meta::Path(path) => CfgResult::Unknown(Meta::Path(path)),
        Meta::NameValue(nv) => CfgResult::Unknown(Meta::NameValue(nv)),
        Meta::List(list) =>
        {
            let path_str = list.path.to_token_stream().to_string();

            // Evaluate channel(...) at macro expansion time
            if path_str == "channel"
                && let Ok(cond) =
                    syn::parse2::<ChannelCondition>(list.tokens.clone())
            {
                let is_match = match cond.channel.to_string().as_str()
                {
                    "nightly" => cfg!(nightly),
                    "beta" => cfg!(beta),
                    "stable" => cfg!(stable),
                    _ => false,
                };
                let result = if cond.negated { !is_match } else { is_match };
                return CfgResult::Known(result);
            }

            // Recursively evaluate nested conditions (all, any, not)
            let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
            if let Ok(nested) = parser.parse2(list.tokens.clone())
            {
                let mut evaluated = Vec::new();
                let mut has_known_true = false;
                let mut has_known_false = false;

                for m in nested
                {
                    match eval_meta(m)
                    {
                        CfgResult::Known(true) => has_known_true = true,
                        CfgResult::Known(false) => has_known_false = true,
                        CfgResult::Unknown(m) => evaluated.push(m),
                    }
                }

                if path_str == "all"
                {
                    if has_known_false
                    {
                        return CfgResult::Known(false);
                    }
                    if evaluated.is_empty()
                    {
                        return CfgResult::Known(true);
                    }
                    if evaluated.len() == 1
                    {
                        return CfgResult::Unknown(evaluated.pop().unwrap());
                    }
                    let tokens = quote! { #(#evaluated),* };
                    return CfgResult::Unknown(Meta::List(syn::MetaList {
                        path: list.path,
                        delimiter: list.delimiter,
                        tokens,
                    }));
                }
                else if path_str == "any"
                {
                    if has_known_true
                    {
                        return CfgResult::Known(true);
                    }
                    if evaluated.is_empty()
                    {
                        return CfgResult::Known(false);
                    }
                    if evaluated.len() == 1
                    {
                        return CfgResult::Unknown(evaluated.pop().unwrap());
                    }
                    let tokens = quote! { #(#evaluated),* };
                    return CfgResult::Unknown(Meta::List(syn::MetaList {
                        path: list.path,
                        delimiter: list.delimiter,
                        tokens,
                    }));
                }
                else if path_str == "not"
                {
                    if has_known_true
                    {
                        return CfgResult::Known(false);
                    }
                    if has_known_false
                    {
                        return CfgResult::Known(true);
                    }
                    if evaluated.len() == 1
                    {
                        let inner = evaluated.pop().unwrap();
                        let tokens = quote! { #inner };
                        return CfgResult::Unknown(Meta::List(syn::MetaList {
                            path: list.path,
                            delimiter: list.delimiter,
                            tokens,
                        }));
                    }
                }
            }

            CfgResult::Unknown(Meta::List(list))
        },
    }
}

fn add_attr_to_item(item: &mut Item, attr: Attribute) -> syn::Result<()>
{
    match item
    {
        Item::Const(i) => i.attrs.push(attr),
        Item::Enum(i) => i.attrs.push(attr),
        Item::ExternCrate(i) => i.attrs.push(attr),
        Item::Fn(i) => i.attrs.push(attr),
        Item::ForeignMod(i) => i.attrs.push(attr),
        Item::Impl(i) => i.attrs.push(attr),
        Item::Macro(i) => i.attrs.push(attr),
        Item::Mod(i) => i.attrs.push(attr),
        Item::Static(i) => i.attrs.push(attr),
        Item::Struct(i) => i.attrs.push(attr),
        Item::Trait(i) => i.attrs.push(attr),
        Item::TraitAlias(i) => i.attrs.push(attr),
        Item::Type(i) => i.attrs.push(attr),
        Item::Union(i) => i.attrs.push(attr),
        Item::Use(i) => i.attrs.push(attr),
        _ =>
        {
            return Err(syn::Error::new_spanned(
                item.to_token_stream(),
                "unsupported item type for reta/reta_attr",
            ))
        },
    }
    Ok(())
}

pub fn reta(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let meta = parse_macro_input!(attr as Meta);
    let tokens: TokenStream2 = item.into();

    match eval_meta(meta)
    {
        CfgResult::Known(false) =>
        {
            // Condition is false at macro expansion time, remove item entirely
            TokenStream::new()
        },
        CfgResult::Known(true) =>
        {
            // Condition is true, emit item as-is without any #[cfg]
            tokens.into()
        },
        CfgResult::Unknown(remaining_meta) =>
        {
            // Condition depends on compiler cfgs, delegate to compiler
            match syn::parse2::<Item>(tokens.clone())
            {
                Ok(mut item) =>
                {
                    let cfg_attr: Attribute = syn::parse_quote! {
                        #[cfg(#remaining_meta)]
                    };
                    if let Err(e) = add_attr_to_item(&mut item, cfg_attr)
                    {
                        return e.to_compile_error().into();
                    }
                    item.into_token_stream().into()
                },
                Err(err) => err.to_compile_error().into(),
            }
        },
    }
}

struct RetaAttrInput
{
    condition: Meta,
    attr:      Meta,
}

impl Parse for RetaAttrInput
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let condition: Meta = input.parse()?;
        let _: Token![,] = input.parse()?;
        let attr: Meta = input.parse()?;

        if !input.is_empty()
        {
            return Err(input.error("unexpected tokens after attribute"));
        }

        Ok(RetaAttrInput { condition, attr })
    }
}

pub fn reta_attr(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(attr as RetaAttrInput);
    let tokens: TokenStream2 = item.into();

    match eval_meta(input.condition)
    {
        CfgResult::Known(false) =>
        {
            // Condition is false, emit item without the attribute
            tokens.into()
        },
        CfgResult::Known(true) =>
        {
            // Condition is true, apply the attribute directly
            match syn::parse2::<Item>(tokens.clone())
            {
                Ok(mut item) =>
                {
                    let attr_meta = input.attr;
                    let new_attr: Attribute = syn::parse_quote! {
                        #[#attr_meta]
                    };
                    if let Err(e) = add_attr_to_item(&mut item, new_attr)
                    {
                        return e.to_compile_error().into();
                    }
                    item.into_token_stream().into()
                },
                Err(err) => err.to_compile_error().into(),
            }
        },
        CfgResult::Unknown(remaining_meta) =>
        {
            // Condition depends on compiler cfgs, use cfg_attr
            match syn::parse2::<Item>(tokens.clone())
            {
                Ok(mut item) =>
                {
                    let attr_meta = input.attr;
                    let cfg_attr: Attribute = syn::parse_quote! {
                        #[cfg_attr(#remaining_meta, #attr_meta)]
                    };
                    if let Err(e) = add_attr_to_item(&mut item, cfg_attr)
                    {
                        return e.to_compile_error().into();
                    }
                    item.into_token_stream().into()
                },
                Err(err) => err.to_compile_error().into(),
            }
        },
    }
}
