#![feature(generic_associated_types)]

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

pub use internal::in_place::InPlace;
pub use internal::ext::InPlaceExt;

pub mod lazy {
    use super::internal;

    pub use internal::lazy_entry;
}

pub mod key_mut {
    use super::internal;

    pub use internal::occupied_entry::OccupiedEntryKeyMut;
}

pub mod ord {
    use super::internal;

    pub use internal::ord::InPlaceOrd;
    pub use internal::ord::InPlaceOrdEntry;
}

pub mod entry {
    use super::internal;

    pub use internal::occupied_entry::OccupiedEntry;
    pub use internal::vacant_entry::VacantEntry;
    pub use internal::entry::*;

    pub use internal::ext::OccupiedEntryExt;
    pub use internal::ext::VacantEntryExt;
}

pub mod option {
    use super::internal;

    pub use internal::option::*;
}
