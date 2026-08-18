use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{ImplItem, Item, Token, TraitItem};

fn apply_async_to_fn_sig(
    sig: &mut syn::Signature,
    make_async: bool,
) -> syn::Result<()>
{
    if make_async
    {
        if sig.asyncness.is_none()
        {
            sig.asyncness = Some(Token![async](Span::call_site()));
        }
    }
    else
    {
        sig.asyncness = None;
    }
    Ok(())
}

fn apply_to_item(item: &mut Item, make_async: bool) -> syn::Result<()>
{
    match item
    {
        Item::Fn(item_fn) =>
        {
            apply_async_to_fn_sig(&mut item_fn.sig, make_async)
        },
        Item::Impl(item_impl) =>
        {
            for impl_item in &mut item_impl.items
            {
                apply_to_impl_item(impl_item, make_async)?;
            }
            Ok(())
        },
        Item::Trait(item_trait) =>
        {
            for trait_item in &mut item_trait.items
            {
                apply_to_trait_item(trait_item, make_async)?;
            }
            Ok(())
        },
        other =>
        {
            if !make_async
            {
                Ok(())
            }
            else
            {
                Err(syn::Error::new_spanned(
                    other.to_token_stream(),
                    "#[async_] can only be applied to functions",
                ))
            }
        },
    }
}

fn apply_to_impl_item(item: &mut ImplItem, make_async: bool)
-> syn::Result<()>
{
    match item
    {
        ImplItem::Fn(item_fn) =>
        {
            apply_async_to_fn_sig(&mut item_fn.sig, make_async)
        },
        other =>
        {
            if !make_async
            {
                Ok(())
            }
            else
            {
                Err(syn::Error::new_spanned(
                    other.to_token_stream(),
                    "#[async_] can only be applied to functions",
                ))
            }
        },
    }
}

fn apply_to_trait_item(
    item: &mut TraitItem,
    make_async: bool,
) -> syn::Result<()>
{
    match item
    {
        TraitItem::Fn(item_fn) =>
        {
            apply_async_to_fn_sig(&mut item_fn.sig, make_async)
        },
        other =>
        {
            if !make_async
            {
                Ok(())
            }
            else
            {
                Err(syn::Error::new_spanned(
                    other.to_token_stream(),
                    "#[async_] can only be applied to functions",
                ))
            }
        },
    }
}

pub fn async_(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    let tokens: TokenStream2 = item.into();
    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match apply_to_item(&mut item, true)
        {
            Ok(()) => item.into_token_stream().into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(item_err) =>
        {
            if let Ok(mut item) = syn::parse2::<ImplItem>(tokens.clone())
            {
                return match apply_to_impl_item(&mut item, true)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens)
            {
                return match apply_to_trait_item(&mut item, true)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}

pub fn sync_(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    let tokens: TokenStream2 = item.into();
    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match apply_to_item(&mut item, false)
        {
            Ok(()) => item.into_token_stream().into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(item_err) =>
        {
            if let Ok(mut item) = syn::parse2::<ImplItem>(tokens.clone())
            {
                return match apply_to_impl_item(&mut item, false)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens)
            {
                return match apply_to_trait_item(&mut item, false)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}
