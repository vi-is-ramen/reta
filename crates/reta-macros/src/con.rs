use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{Ident, Item, Token, TraitBound, TypeParamBound, WherePredicate};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConstKind
{
    MaybeConst,
    Const,
    None,
}

const RETA_MAYBE_CONST: &str = "reta_maybe_const";
const RETA_CONST: &str = "reta_const";
const CRATE_ALIAS: &str = "crate_";
const ABSOLUTE_ALIAS: &str = "__";

fn apply_constness_to_fn_sig(
    sig: &mut syn::Signature,
    make_const: bool,
) -> syn::Result<()>
{
    if make_const
    {
        if sig.constness.is_none()
        {
            sig.constness = Some(Token![const](Span::call_site()));
        }
    }
    else
    {
        sig.constness = None;
    }
    Ok(())
}

fn get_const_kind_from_bound(bound: &TraitBound) -> ConstKind
{
    if let Some(first_segment) = bound.path.segments.first()
    {
        let ident_str = first_segment.ident.to_string();
        if ident_str == RETA_MAYBE_CONST
        {
            return ConstKind::MaybeConst;
        }
        else if ident_str == RETA_CONST
        {
            return ConstKind::Const;
        }
    }
    ConstKind::None
}

fn remove_const_prefix(bound: &mut TraitBound)
{
    if bound.path.segments.len() < 2
    {
        return;
    }

    let mut iter = bound.path.segments.iter().cloned();
    iter.next();
    let remaining: syn::punctuated::Punctuated<syn::PathSegment, Token![::]> =
        iter.collect();

    let mut new_segments = remaining;
    let mut leading_colon = bound.path.leading_colon;

    if let Some(first_remaining) = new_segments.first()
    {
        let first_ident = first_remaining.ident.to_string();

        if first_ident == CRATE_ALIAS
        {
            let mut first_mut = first_remaining.clone();
            first_mut.ident = Ident::new("crate", first_remaining.ident.span());
            let mut rebuilt = syn::punctuated::Punctuated::new();
            let mut iter2 = new_segments.into_iter();
            iter2.next();
            rebuilt.push(first_mut);
            for seg in iter2
            {
                rebuilt.push(seg);
            }
            new_segments = rebuilt;
        }
        else if first_ident == ABSOLUTE_ALIAS
        {
            leading_colon = Some(Token![::](first_remaining.ident.span()));
            let mut iter2 = new_segments.into_iter();
            iter2.next();
            new_segments = iter2.collect();
        }
    }

    bound.path.leading_colon = leading_colon;
    bound.path.segments = new_segments;
}

