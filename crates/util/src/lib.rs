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

    /// Use a functor on a reference to an object and a value if the value is some else do
    /// nothing.
    #[must_use]
    fn with_ref<Value, IfSome, IfSomeRet>(self, value: Option<Value>, mut if_some: IfSome) -> Self
    where
        Self: Sized,
        IfSome: FnMut(&Self, Value) -> IfSomeRet,
    {
        if let Some(value) = value {
            if_some(&self, value);
        }
        self
    }

    /// Use a functor on a mutable reference to an object and a value if the value is some else do
    /// nothing.
    #[must_use]
    fn with_mut<Value, IfSome, IfSomeRet>(
        mut self,
        value: Option<Value>,
        mut if_some: IfSome,
    ) -> Self
    where
        Self: Sized,
        IfSome: FnMut(&mut Self, Value) -> IfSomeRet,
    {
        if let Some(value) = value {
            if_some(&mut self, value);
        }
        self
    }

    /// Use a functor on a reference to an object and a value if the value is some else do
    /// nothing.
    #[must_use]
    fn with_as_ref<Value, IfSome, SelfRef, IfSomeRet>(
        self,
        value: Option<Value>,
        mut if_some: IfSome,
    ) -> Self
    where
        Self: Sized + AsRef<SelfRef>,
        IfSome: FnMut(&SelfRef, Value) -> IfSomeRet,
    {
        if let Some(value) = value {
            if_some(self.as_ref(), value);
        }
        self
    }
}

impl<Ty> AnyWithExt for Ty {}
