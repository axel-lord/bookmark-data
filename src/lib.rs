//! Data layout and loading.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::all
)]

use std::{io, result};
use thiserror::Error;

/// Error type for bookmark data.
#[must_use]
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

pub use file::{
    bookmark::Bookmark,
    category::{identifier::Identifier, Category},
    File,
};

pub mod file {
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

    pub mod category {
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

        pub mod identifier {
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
        }
    }

    pub mod bookmark {
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
    }
}
