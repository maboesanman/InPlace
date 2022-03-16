use std::borrow::Borrow;

use super::{entry::{Entry, EntryWithSearchKey}, vacant_entry::{VacantEntry, KeyedVacantEntry, IndexedVacantEntry}, occupied_entry::{OccupiedEntry, KeyedOccupiedEntry, IndexedOccupiedEntry}};

pub trait EntryCollection<V> {
    type Occupied<'a>: OccupiedEntry<'a, V>
    where Self: 'a;
}

pub trait KeyedCollection<K, V>
{
    type Occupied<'a>: KeyedOccupiedEntry<'a, K, V>
    where Self: 'a;

    /// A vacant entry. This is a handle to insert an item into your collection,
    /// and does not allow the caller to borrow any keys or items from the collection,
    /// though it likely has references or pointers into it under the hood.
    /// The key should be thought of as owned, and there is no value
    ///
    /// this can be turned into an OccupiedEntry via `insert`
    type Vacant<'a>: KeyedVacantEntry<'a, K, V, Occupied = Self::Occupied<'a>>
    where Self: 'a;
}

pub trait IndexedCollection<V>
{
    type Occupied<'a>: IndexedOccupiedEntry<'a, V>
    where Self: 'a;

    /// A vacant entry. This is a handle to insert an item into your collection,
    /// and does not allow the caller to borrow any keys or items from the collection,
    /// though it likely has references or pointers into it under the hood.
    /// The key should be thought of as owned, and there is no value
    ///
    /// this can be turned into an OccupiedEntry via `insert`
    type Vacant<'a>: IndexedVacantEntry<'a, V, Occupied = Self::Occupied<'a>>
    where Self: 'a;
}

/// A trait for in-place modification of items in collections.
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
pub trait GetEntryFromKey<K: Eq, V>: KeyedCollection<K, V>
{
    fn get_entry_from_key<'a>(&'a mut self, key: K) -> Entry<'a, V, Self::Vacant<'a>> {
        self.get_entry_with_key(key).into()
    }

    fn get_entry_with_key<'a>(&'a mut self, key: K) -> EntryWithSearchKey<'a, K, V, Self::Vacant<'a>>;

    fn insert_into_entry<'a>(&'a mut self, key: K, value: V) -> (Entry<'a, V, Self::Vacant<'a>>, Option<V>) {
        self.get_entry_from_key(key).insert_into_entry(value)
    }

    fn occupy<'a>(&'a mut self, key: K, value: V) -> (<Self::Vacant<'a> as VacantEntry<'a, V>>::Occupied, Option<V>) {
        self.get_entry_from_key(key).occupy(value)
    }
}

pub trait GetEntryByKey<K, V, Q>: GetEntryFromKey<K, V>
where
    K: Borrow<Q> + Eq + Clone,
    Q: Eq,
{
    fn get_entry<'a>(&'a mut self, key: &Q) -> Entry<'a, V, Self::Vacant<'a>>;

    fn remove_entry<'a>(&'a mut self, key: &Q) -> (Entry<'a, V, Self::Vacant<'a>>, Option<V>) {
        let (vacant, value) = self.vacate(key);
        (Entry::from_vacant(vacant), value)
    }

    fn vacate<'a>(&'a mut self, key: &Q) -> (Self::Vacant<'a>, Option<V>);
}

pub trait GetFirstEntry<V>: EntryCollection<V> {
    fn get_first_occupied<'a>(&'a mut self) -> Option<Self::Occupied<'a>>;
}

pub trait GetLastEntry<V>: EntryCollection<V> {
    fn get_last_occupied<'a>(&'a mut self) -> Option<Self::Occupied<'a>>;
}

pub trait GetEntryByIndex<V>: EntryCollection<V> {
    fn get_occupied<'a>(&'a mut self, index: usize) -> Option<Self::Occupied<'a>>;
}
