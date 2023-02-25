//! How file data is stored.

use super::Result;
use deepsize::DeepSizeOf;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::io::Read;

/// Layout of file data.
#[must_use]
#[derive(Default, Debug, Serialize, Deserialize, DeepSizeOf, Hash, PartialEq, Eq, Clone, new)]
pub struct File {
    /// Cache of all tags in use.
    #[new(default)]
    pub tag: Vec<String>,
    /// Categories stored.
    #[new(default)]
    pub category: Vec<category::Category>,
    /// Bookmarks stored.
    #[new(default)]
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
}

pub mod bookmark;
pub mod category;
