use super::occupied_entry::{KeyedOccupiedEntry, OccupiedEntry, IndexedOccupiedEntry};

/// A trait to represent a vacant entry of a collection.
///
/// this can be thought of as holding a `K`, ready to insert when given a value.
///
/// The idea is that you've done hard work of finding your place in the collection,
/// so inserting shouldn't be a huge penalty at this point.
pub trait VacantEntry<'a, V>: Sized {
    /// The type of OccupiedEntry we convert to when inserting.
    type Occupied: OccupiedEntry<'a, V>;

    /// insert the `val` using the owned key.
    ///
    /// if you need to chain a future deletion, use `insert_entry`
    fn insert(self, val: V) {
        self.occupy(val);
    }

    /// insert the `val` using the owned key, returning the occupied entry
    fn occupy(self, val: V) -> Self::Occupied;
}

pub trait KeyedVacantEntry<'a, K, V>: VacantEntry<'a, V>
where Self::Occupied: KeyedOccupiedEntry<'a, K, V>
{

    /// Get a reference to the key an item will be inserted with.
    fn get_key(&self) -> &K;

    /// Consume self and return the contained key.
    fn into_key(self) -> K;
}

pub trait IndexedVacantEntry<'a, V>: VacantEntry<'a, V>
where Self::Occupied: IndexedOccupiedEntry<'a, V>
{
    /// Get the index the item will be inserted at.
    fn get_key(&self) -> usize;
}
