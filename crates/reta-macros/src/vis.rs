use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{ExprPath, ImplItem, Item, TraitItem, Visibility, parse_macro_input};

fn set_visibility(item: &mut Item, vis: Visibility) -> syn::Result<()>
{
    let is_inherited = matches!(&vis, Visibility::Inherited);

    match item
    {
        Item::Const(item) => item.vis = vis,
        Item::Enum(item) => item.vis = vis,
        Item::ExternCrate(item) => item.vis = vis,
        Item::Fn(item) => item.vis = vis,
        Item::Mod(item) => item.vis = vis,
        Item::Static(item) => item.vis = vis,
        Item::Struct(item) => item.vis = vis,
        Item::Trait(item) => item.vis = vis,
        Item::TraitAlias(item) => item.vis = vis,
        Item::Type(item) => item.vis = vis,
        Item::Union(item) => item.vis = vis,
        Item::Use(item) => item.vis = vis,
        other =>
        {
            if is_inherited
            {
                return Ok(());
            }
            return Err(syn::Error::new_spanned(
                other.to_token_stream(),
                "this item does not support visibility",
            ));
        },
    }
    Ok(())
}

fn set_impl_item_visibility(
    item: &mut ImplItem,
    vis: Visibility,
) -> syn::Result<()>
{
    let is_inherited = matches!(&vis, Visibility::Inherited);

    match item
    {
        ImplItem::Const(item) => item.vis = vis,
        ImplItem::Fn(item) => item.vis = vis,
        ImplItem::Type(item) => item.vis = vis,
        other =>
        {
            if is_inherited
            {
                return Ok(());
            }
            return Err(syn::Error::new_spanned(
                other.to_token_stream(),
                "this impl item does not support visibility",
            ));
        },
    }
    Ok(())
}

fn set_trait_item_visibility(
    item: &mut TraitItem,
    vis: Visibility,
) -> syn::Result<()>
{
    let is_inherited = matches!(&vis, Visibility::Inherited);

    if is_inherited
    {
        return Ok(());
    }

    Err(syn::Error::new_spanned(
        item.to_token_stream(),
        "trait items cannot have visibility",
    ))
}

fn apply_visibility(item: &mut Item, vis: Visibility) -> syn::Result<()>
{
    set_visibility(item, vis)
}

fn apply_impl_item_visibility(
    item: &mut ImplItem,
    vis: Visibility,
) -> syn::Result<()>
{
    set_impl_item_visibility(item, vis)
}

fn apply_trait_item_visibility(
    item: &mut TraitItem,
    vis: Visibility,
) -> syn::Result<()>
{
    set_trait_item_visibility(item, vis)
}

struct PubInPath
{
    path: ExprPath,
}

impl Parse for PubInPath
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let content;
        syn::parenthesized!(content in input);
        let path: ExprPath = content.parse()?;
        Ok(PubInPath { path })
    }
}

pub fn pub_(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    let tokens: TokenStream2 = item.into();
    let vis: Visibility = syn::parse_quote!(pub);

    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match apply_visibility(&mut item, vis)
        {
            Ok(()) => item.into_token_stream().into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(item_err) =>
        {
            if let Ok(mut item) = syn::parse2::<ImplItem>(tokens.clone())
            {
                return match apply_impl_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens)
            {
                return match apply_trait_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}

pub fn pub_crate(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    let tokens: TokenStream2 = item.into();
    let vis: Visibility = syn::parse_quote!(pub(crate));

    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match apply_visibility(&mut item, vis)
        {
            Ok(()) => item.into_token_stream().into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(item_err) =>
        {
            if let Ok(mut item) = syn::parse2::<ImplItem>(tokens.clone())
            {
                return match apply_impl_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens)
            {
                return match apply_trait_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}

pub fn pub_super(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    let tokens: TokenStream2 = item.into();
    let vis: Visibility = syn::parse_quote!(pub(super));

    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match apply_visibility(&mut item, vis)
        {
            Ok(()) => item.into_token_stream().into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(item_err) =>
        {
            if let Ok(mut item) = syn::parse2::<ImplItem>(tokens.clone())
            {
                return match apply_impl_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens)
            {
                return match apply_trait_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}

pub fn pub_in(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let pub_in_path = parse_macro_input!(attr as PubInPath);
    let tokens: TokenStream2 = item.into();
    let path = &pub_in_path.path;
    let vis: Visibility = syn::parse_quote!(pub(in #path));

    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match apply_visibility(&mut item, vis)
        {
            Ok(()) => item.into_token_stream().into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(item_err) =>
        {
            if let Ok(mut item) = syn::parse2::<ImplItem>(tokens.clone())
            {
                return match apply_impl_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens)
            {
                return match apply_trait_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}

pub fn priv_(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    let tokens: TokenStream2 = item.into();
    let vis: Visibility = Visibility::Inherited;

    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match apply_visibility(&mut item, vis)
        {
            Ok(()) => item.into_token_stream().into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(item_err) =>
        {
            if let Ok(mut item) = syn::parse2::<ImplItem>(tokens.clone())
            {
                return match apply_impl_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens)
            {
                return match apply_trait_item_visibility(&mut item, vis)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}
