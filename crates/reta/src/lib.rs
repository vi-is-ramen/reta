//! **R**ust m**eta**: Crate for extended meta‑manipulation over your items.
//!
//! This crate provides a collection of attribute macros that give you
//! fine‑grained control over visibility, async‑ness, const‑ness, safety,
//! and conditional compilation at compile time. It is designed to work
//! seamlessly with `#[cfg_attr]` and similar tools, allowing you to write
//! cleaner and more maintainable conditional code.
//!
//! # Features
//!
//! - **Visibility as attributes**: `#[pub_]`, `#[pub_crate]`, `#[pub_super]`,
//!   `#[pub_in(path)]`, `#[priv_]` – use them with `#[cfg_attr]` to toggle
//!   visibility based on features or other conditions.
//! - **Async‑ness as attributes**: `#[async_]` and `#[sync_]` – make a function
//!   `async` or `sync` conditionally.
//! - **Const‑ness as attributes**: `#[const_]` and `#[dyn_]` – make a function,
//!   trait, or `impl` block `const` or non‑`const` conditionally.
//! - **Safety as attributes**: `#[safe_]` and `#[unsafe_]` – mark a function
//!   `safe` or `unsafe` conditionally.
//! - **Extended compile‑time branching**: `#[reta]` and `#[reta_attr]` act as
//!   enhanced alternatives to `#[cfg]` and `#[cfg_attr]`, with support for
//!   evaluating the compiler channel (`nightly`/`beta`/`stable`) at macro
//!   expansion time.
//! - **Conditional boolean expression**: `reta_if!` returns `true` or `false`
//!   based on the same conditions, useful for runtime decisions that depend on
//!   compile‑time knowledge.
//!
//! # Feature Flags
//!
//! - **`std`** *(enabled by default)*: If disabled, the crate does not link
//!   against the standard library.
//! - **`core`** *(enabled by default)*: If disabled, the crate does not link
//!   against `libcore`. Some functionality, such as the `Debug` and `Display`
//!   implementations of `Mut`, `Const`, and `Channel`, will be disabled.
//!
//! # Examples
//!
//! ## Make a function public only when feature `x` is enabled
//!
//! ```rust
//! use reta::*;
//!
//! #[reta_attr(feature = "x", pub_)]
//! fn hidden_otherwise()
//! { /* ... */
//! }
//! ```
//!
//! ## Make a function async only when feature `asio` is enabled
//!
//! ```rust
//! use reta::*;
//!
//! #[reta_attr(feature = "asio", async_)]
//! fn maybe_async()
//! { /* ... */
//! }
//! ```
//!
//! ## Make a function const only on nightly
//!
//! ```rust
//! use reta::*;
//!
//! #[reta_attr(channel(nightly), const_)]
//! fn const_on_nightly()
//! { /* ... */
//! }
//! ```
//!
//! ## Remove an item on stable
//!
//! ```rust
//! use reta::*;
//!
//! #[reta(channel(not(stable)))]
//! fn only_not_stable()
//! { /* ... */
//! }
//! ```
//!
//! ## Use `reta_if!` to conditionally include code
//!
//! ```rust
//! use reta::reta_if;
//!
//! if reta_if!(channel(nightly))
//! {
//!     // nightly‑only code
//! }
//! else
//! {
//!     // fallback
//! }
//! ```
//!
//! # Channel Detection
//!
//! The build script (`build.rs`) detects the current Rust compiler channel
//! (`nightly`, `beta`, or `stable`) and sets the corresponding `cfg` flag.
//! These flags are used by the `channel(...)` condition inside `reta` and
//! `reta_attr`.
//!
//! # No‑std Support
//!
//! The crate is compatible with `no_std` and `no_core` environments.
//! By disabling the `std` and `core` features, you can use the macros
//! without any dependency on the standard library. If you disable `core`
//! feature, whole crate's content except macros would be disabled. You can
//! achieve the same behavior by enabling `macros` feature, but it would not
//! activate `#![no_core]`, so this feature exists mostly for stable compiler
//! users.

#![cfg_attr(all(nightly, not(feature = "core")), feature(no_core))]
#![cfg_attr(all(nightly, not(feature = "core")), no_core)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    all(nightly, feature = "core", not(feature = "macros")),
    feature(derive_const, const_clone)
)]

pub use reta_macros::*;

#[reta(all(feature = "core", not(feature = "macros")))]
mod chan;
#[reta(all(feature = "core", not(feature = "macros")))]
mod con_;

#[reta(all(feature = "core", not(feature = "macros")))]
pub use chan::*;
#[reta(all(feature = "core", not(feature = "macros")))]
pub use con_::*;

/// Reta's "prelude" with the most essential re-exports.
pub mod pre
{
    #[reta(all(feature = "core", not(feature = "macros")))]
    pub use super::{Const, Constant, Constness, Mut, Mutable};
    pub use reta_macros::*;
}
