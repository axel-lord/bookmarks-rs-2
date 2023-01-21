//! Some utilities to make working with ui easier.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use iced::{
    widget::{Column, Row},
    Element,
};

/// Extension trait to create rows or columns from an iterator.
pub trait IteratorWidgetExt<Message>: Iterator {
    /// Collect an iterator into a column using the passed function to transform the iterator
    /// content to elements.
    fn collect_column<'a, E, F>(self, f: F) -> Column<'a, Message>
    where
        E: Into<Element<'a, Message>>,
        F: FnMut(Self::Item) -> E;

    /// Collect an iterator into a row using the passed function to transform the iterator content
    /// to elements
    fn collect_row<'a, E, F>(self, f: F) -> Row<'a, Message>
    where
        E: Into<Element<'a, Message>>,
        F: FnMut(Self::Item) -> E;
}

impl<I, Message> IteratorWidgetExt<Message> for I
where
    I: Iterator,
{
    fn collect_row<'a, E, F>(self, mut f: F) -> Row<'a, Message>
    where
        E: Into<Element<'a, Message>>,
        F: FnMut(Self::Item) -> E,
    {
        self.fold(Row::new(), |row, item| row.push(f(item)))
    }

    fn collect_column<'a, E, F>(self, mut f: F) -> Column<'a, Message>
    where
        E: Into<Element<'a, Message>>,
        F: FnMut(Self::Item) -> E,
    {
        self.fold(Column::new(), |column, item| column.push(f(item)))
    }
}
