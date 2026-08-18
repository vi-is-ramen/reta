//! Procedural macros for the `reta` crate.
//!
//! These macros are re‑exported by the `reta` crate. They provide
//! conditional attributes and modifiers for items.

mod asy;
mod cfg;
mod con;
mod rif;
mod saf;
mod vis;

use proc_macro::TokenStream;

/// Makes an item `async` unconditionally.
///
/// # Errors
///
/// This macro will produce a compile‑time error if applied to an item that is
/// not a function (or an `impl`/`trait` item that is a function).
///
/// # Examples
///
/// ```
/// # use reta_macros::async_;
///
/// #[async_]
/// fn make_me_async() {}
/// ```
#[proc_macro_attribute]
pub fn async_(attr: TokenStream, item: TokenStream) -> TokenStream
{
    asy::async_(attr, item)
}

/// Makes an item `sync` (i.e., non‑`async`) unconditionally.
///
/// This removes any `async` keyword from the item, if there is one.
///
/// # Examples
///
/// ```
/// # use reta_macros::sync_;
///
/// #[sync_]
/// async fn make_me_sync() {}
/// ```
#[proc_macro_attribute]
pub fn sync_(attr: TokenStream, item: TokenStream) -> TokenStream
{
    asy::sync_(attr, item)
}

/// Makes an item `const` unconditionally.
///
/// Can be applied to functions, traits, and `impl` blocks.
/// For traits and `impl` blocks, it adds the `const` keyword and also handles
/// `[const]` and `const` modifiers in trait bounds (e.g., `T: [const] ...`).
///
/// # Example
///
/// ```ignore
/// # use reta_macros::const_;
///
/// #[const_]
/// fn this_is_const() {}
///
/// #[const_]
/// trait RequiresConstDefault
/// where Self: reta_const::Default
/// {
/// }
///
/// #[const_]
/// trait MaybeConstDefault
/// where Self: reta_maybe_const::Default
/// {
/// }
/// ```
///
/// Will become:
///
/// ```ignore
/// # use reta_macros::const_;
///
/// const fn this_is_const() {}
///
/// const trait RequiresConstDefault
/// where Self: const Default
/// {
/// }
///
/// const trait MaybeConstDefault
/// where Self: [const] Default
/// {
/// }
/// ```
///
/// # Trait supertraits and where clauses handling rules
///
/// Because `const_traits` and `const_trait_impls` are nightly features which
/// modifies syntax, you must # use reta_macros's original (maybe ugly, but
/// still working ) syntax. If you wanna require const-ness from supertrait, add
/// `reta_const::` before it. If you wanna declare that supertrait **may be**
/// constant, add `reta_maybe_const::` before it. If you need to refer to the
/// supertrait using absolute path (which begins with `::` in usual), add `__`
/// path component between `reta*_const::` and `::YourTrait`. If you need to
/// refer to the supertrait using `crate`-relative path, add `crate_` between
/// `reta*_const::` and `::YourTrait`.
#[proc_macro_attribute]
pub fn const_(attr: TokenStream, item: TokenStream) -> TokenStream
{
    con::const_(attr, item)
}

/// Makes an item non‑`const` unconditionally.
///
/// This removes any `const` keyword from the item. It is the counterpart to
/// `#[const_]`.
#[proc_macro_attribute]
pub fn dyn_(attr: TokenStream, item: TokenStream) -> TokenStream
{
    con::dyn_(attr, item)
}

/// A conditional macro that expands to `true` or `false` based on a condition.
///
/// It's absolutely identical to `cfg!()`, but handles all conditions which are
/// handled by `reta` and `reta_attr` attributes.
///
/// # Examples
///
/// ```
/// # use reta_macros::reta_if;
///
/// if reta_if!(feature = "debug")
/// {
///     // debugging code
/// }
/// else
/// {
///     // release code
/// }
/// ```
#[proc_macro]
pub fn reta_if(input: TokenStream) -> TokenStream
{
    rif::reta_if(input)
}

/// An enhanced conditional attribute that removes the item if the condition is
/// false at expansion time.
///
/// It behaves like `#[cfg]` but additionally evaluates `channel(...)`
/// conditions directly.
///
/// # Examples
///
/// ```
/// # use reta_macros::reta;
///
/// #[reta(channel(nightly))]
/// fn only_on_nightly() {}
/// ```
#[proc_macro_attribute]
pub fn reta(attr: TokenStream, item: TokenStream) -> TokenStream
{
    cfg::reta(attr, item)
}

/// An enhanced conditional attribute that applies another attribute only if the
/// condition is true.
///
/// It is similar to `#[cfg_attr]` but supports `channel(...)` evaluation.
///
/// # Examples
///
/// ```
/// # use reta_macros::reta_attr;
///
/// #[reta_attr(feature = "serde", derive(Serialize))]
/// struct MyStruct;
/// ```
#[proc_macro_attribute]
pub fn reta_attr(attr: TokenStream, item: TokenStream) -> TokenStream
{
    cfg::reta_attr(attr, item)
}

/// Marks an item as `safe` unconditionally.
///
/// This removes any `unsafe` keyword from the item and adds `safe` keyword if
/// applied to the foreign item.
#[proc_macro_attribute]
pub fn safe_(attr: TokenStream, item: TokenStream) -> TokenStream
{
    saf::safe_(attr, item)
}

/// Marks an item as `unsafe` unconditionally.
///
/// # Examples
///
/// ```
/// # use reta_macros::unsafe_;
///
/// #[unsafe_]
/// fn this_is_unsafe() {}
/// ```
#[proc_macro_attribute]
pub fn unsafe_(attr: TokenStream, item: TokenStream) -> TokenStream
{
    saf::unsafe_(attr, item)
}

/// Sets the visibility of an item to `pub`.
///
/// This is equivalent to writing `pub` before the item. It is useful in
/// combination with `#[cfg_attr]` or `#[reta_attr]`.
///
/// # Examples
///
/// ```
/// # use reta_macros::pub_;
///
/// #[cfg_attr(feature = "export", pub_)]
/// fn internal_fn() {}
/// ```
#[proc_macro_attribute]
pub fn pub_(attr: TokenStream, item: TokenStream) -> TokenStream
{
    vis::pub_(attr, item)
}

/// Sets the visibility of an item to `pub(crate)`.
#[proc_macro_attribute]
pub fn pub_crate(attr: TokenStream, item: TokenStream) -> TokenStream
{
    vis::pub_crate(attr, item)
}

/// Sets the visibility of an item to `pub(super)`.
#[proc_macro_attribute]
pub fn pub_super(attr: TokenStream, item: TokenStream) -> TokenStream
{
    vis::pub_super(attr, item)
}

/// Sets the visibility of an item to `pub(in path)`.
///
/// The path is specified as the macro attribute argument, e.g.,
/// `#[pub_in(crate::foo)]`.
#[proc_macro_attribute]
pub fn pub_in(attr: TokenStream, item: TokenStream) -> TokenStream
{
    vis::pub_in(attr, item)
}

/// Sets the visibility of an item to private (inherited).
///
/// This effectively removes any `pub` keyword, making the item private.
///
/// # Examples
///
/// ```
/// # use reta_macros::priv_;
///
/// #[cfg_attr(feature = "internal", priv_)]
/// pub fn should_be_private() {}
/// ```
#[proc_macro_attribute]
pub fn priv_(attr: TokenStream, item: TokenStream) -> TokenStream
{
    vis::priv_(attr, item)
}
