//! Identifiers for deciding what bookmarks belong to which category.

use deepsize::DeepSizeOf;
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Sorting rules for a category.
#[must_use]
#[derive(Default, Debug, Serialize, Deserialize, DeepSizeOf, Hash, PartialEq, Eq, Clone, new)]
pub struct List {
    /// For a bookmark to belong to a catgory these substrings are required to be in the url of the
    /// bookmark.
    #[new(default)]
    pub require: Vec<String>,
    /// If the url of a bookmark exactly matches one of these strings it will be included in the
    /// category.
    #[new(default)]
    pub whole: Vec<String>,
    /// If the url of a bookmark contains one of these substrings it will be included in the
    /// category.
    #[new(default)]
    pub include: Vec<String>,
}
