//! Data layout and loading.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf, result};
use tap::Pipe;
use thiserror::Error;
use tokio::fs;
use uuid::Uuid;

/// Error type for bookmark data.
#[derive(Error, Debug)]
pub enum Error {
    /// Forward for IO errors.
    #[error(transparent)]
    IO(#[from] io::Error),
    /// Forward for message pack deserialization errors.
    #[error(transparent)]
    RmpDeserialize(#[from] rmp_serde::decode::Error),
}

/// Result type for bookmark data.
pub type Result<T = ()> = result::Result<T, Error>;

/// Layout of file data.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FileData {
    /// Cache of all tags in use.
    pub tag: Vec<String>,
    /// Categories stored.
    pub category: Vec<CategoryData>,
    /// Bookmarks stored.
    pub bookmark: Vec<BookmarkData>,
}

/// Layout of a category.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CategoryData {
    /// Name of the category.
    pub name: String,
    /// Description/info for the category.
    pub info: String,
    /// Identifiers used to define what is in category.
    pub identifier: IdentifierData,
    /// Any subcategories of category.
    pub subcategory: Vec<CategoryData>,
}

/// Sorting rules for a category.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct IdentifierData {
    /// For a bookmark to belong to a catgory these substrings are required to be in the url of the
    /// bookmark.
    pub require: Vec<String>,
    /// If the url of a bookmark exactly matches one of these strings it will be included in the
    /// category.
    pub whole: Vec<String>,
    /// If the url of a bookmark contains one of these substrings it will be included in the
    /// category.
    pub include: Vec<String>,
}

/// Layout of a bookmark.
#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkData {
    /// The url of the bookmark.
    pub url: String,
    /// Description/info for the bookmark, often a display name.
    pub info: String,
    /// An identifier used for the bookmark.
    pub uuid: Uuid,
    /// Any tags which may be used to find the bookmark.
    pub tag: Vec<String>,
}

impl FileData {
    /// Load a bookmark file from a path.
    ///
    /// # Errors
    /// If the file does not exist or if it is wrongly formatted.
    pub async fn load(path: PathBuf) -> Result<Self> {
        Ok(fs::read(path).await?.pipe_deref(rmp_serde::from_slice)?)
    }
}

impl Default for BookmarkData {
    fn default() -> Self {
        Self {
            url: String::new(),
            info: String::new(),
            uuid: Uuid::new_v4(),
            tag: Vec::new(),
        }
    }
}
