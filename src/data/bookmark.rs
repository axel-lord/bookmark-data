//! how bookmarks are stored.

use std::mem;

use deepsize::DeepSizeOf;
use derive_new::new;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Layout of a bookmark.
#[must_use]
#[derive(Debug, Serialize, Deserialize, new, Hash, PartialEq, Eq, Clone)]
pub struct Bookmark {
    /// The url of the bookmark.
    pub url: String,
    /// Description/info for the bookmark, often a display name.
    pub info: String,
    /// An identifier used for the bookmark.
    #[new(value = "Uuid::new_v4()")]
    pub uuid: Uuid,
    /// Any tags which may be used to find the bookmark.
    #[new(default)]
    pub tag: Vec<String>,
}

impl Default for Bookmark {
    fn default() -> Self {
        Self::new(String::default(), String::default())
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
