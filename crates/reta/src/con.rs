use crate::*;

mod sealed
{
    pub(super) trait ConstnessSeal {}
}

/// A marker trait for types that represent const‑ness.
///
/// This trait is sealed and cannot be implemented outside this crate.
/// It is used to distinguish between mutable and constant contexts.
#[allow(private_bounds)] // I know!
pub trait Constness: sealed::ConstnessSeal {}

/// A marker type for mutable contexts.
///
/// Implements `Constness` and `Mutable`.
pub trait Mutable: Constness {}

/// A marker type for constant contexts.
///
/// Implements `Constness` and `Constant`.
pub trait Constant: Constness {}

/// Type representing a mutable context.
///
/// Used as a token to indicate that an operation is performed in a mutable
/// environment.
///
/// If `core` feature is enabled (default), this type doesn't implement
/// `Display` and `Debug`
///
/// # Examples
///
/// ```
/// use reta::{Constness, Mut, Mutable};
///
/// fn requires_mut<T: Mutable>() {}
/// requires_mut::<Mut>();
/// ```
#[reta_attr(not(feature = "core"), allow(missing_debug_implementations))]
#[reta_attr(feature = "core", derive(Copy))]
#[reta_attr(all(feature = "core", channel(nightly)), derive_const(Clone))]
#[reta_attr(all(feature = "core", not(channel(nightly))), derive(Clone))]
pub struct Mut;

/// Type representing a constant context.
///
/// Used as a token to indicate that an operation is performed in a constant
/// environment.
///
/// If `core` feature is enabled (default), this type doesn't implement
/// `Display` and `Debug`
///
/// # Examples
///
/// ```
/// use reta::{Const, Constant, Constness};
///
/// fn requires_const<T: Constant>() {}
/// requires_const::<Const>();
/// ```
#[reta_attr(not(feature = "core"), allow(missing_debug_implementations))]
#[reta_attr(feature = "core", derive(Copy))]
#[reta_attr(all(feature = "core", channel(nightly)), derive_const(Clone))]
#[reta_attr(all(feature = "core", not(channel(nightly))), derive(Clone))]
pub struct Const;

#[reta(feature = "core")]
impl ::core::fmt::Debug for Mut
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str("Mut")
    }
}

#[reta(feature = "core")]
impl ::core::fmt::Debug for Const
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str("Const")
    }
}

#[reta(feature = "core")]
impl ::core::fmt::Display for Mut
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str("Mut")
    }
}

#[reta(feature = "core")]
impl ::core::fmt::Display for Const
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str("Const")
    }
}

impl sealed::ConstnessSeal for Mut {}
impl sealed::ConstnessSeal for Const {}

impl Constness for Mut {}
impl Constness for Const {}

impl Mutable for Mut {}
impl Constant for Const {}
