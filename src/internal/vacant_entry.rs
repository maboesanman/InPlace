use super::occupied_entry::{KeyedOccupiedEntry, OccupiedEntry};

/// A vacant entry of a collection.
/// 
/// This is a slot into which you can insert a value with no concern for there already being a value there,
/// because if you have a vacant entry, you have already done the work to confirm the slot is open and ready for a value.
///
/// The idea is that you've done hard work of finding your place in the collection,
/// so inserting shouldn't be a huge penalty at this point.
pub trait VacantEntry<'c>: Sized {
    /// The type of values in the collection.
    type Value: 'c;

    /// The type of OccupiedEntry we convert to when inserting.
    type Occupied: OccupiedEntry<'c>;

    /// insert the `val` using the owned key.
    ///
    /// if you need to chain a future deletion, use `insert_entry`
    fn insert(self, val: Self::Value) {
        self.occupy(val);
    }

    /// insert the `val` using the owned key, returning the occupied entry
    fn occupy(self, val: Self::Value) -> Self::Occupied;
}

/// A vacant entry which has a key.
///
/// This is usually going to be created by some search by key, which was not present in the collection.
/// When this occurs, the get_entry method should move the owned key into a vacant entry, or clone it.
///
/// this can be thought of as owning a `K`, ready to insert when given a corresponding value.
pub trait KeyedVacantEntry<'c>: VacantEntry<'c>
where
    Self::Occupied: KeyedOccupiedEntry<'c, Key = Self::Key>,
{
    type Key: 'c;

    /// Get a reference to the key an item will be inserted with.
    fn get_key<'e>(&'e self) -> &'e Self::Key
    where
        'c: 'e;

    /// Consume self and return the contained key.
    fn into_key(self) -> Self::Key;
}
