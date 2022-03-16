use super::{collection::EntryCollection, entry::Entry, vacant_entry::VacantEntry};

/// A trait to represent an occupied entry of a collection.
///
/// this can be thought of as holding a `&mut V` into the collection.
pub trait OccupiedEntry<'a>: Sized {
    /// the type of the values in the collection
    type Value;

    /// get the value, immutably borrowed.
    fn get_value(&self) -> &Self::Value;

    /// get the value, mutably borrowed.
    fn get_value_mut(&mut self) -> &mut Self::Value;

    /// convert the entry into a mutable to the value.
    fn into_value_mut(self) -> &'a mut Self::Value;

    /// replace the value in the entry, returning the old value.
    fn replace_value(&mut self, value: Self::Value) -> Self::Value {
        core::mem::replace(self.get_value_mut(), value)
    }
}

/// A trait to represent an occupied entry of a collection which owns its keys.
///
/// this can be thought of as holding a `&K`, and a `&mut V` into the collection.
pub trait KeyedOccupiedEntry<'a>: OccupiedEntry<'a> {
    /// the type of the keys in the collection
    type Key;

    /// get the key value pair, immutably borrowed
    fn get_pair(&self) -> (&Self::Key, &Self::Value);

    /// get the key value pair.
    /// the key is immutably borrowed, and the value immutably.
    ///
    /// note the lifetime of these borrows is independent from the entry.
    ///
    /// only the value is allowed to be mutated, because mutating a key
    /// is likely to violate some invariants of the data structure.
    fn get_pair_mut(&mut self) -> (&Self::Key, &mut Self::Value);

    /// convert the entry into key value pair.
    /// the key is immutably borrowed, and the value immutably.
    ///
    /// note the lifetime of these borrows is the same as the destroyed entry.
    ///
    /// only the value is allowed to be mutated, because mutating a key
    /// is likely to violate some invariants of the data structure.
    fn into_pair(self) -> (&'a Self::Key, &'a mut Self::Value);
}

/// A trait to represent an occupied entry of a collection which owns its keys.
///
/// this can be thought of as holding a `&K`, and a `&mut V` into the collection.
pub trait IndexedOccupiedEntry<'a>: OccupiedEntry<'a> {
    /// get the key value pair, immutably borrowed
    fn get_pair(&self) -> (usize, &Self::Value);

    fn get_pair_mut(&mut self) -> (usize, &mut Self::Value);

    fn into_pair(self) -> (usize, &'a mut Self::Value);
}

/// A trait to represent an OccupiedEntry that can be removed from the collection.
///
/// This will consume the OccupiedEntry, returning RemovableOccupiedEntry::Removed.
pub trait RemovableOccupiedEntry<'a>: OccupiedEntry<'a> {
    /// The type returned when removing the OccupiedEntry
    type Removed;

    /// remove this entry from the collection, converting the entry
    /// into a Self::Removed and returning the value that was there.
    fn remove(self) -> (Self::Value, Self::Removed);

    /// remove this entry from the collection, consuming the entry
    /// and returning the value that was there.
    fn remove_value(self) -> Self::Value {
        self.remove().0
    }
}

/// A trait to represent recovering to an entry on removal.
pub trait EntryRemovableOccupiedEntry<'a, C>: RemovableOccupiedEntry<'a>
where
    C: EntryCollection,
{
    type Vacant: VacantEntry<'a, Occupied = Self>;

    fn recover_removed_entry(removed: Self::Removed) -> Entry<'a, C>;
}

/// A trait to represent inserting a new entry on top of an existing one.
///
/// This is only if you can insert without overwriting the last item, but
/// with the same key.
pub trait InsertableOccupiedEntry<'a>: OccupiedEntry<'a> {
    /// insert a new element at this position, shifting the others out of the way.
    ///
    /// the old item's key will no longer equal the new item's key.
    ///
    /// this is most likely to look like an insertion into a LinkedList, where
    /// the other items move to have greater indices.
    fn insert_new(self, value: Self::Value) -> Self;
}

/// A trait to extend OccupiedEntry, allowing the user to mutate keys.
///
/// They are only able to do so through a `replace_key` function, not a regular mutable reference,
/// so the implementer can ensure that collection invariants still hold for the new key.
pub trait OccupiedEntryKeyMut<'a>: KeyedOccupiedEntry<'a> {
    /// replace key with another one.
    ///
    /// the data structure's integrity is maintained. If the value would move in the collection
    /// (for example if the key changed hash in a hashmap) then the error state is returned,
    /// with the new_key not inserted.
    fn replace_key(&mut self, key: Self::Key) -> Result<Self::Key, Self::Key>;
}
