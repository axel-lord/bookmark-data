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

pub use data::{
    bookmark::Bookmark,
    category::{identifier::List as IdentifierList, Category},
    Data,
};

pub mod data;
