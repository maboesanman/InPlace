use super::occupied_entry::{IndexedOccupiedEntry, KeyedOccupiedEntry, OccupiedEntry};

/// A trait to represent a vacant entry of a collection.
///
/// this can be thought of as holding a `K`, ready to insert when given a value.
///
/// The idea is that you've done hard work of finding your place in the collection,
/// so inserting shouldn't be a huge penalty at this point.
pub trait VacantEntry<'a>: Sized {
    /// The type of values in the collection.
    type Value;

    /// The type of OccupiedEntry we convert to when inserting.
    type Occupied: OccupiedEntry<'a, Value = Self::Value>;

    /// insert the `val` using the owned key.
    ///
    /// if you need to chain a future deletion, use `insert_entry`
    fn insert(self, val: Self::Value) {
        self.occupy(val);
    }

    /// insert the `val` using the owned key, returning the occupied entry
    fn occupy(self, val: Self::Value) -> Self::Occupied;
}

pub trait KeyedVacantEntry<'a>: VacantEntry<'a>
where
    Self::Occupied: KeyedOccupiedEntry<'a, Key = Self::Key>,
{
    type Key;

    /// Get a reference to the key an item will be inserted with.
    fn get_key(&self) -> &Self::Key;

    /// Consume self and return the contained key.
    fn into_key(self) -> Self::Key;
}

pub trait IndexedVacantEntry<'a>: VacantEntry<'a>
where
    Self::Occupied: IndexedOccupiedEntry<'a>,
{
    /// Get the index the item will be inserted at.
    fn get_key(&self) -> usize;
}
