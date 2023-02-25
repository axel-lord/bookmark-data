//! How file data is stored.

use super::Result;
use deepsize::DeepSizeOf;
use serde::{Deserialize, Serialize};
use std::io::Read;

/// Layout of file data.
#[must_use]
#[derive(Default, Debug, Serialize, Deserialize, DeepSizeOf)]
pub struct File {
    /// Cache of all tags in use.
    pub tag: Vec<String>,
    /// Categories stored.
    pub category: Vec<category::Category>,
    /// Bookmarks stored.
    pub bookmark: Vec<bookmark::Bookmark>,
}

impl File {
    /// Load a bookmark file from a path.
    ///
    /// # Errors
    /// If the file does not exist or if it is wrongly formatted.
    pub fn load(reader: impl Read) -> Result<Self> {
        Ok(rmp_serde::from_read(reader)?)
    }

    /// Get the size of loaded data in bytes.
    #[must_use]
    pub fn storage_size(&self) -> usize {
        self.deep_size_of()
    }

    /// Get a new instance with no data.
    pub fn new() -> Self {
        Self::default()
    }
}

pub mod bookmark;
pub mod category;
