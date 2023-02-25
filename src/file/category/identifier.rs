//! Identifiers for deciding what bookmarks belong to which category.

use deepsize::DeepSizeOf;
use serde::{Deserialize, Serialize};

/// Sorting rules for a category.
#[must_use]
#[derive(Default, Debug, Serialize, Deserialize, DeepSizeOf)]
pub struct Identifier {
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

impl Identifier {
    /// Create a new empty instance.
    pub fn new() -> Self {
        Self::default()
    }
}
