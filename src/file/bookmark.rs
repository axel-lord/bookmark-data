//! how bookmarks are stored.

use std::mem;

use deepsize::DeepSizeOf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Layout of a bookmark.
#[must_use]
#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    /// The url of the bookmark.
    pub url: String,
    /// Description/info for the bookmark, often a display name.
    pub info: String,
    /// An identifier used for the bookmark.
    pub uuid: Uuid,
    /// Any tags which may be used to find the bookmark.
    pub tag: Vec<String>,
}

impl Bookmark {
    /// Create a new instance from a url and some info.
    pub fn new(url: impl Into<String>, info: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            info: info.into(),
            ..Default::default()
        }
    }
}

impl Default for Bookmark {
    fn default() -> Self {
        Self {
            url: String::new(),
            info: String::new(),
            uuid: Uuid::new_v4(),
            tag: Vec::new(),
        }
    }
}

impl DeepSizeOf for Bookmark {
    fn deep_size_of_children(&self, _context: &mut deepsize::Context) -> usize {
        0
    }

    fn deep_size_of(&self) -> usize {
        mem::size_of::<Self>()
            + self.url.capacity()
            + self.info.capacity()
            + self.tag.iter().map(String::capacity).sum::<usize>()
            + self.tag.capacity() * mem::size_of::<String>()
    }
}
