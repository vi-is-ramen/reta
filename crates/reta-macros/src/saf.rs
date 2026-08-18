use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{ForeignItem, ImplItem, Item, Token, TraitItem};

fn apply_safety_to_fn_sig(
    sig: &mut syn::Signature,
    make_unsafe: bool,
) -> syn::Result<()>
{
    if make_unsafe
    {
        sig.safety = syn::Safety::Unsafe(Token![unsafe](Span::call_site()));
    }
    else
    {
        sig.safety = syn::Safety::Safe(Token![safe](Span::call_site()));
    }
    Ok(())
}

fn apply_to_item(item: &mut Item, make_unsafe: bool) -> syn::Result<()>
{
    match item
    {
        Item::Fn(item_fn) =>
        {
            apply_safety_to_fn_sig(&mut item_fn.sig, make_unsafe)
        },
        Item::Impl(item_impl) =>
        {
            for impl_item in &mut item_impl.items
            {
                apply_to_impl_item(impl_item, make_unsafe)?;
            }
            Ok(())
        },
        Item::Trait(item_trait) =>
        {
            for trait_item in &mut item_trait.items
            {
                apply_to_trait_item(trait_item, make_unsafe)?;
            }
            Ok(())
        },
        Item::ForeignMod(item_foreign) =>
        {
            for foreign_item in &mut item_foreign.items
            {
                apply_to_foreign_item(foreign_item, make_unsafe)?;
            }
            Ok(())
        },
        other =>
        {
            if !make_unsafe
            {
                Ok(())
            }
            else
            {
                Err(syn::Error::new_spanned(
                    other.to_token_stream(),
                    "#[unsafe_] can only be applied to functions",
                ))
            }
        },
    }
}

fn apply_to_impl_item(item: &mut ImplItem, make_unsafe: bool)
-> syn::Result<()>
{
    match item
    {
        ImplItem::Fn(item_fn) =>
        {
            apply_safety_to_fn_sig(&mut item_fn.sig, make_unsafe)
        },
        other =>
        {
            if !make_unsafe
            {
                Ok(())
            }
            else
            {
                Err(syn::Error::new_spanned(
                    other.to_token_stream(),
                    "#[unsafe_] can only be applied to functions",
                ))
            }
        },
    }
}

fn apply_to_trait_item(
    item: &mut TraitItem,
    make_unsafe: bool,
) -> syn::Result<()>
{
    match item
    {
        TraitItem::Fn(item_fn) =>
        {
            apply_safety_to_fn_sig(&mut item_fn.sig, make_unsafe)
        },
        other =>
        {
            if !make_unsafe
            {
                Ok(())
            }
            else
            {
                Err(syn::Error::new_spanned(
                    other.to_token_stream(),
                    "#[unsafe_] can only be applied to functions",
                ))
            }
        },
    }
}

fn apply_to_foreign_item(
    item: &mut ForeignItem,
    make_unsafe: bool,
) -> syn::Result<()>
{
    match item
    {
        ForeignItem::Fn(item_fn) =>
        {
            apply_safety_to_fn_sig(&mut item_fn.sig, make_unsafe)
        },
        other =>
        {
            if !make_unsafe
            {
                Ok(())
            }
            else
            {
                Err(syn::Error::new_spanned(
                    other.to_token_stream(),
                    "#[unsafe_] can only be applied to functions",
                ))
            }
        },
    }
}

pub fn unsafe_(_attr: TokenStream, item: TokenStream) -> TokenStream
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
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens.clone())
            {
                return match apply_to_trait_item(&mut item, true)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<ForeignItem>(tokens)
            {
                return match apply_to_foreign_item(&mut item, true)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}

pub fn safe_(_attr: TokenStream, item: TokenStream) -> TokenStream
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
            if let Ok(mut item) = syn::parse2::<TraitItem>(tokens.clone())
            {
                return match apply_to_trait_item(&mut item, false)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            if let Ok(mut item) = syn::parse2::<ForeignItem>(tokens)
            {
                return match apply_to_foreign_item(&mut item, false)
                {
                    Ok(()) => item.into_token_stream().into(),
                    Err(err) => err.to_compile_error().into(),
                };
            }
            item_err.to_compile_error().into()
        },
    }
}
