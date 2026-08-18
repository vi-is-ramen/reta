# Reta
[![Crates.io](https://img.shields.io/crates/v/reta.svg)](https://crates.io/crates/reta)
[![Documentation](https://docs.rs/reta/badge.svg)](https://docs.rs/reta)
[![License](https://img.shields.io/crates/l/reta.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-stable%20|%20beta%20|%20nightly-orange.svg)](https://www.rust-lang.org)
![no_std|no_core](https://img.shields.io/badge/compatible-no__std%20|%20no__core-orange.svg)

> **HELP WANTED**
>
> Every day, I conduct research and develop prototypes for libraries, utilities,
> and other developer tools — investing a great deal of time without any
> financial return. All my projects are driven purely by enthusiasm and
> willpower. I need help developing the ecosystem — specifically the Reta crate
> — and would be very grateful for issues, patches, spreading the word, or any
> other form of contribution.

<!--
May be.. May not be... I'll think about it.
> **GUIDEBOOK**
>
> For a comprehensive, interactive guide on the library's philosophy, design
> decisions, and advanced usage patterns, please visit the
> **[Reta Book](https://vi-is-ramen.github.io/reta/)**.
-->

**R**ust m**eta**: Crate for extended meta-manipulation over your items.

This crate provides a bunch of useful attributes which gives you much
more control over your items at compilation time.

## Features

- **Visibility as attributes**: `#[pub_]`, `#[pub_super]`, `#[priv_]` and so on
for combination with `#[cfg_attr]`, `#[reta_attr]` or analogs;
- **Async-ness as attributes**: `#[async_]` & `#[sync_]` attributes for
combination with `#[cfg_attr]`, `#[reta_attr]` or analogs;
- **Const-ness as attributes**: `#[const_]` & `#[dyn_]` attributes for
combination with `#[cfg_attr]`, `#[reta_attr]` or analogs;
- **Extended compile-time branching**: `#[reta]` & `#[reta_attr]` attributes as
alternatives to `#[cfg]` & `#[cfg_attr]` with extended functionality.
- **`no_std` & `no_core` environments support**: you can use Reta literally
everywhere!

## Usage

### Case A

Make item visible only when feature `x` is enabled:

```rust
use reta::*;

#[reta_attr(feature = "x", pub_)]
fn sth() { /* ... */ }
```

### Case B

Make item async only when feature `asio` is enabled:

```rust
use reta::*;

#[reta_attr(feature = "asio", async_)]
fn sth() { /* ... */ }
```

### Case C

Make item const only when feature `c` is enabled:

```rust
use reta::*;

#[reta_attr(feature = "c", const_)]
fn sth() { /* ... */ }
```

### Case D

Make item const only on nightly compiler channel:

```rust
use reta::*;

#[reta_attr(channel(nightly), const_)]
fn sth() { /* ... */ }
```

### Case E

Throw out the item on stable compiler channel:

```rust
use reta::*;

#[reta(channel(not(stable)))]
fn sth() { /* ... */ }
```

### Case F

Make item const only on stable and beta channels:

```rust
use reta::*;

#[reta_attr(channel(not(nightly)), const_)]
fn sth() { /* ... */ }
```

## Feature Flags

- **`std`** *(enabled by default)*: If disabled, crate doesn't link with libstd.
- **`core`** *(enabled by default)*: If disabled, crate doesn't link with
libcore. Some functionality, such as built-in `Debug` and `Display`
implementations of Reta's types, such as `Mut`, `Const`, or `Channel` also would
be disabled (It's obvious while both `Debug` and `Display` are provided by
libcore).
- **`no_core`** *(disabled by default)*: If you disable the `core` feature,
don't forget to enable `no_core` feature, otherwise Reta would implement
`pointee_sized`, `meta_sized`, and `sized` lang items by self! If you are ok
with it, they are available at `reta::core` and have names `PointeeSized`,
`MetaSized`, and `Sized`. If you enable this feature, don't forget to implement
mentioned lang items by self or enable `macros` feature.
- **`macros`** *(disabled by default)*: If you want Reta to act only as a macros
provider without it's types, functions and so on, you can enable this feature.
This way, you can enable both `core` and `no_core` safely without requirement to
implement `pointee_sized`, `meta_sized`, and `sized` lang items (which doesn't
look way useful; but if you need it, we support it).

## License

Licensed under either of

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)

at your option.
