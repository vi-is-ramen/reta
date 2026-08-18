use crate::*;

/// The compiler release channel.
///
/// This enum represents the three possible Rust channels: stable, beta, and
/// nightly. It is primarily used by the [`channel()`] function and the
/// [`CHANNEL`] constant.
///
/// If `core` feature is enabled (default), this type doesn't implement
/// `Display` and `Debug`
///
/// It's `#[non_exhaustive]` as we aren't mindreaders and rustc team can
/// announce new compiler channel. It's better to care about it before it
/// become a reason for breaking change.
#[derive(Copy)]
#[reta_attr(channel(nightly), derive_const(Clone))]
#[reta_attr(not(channel(nightly)), derive(Clone))]
#[non_exhaustive]
pub enum Channel
{
    /// The stable release channel.
    Stable,
    /// The beta release channel.
    Beta,
    /// The nightly release channel.
    Nightly,
}

impl core::fmt::Debug for Channel
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str(match self
        {
            Channel::Stable => "stable",
            Channel::Beta => "beta",
            Channel::Nightly => "nightly",
        })
    }
}

impl core::fmt::Display for Channel
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str(match self
        {
            Channel::Stable => "stable",
            Channel::Beta => "beta",
            Channel::Nightly => "nightly",
        })
    }
}

/// Returns the current compiler release channel.
///
/// This function is `const` and is resolved at compile time based on the
/// detected channel. The result is one of `Channel::Stable`, `Channel::Beta`,
/// or `Channel::Nightly`.
///
/// By the way, now you are using stable toolchain.
///
/// # Examples
///
/// ```
/// use reta::{Channel, channel};
///
/// assert!(matches!(
///     channel(),
///     Channel::Nightly | Channel::Beta | Channel::Stable
/// ));
/// ```
#[reta(all(channel(stable), not(docsrs)))]
pub const fn channel() -> Channel
{
    Channel::Stable
}

/// Returns the current compiler release channel.
///
/// This function is `const` and is resolved at compile time based on the
/// detected channel. The result is one of `Channel::Stable`, `Channel::Beta`,
/// or `Channel::Nightly`.
///
/// By the way, now you are using beta toolchain.
///
/// # Examples
///
/// ```
/// use reta::{Channel, channel};
///
/// assert!(matches!(
///     channel(),
///     Channel::Nightly | Channel::Beta | Channel::Stable
/// ));
/// ```
#[reta(all(channel(beta), not(docsrs)))]
pub const fn channel() -> Channel
{
    Channel::Beta
}

/// Returns the current compiler release channel.
///
/// This function is `const` and is resolved at compile time based on the
/// detected channel. The result is one of `Channel::Stable`, `Channel::Beta`,
/// or `Channel::Nightly`.
///
/// By the way, now you are using nightly toolchain.
///
/// # Examples
///
/// ```
/// use reta::{Channel, channel};
///
/// assert!(matches!(
///     channel(),
///     Channel::Nightly | Channel::Beta | Channel::Stable
/// ));
/// ```
#[reta(all(channel(nightly), not(docsrs)))]
pub const fn channel() -> Channel
{
    Channel::Nightly
}

/// Returns the current compiler release channel.
///
/// This function is `const` and is resolved at compile time based on the
/// detected channel. The result is one of `Channel::Stable`, `Channel::Beta`,
/// or `Channel::Nightly`.
///
/// # Examples
///
/// ```
/// use reta::{Channel, channel};
///
/// assert!(matches!(
///     channel(),
///     Channel::Nightly | Channel::Beta | Channel::Stable
/// ));
/// ```
#[reta(docsrs)]
pub const fn channel() -> Channel
{
    unreachable!()
}

/// A constant holding the current compiler release channel.
///
/// # Examples
///
/// ```
/// use reta::{CHANNEL, Channel};
///
/// match CHANNEL
/// {
///     Channel::Nightly => println!("Running on nightly"),
///     Channel::Beta => println!("Running on beta"),
///     Channel::Stable => println!("Running on stable"),
///     _ => unreachable!(),
/// }
/// ```
pub const CHANNEL: Channel = channel();
