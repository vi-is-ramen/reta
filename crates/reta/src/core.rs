/// Types that may or may not have a size.
#[unstable(feature = "sized_hierarchy", issue = "144404")]
#[lang = "pointee_sized"]
#[diagnostic::on_unimplemented(
    message = "values of type `{Self}` may or may not have a size",
    label = "may or may not have a known size"
)]
#[fundamental]
#[rustc_specialization_trait]
#[rustc_deny_explicit_impl]
#[rustc_coinductive]
pub trait PointeeSized
{
    // Empty
}

#[unstable(feature = "sized_hierarchy", issue = "144404")]
#[lang = "meta_sized"]
#[diagnostic::on_unimplemented(
    message = "the size for values of type `{Self}` cannot be known",
    label = "doesn't have a known size"
)]
#[fundamental]
#[rustc_specialization_trait]
#[rustc_deny_explicit_impl]
// `MetaSized` being coinductive, despite having supertraits, is okay for the
// same reasons as `Sized` above.
#[rustc_coinductive]
pub trait MetaSized: PointeeSized
{
    // Empty
}

#[lang = "sized"]
#[diagnostic::on_unimplemented(
    message = "the size for values of type `{Self}` cannot be known at \
               compilation time",
    label = "doesn't have a size known at compile-time"
)]
#[fundamental]
#[rustc_specialization_trait]
#[rustc_deny_explicit_impl]
#[rustc_dyn_incompatible_trait]
// `Sized` being coinductive, despite having supertraits, is okay as there are
// no user-written impls, and we know that the supertraits are always
// implemented if the subtrait is just by looking at the builtin impls.
#[rustc_coinductive]
pub trait Sized: MetaSized {}
