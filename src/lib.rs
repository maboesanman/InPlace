#![feature(generic_associated_types)]
// #![cfg_attr(not(feature = "std"), no_std)]

//!
//! A trait formulation of the Entry API, to make working with collections more flexible.
//!
//! Particularly if you may need to do some number of unknown operations on multiple collections,
//! it can be helpful to keep the current entries you're working with so you don't need to
//! keep requesting them from the collection.
//!
//! overuse is potentially confusing, but flexibility is powerful, and these functions can be
//! used to create other useful operations on data structures.
//!

mod internal;

/// This module allows you to manipulate entries you have acquired
pub mod entry {
    use super::internal;

    pub use internal::entry::*;
    pub use internal::occupied_entry::*;
    pub use internal::vacant_entry::*;
}

/// This module allows you to aquire entries from collections.
///
/// This requires generic associated types.
pub mod get_entry {
    use super::internal;

    pub use internal::get_entry::*;
}

/// This module contains stubbed out examples of what this API looks like for standard collections.
pub mod dummy_collections {
    use super::internal;

    pub use internal::dummy_collections::*;
}

/// This module contains implementations of traits on collections, and struct definitions for the entries pertaining to those collections.
pub mod implementations {
    use super::internal;

    pub use internal::implementations::*;
}
