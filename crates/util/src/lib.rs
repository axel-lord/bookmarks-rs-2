//! Some general utilities.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

/// Trait extending all types with utility methods.
pub trait AnyWithExt {
    /// Change state of type with a function and a value if said value is Some else do nothing.
    #[must_use]
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

    /// Change the state of a type with a function and a value if said value is Some else use
    /// another function. As long as the two functions have the same return type they may change
    /// self to said type.
    #[must_use]
    fn with_else<T, R>(
        self,
        value: Option<T>,
        mut if_some: impl FnMut(Self, T) -> R,
        mut if_none: impl FnMut(Self) -> R,
    ) -> R
    where
        Self: Sized,
    {
        if let Some(value) = value {
            if_some(self, value)
        } else {
            if_none(self)
        }
    }
}

impl<T> AnyWithExt for T {}
