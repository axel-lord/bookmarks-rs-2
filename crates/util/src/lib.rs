//! Some general utilities.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use std::ops::Deref;

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

/// An object that exists "somewhere" be it in a box, or staticaly somwhere else. Primary use is
/// for trait objects.
#[derive(Clone, Debug)]
pub enum Somewhere<T>
where
    T: 'static + ?Sized,
{
    /// A reference to the value if it exists somewhere else.
    Borrowed(&'static T),
    /// A Box to the value if it is owned by self.
    Owned(Box<T>),
}

impl<T> From<&'static T> for Somewhere<T>
where
    T: 'static + ?Sized,
{
    fn from(value: &'static T) -> Self {
        Self::Borrowed(value)
    }
}

impl<T> Deref for Somewhere<T>
where
    T: 'static + ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Somewhere::Borrowed(val) => val,
            Somewhere::Owned(val) => val,
        }
    }
}
