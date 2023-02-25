//! How categories are stored.

use deepsize::DeepSizeOf;
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Layout of a category.
#[must_use]
#[derive(Default, Debug, Serialize, Deserialize, DeepSizeOf, Hash, PartialEq, Eq, Clone, new)]
pub struct Category {
    /// Name of the category.
    pub name: String,
    /// Description/info for the category.
    pub info: String,
    /// Identifiers used to define what is in category.
    #[new(default)]
    pub identifier: identifier::List,
    /// Any subcategories of category.
    #[new(default)]
    pub subcategory: Vec<Category>,
}

pub mod identifier;