fn print_trait_bound_with_const(
    bound: &TraitBound,
    const_kind: ConstKind,
) -> TokenStream2
{
    let modifier = match const_kind
    {
        ConstKind::MaybeConst => quote! { [const] },
        ConstKind::Const => quote! { const },
        ConstKind::None => quote! {},
    };

    let maybe = &bound.maybe;
    let path = &bound.path;
    let lifetimes = &bound.lifetimes;

    if let Some(lifetimes) = lifetimes
    {
        quote! { #modifier #maybe for<#lifetimes> #path }
    }
    else
    {
        quote! { #modifier #maybe #path }
    }
}

fn transform_trait_bounds_for_const(
    trait_item: &mut syn::ItemTrait,
) -> Vec<ConstKind>
{
    let mut const_kinds = Vec::new();

    for bound in trait_item.supertraits.iter_mut()
    {
        if let TypeParamBound::Trait(trait_bound) = bound
        {
            let kind = get_const_kind_from_bound(trait_bound);
            if kind != ConstKind::None
            {
                const_kinds.push(kind);
                remove_const_prefix(trait_bound);
            }
        }
    }

    if let Some(where_clause) = &mut trait_item.generics.where_clause
    {
        for predicate in where_clause.predicates.iter_mut()
        {
            if let WherePredicate::Type(pred_type) = predicate
            {
                for bound in pred_type.bounds.iter_mut()
                {
                    if let TypeParamBound::Trait(trait_bound) = bound
                    {
                        let kind = get_const_kind_from_bound(trait_bound);
                        if kind != ConstKind::None
                        {
                            const_kinds.push(kind);
                            remove_const_prefix(trait_bound);
                        }
                    }
                }
            }
        }
    }

    const_kinds
}

fn print_trait_with_const_modifiers(
    trait_item: &syn::ItemTrait,
    const_kinds: &[ConstKind],
    make_const: bool,
) -> TokenStream2
{
    let attrs = &trait_item.attrs;
    let vis = &trait_item.vis;
    let unsafety = &trait_item.unsafety;
    let modifiers = &trait_item.modifiers;
    let ident = &trait_item.ident;
    let items = &trait_item.items;

    let const_token = if make_const
    {
        quote! { const }
    }
    else
    {
        quote! {}
    };

    let auto_token = &modifiers.auto_token;
    let modifiers_tokens = quote! { #auto_token };

    let mut const_index = 0;

    let print_bound =
        |bound: &TypeParamBound, idx: &mut usize| -> TokenStream2 {
            if let TypeParamBound::Trait(trait_bound) = bound
            {
                let const_kind = if *idx < const_kinds.len()
                {
                    let kind = const_kinds[*idx];
                    *idx += 1;
                    kind
                }
                else
                {
                    ConstKind::None
                };
                print_trait_bound_with_const(trait_bound, const_kind)
            }
            else
            {
                bound.to_token_stream()
            }
        };

    let supertraits: Vec<TokenStream2> = trait_item
        .supertraits
        .iter()
        .map(|b| print_bound(b, &mut const_index))
        .collect();

    let colon_token = trait_item
        .colon_token
        .as_ref()
        .map(|_| quote! { : })
        .unwrap_or_default();
    let supertraits_tokens = if supertraits.is_empty()
    {
        quote! {}
    }
    else
    {
        quote! { #colon_token #(#supertraits),* }
    };

    let mut generics_no_where = trait_item.generics.clone();
    generics_no_where.where_clause = None;
    let lt_token = &generics_no_where.lt_token;
    let gt_token = &generics_no_where.gt_token;
    let params = &generics_no_where.params;

    let generics_tokens = if params.is_empty()
    {
        quote! {}
    }
    else
    {
        quote! { #lt_token #params #gt_token }
    };

    let where_clause_tokens = if let Some(wc) =
        &trait_item.generics.where_clause
    {
        let predicates: Vec<TokenStream2> = wc
            .predicates
            .iter()
            .map(|pred| {
                if let WherePredicate::Type(pred_type) = pred
                {
                    let bounds: Vec<TokenStream2> = pred_type
                        .bounds
                        .iter()
                        .map(|b| print_bound(b, &mut const_index))
                        .collect();
                    let lifetimes = &pred_type.lifetimes;
                    let bounded_ty = &pred_type.bounded_ty;
                    let colon = &pred_type.colon_token;
                    if let Some(lifetimes) = lifetimes
                    {
                        quote! {
                            for<#lifetimes> #bounded_ty #colon #(#bounds),*
                        }
                    }
                    else
                    {
                        quote! { #bounded_ty #colon #(#bounds),* }
                    }
                }
                else
                {
                    pred.to_token_stream()
                }
            })
            .collect();
        let where_token = &wc.where_token;
        quote! { #where_token #(#predicates),* }
    }
    else
    {
        quote! {}
    };

    quote! {
        #(#attrs)*
        #vis #unsafety #modifiers_tokens #const_token
        trait #ident #generics_tokens #supertraits_tokens #where_clause_tokens
        {
            #(#items)*
        }
    }
}

fn transform_trait_for_const(item: &mut syn::ItemTrait) -> TokenStream2
{
    let const_kinds = transform_trait_bounds_for_const(item);
    print_trait_with_const_modifiers(item, &const_kinds, true)
}

fn transform_trait_for_dyn(item: &mut syn::ItemTrait) -> TokenStream2
{
    let const_kinds = transform_trait_bounds_for_const(item);
    let none_kinds = vec![ConstKind::None; const_kinds.len()];
    print_trait_with_const_modifiers(item, &none_kinds, false)
}

fn print_impl_with_const(
    item_impl: &syn::ItemImpl,
    make_const: bool,
) -> TokenStream2
{
    let attrs = &item_impl.attrs;
    let unsafety = &item_impl.unsafety;
    let impl_token = &item_impl.impl_token;
    let generics = &item_impl.generics;
    let trait_ = &item_impl.trait_;
    let self_ty = &item_impl.self_ty;
    let items = &item_impl.items;

    let const_token = if make_const
    {
        quote! { const }
    }
    else
    {
        quote! {}
    };

    let mut generics_no_where = generics.clone();
    generics_no_where.where_clause = None;
    let lt_token = &generics_no_where.lt_token;
    let gt_token = &generics_no_where.gt_token;
    let params = &generics_no_where.params;

    let generics_tokens = if params.is_empty()
    {
        quote! {}
    }
    else
    {
        quote! { #lt_token #params #gt_token }
    };

    let trait_tokens = if let Some((path, for_token)) = trait_
    {
        quote! { #path #for_token }
    }
    else
    {
        quote! {}
    };

    let where_clause_tokens = if let Some(wc) = &generics.where_clause
    {
        let predicates = &wc.predicates;
        let where_token = &wc.where_token;
        quote! { #where_token #predicates }
    }
    else
    {
        quote! {}
    };

    quote! {
        #(#attrs)*
        #const_token #unsafety #impl_token #generics_tokens #trait_tokens
        #self_ty #where_clause_tokens {
            #(#items)*
        }
    }
}

fn apply_to_item(item: &mut Item, make_const: bool) -> syn::Result<()>
{
    match item
    {
        Item::Fn(item_fn) =>
        {
            apply_constness_to_fn_sig(&mut item_fn.sig, make_const)
        },
        other =>
        {
            if !make_const
            {
                Ok(())
            }
            else
            {
                Err(syn::Error::new_spanned(
                    other.to_token_stream(),
                    "#[const_] can only be applied to functions, traits, and \
                     impls",
                ))
            }
        },
    }
}

pub fn const_(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    let tokens: TokenStream2 = item.into();

    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match &mut item
        {
            Item::Trait(trait_item) =>
            {
                transform_trait_for_const(trait_item).into()
            },
            Item::Impl(item_impl) =>
            {
                print_impl_with_const(item_impl, true).into()
            },
            _ => match apply_to_item(&mut item, true)
            {
                Ok(()) => item.into_token_stream().into(),
                Err(err) => err.to_compile_error().into(),
            },
        },
        Err(item_err) => item_err.to_compile_error().into(),
    }
}

pub fn dyn_(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    let tokens: TokenStream2 = item.into();

    match syn::parse2::<Item>(tokens.clone())
    {
        Ok(mut item) => match &mut item
        {
            Item::Trait(trait_item) =>
            {
                transform_trait_for_dyn(trait_item).into()
            },
            Item::Impl(item_impl) =>
            {
                print_impl_with_const(item_impl, false).into()
            },
            _ => match apply_to_item(&mut item, false)
            {
                Ok(()) => item.into_token_stream().into(),
                Err(err) => err.to_compile_error().into(),
            },
        },
        Err(item_err) => item_err.to_compile_error().into(),
    }
}
