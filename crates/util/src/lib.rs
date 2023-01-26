//! Some general utilities.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

/// Trait extending all types with utility methods.
pub trait AnyUtilExt {
    /// Change state of type with a function and a value if said value is Some else do noting.
    #[must_use]
    fn with<T>(self, value: Option<T>, if_some: impl FnMut(Self, T) -> Self) -> Self
    where
        Self: Sized;
}

impl<Ty> AnyUtilExt for Ty {
    fn with<T>(self, value: Option<T>, mut if_some: impl FnMut(Self, T) -> Self) -> Self
    where
        Self: Sized,
    {
        if let Some(value) = value {
            if_some(self, value)
        } else {
            self
        }
    }
}
