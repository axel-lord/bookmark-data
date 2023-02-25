//! How categories are stored.

use deepsize::DeepSizeOf;
use serde::{Deserialize, Serialize};

/// Layout of a category.
#[must_use]
#[derive(Default, Debug, Serialize, Deserialize, DeepSizeOf)]
pub struct Category {
    /// Name of the category.
    pub name: String,
    /// Description/info for the category.
    pub info: String,
    /// Identifiers used to define what is in category.
    pub identifier: identifier::Identifier,
    /// Any subcategories of category.
    pub subcategory: Vec<Category>,
}

impl Category {
    /// Create a new instance from a name and some info.
    pub fn new(name: impl Into<String>, info: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            info: info.into(),
            ..Default::default()
        }
    }
}

pub mod identifier;
