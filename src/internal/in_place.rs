use super::{entry::Entry, occupied_entry::OccupiedEntry, vacant_entry::VacantEntry};

/// A trait for in-place modification of collections.
///
/// InPlace<K, V> indicates that the type is some kind of map from keys `K` to values `V`.
///
/// when calling `my_in_place_collection.get_entry()` you are given an entry
/// that is either vacant or occupied. occupied entries can be used to modify the value in place,
/// or remove it entirely. removal consumes the OccupiedEntry, and returns a VacantEntry and a value.
///
/// you can also insert a value into a VacantEntry, which returns an OccupiedEntry.
///
/// you are able to flip flop as much as you like without having to re-query the collection, or
/// unwrap values you know are present.
pub trait InPlace<K, V> {
    /// An occupied entry. This is a handle to read and modify an item which is
    /// currently in your collection. The key should be thought of as immutably borrowed,
    /// and the value should be thought of as mutably borrowed.
    ///
    /// this can be turned into a VacantEntry via `remove`
    type Occupied<'a>: OccupiedEntry<'a, K, V, Self>
    where
        Self: 'a;

    /// A vacant entry. This is a handle to insert an item into your collection,
    /// and does not allow the caller to borrow any keys or items from the collection,
    /// though it likely has references or pointers into it under the hood.
    /// The key should be thought of as owned, and there is no value
    ///
    /// this can be turned into an OccupiedEntry via `insert`
    type Vacant<'a>: VacantEntry<'a, K, V, Self>
    where
        Self: 'a;

    /// get the entry for k, whether vacant or occupied.
    ///
    /// the entry can be thought of as "preserving the work you did to search",
    /// so you can insert and remove to your heart's content without re-traversing the
    /// internal data structure, or unwrapping values you know you inserted.
    fn get_entry<'a, Q>(&'a mut self, k: Q) -> Entry<'a, K, V, Self>
    where
        Q: ToOwned<Owned = K>;
}
