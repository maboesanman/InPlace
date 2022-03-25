use crate::internal::{
    entry::Entry,
    get_entry::{GetEntryByKey, GetEntryFromKey},
    occupied_entry::*,
    vacant_entry::*,
};

use core::hash::Hash;
use core::{borrow::Borrow, marker::PhantomData};

/// stand in for std::collections::HashMap
pub struct DummyHashMap<K, V> {
    phantom: PhantomData<(K, V)>,
}

/// Occupied entry of a HashMap.
///
/// This entry can be used to read the key, mutate the value,
/// or remove the key/value pair from the HashMap, without re-hashing.
pub struct DummyHashMapOccupiedEntry<'c, K, V> {
    _map: &'c mut DummyHashMap<K, V>,
}

/// Vacant entry of a HashMap.
///
/// This contains the key used to search for this entry,
/// and can read that key, or insert into the HashMap at that key,
/// when provided a value.
pub struct DummyHashMapVacantEntry<'c, K, V> {
    _map: &'c mut DummyHashMap<K, V>,
}

/// Raw vacant entry of a HashMap.
///
/// This is acquired from the `get_raw_entry` method, and can insert into
/// the HashMap when provided a `(K, V)` tuple where K hashes to the hash used
/// to aquire this entry. Inserting with a (K, V) tuple gives a `DummyHashMapOccupiedEntry`
pub struct DummyHashMapRawVacantEntry<'c, K, V> {
    _map: &'c mut DummyHashMap<K, V>,
}

/// An entry of a HashMap. This is either `DummyHashMapOccupiedEntry` or `DummyHashMapVacantEntry`.
pub type DummyHashMapEntry<'c, K, V> =
    Entry<DummyHashMapOccupiedEntry<'c, K, V>, DummyHashMapVacantEntry<'c, K, V>>;

/// A raw entry of a HashMap. This is either `DummyHashMapOccupiedEntry` or `DummyHashMapRawVacantEntry`.
pub type DummyHashMapRawEntry<'c, K, V> =
    Entry<DummyHashMapOccupiedEntry<'c, K, V>, DummyHashMapRawVacantEntry<'c, K, V>>;

impl<K, V> GetEntryFromKey<K, V> for DummyHashMap<K, V>
where
    K: Eq + Hash,
{
    type Occupied<'c> = DummyHashMapOccupiedEntry<'c, K, V>
    where
        Self: 'c;

    type Vacant<'c> = DummyHashMapVacantEntry<'c, K, V>
    where
        Self: 'c;

    fn get_entry_with_key<'c>(
        &'c mut self,
        key: K,
    ) -> crate::internal::entry::EntryWithSearchKey<Self::Occupied<'c>, Self::Vacant<'c>, K> {
        todo!()
    }
}

impl<K, V, Q> GetEntryByKey<K, V, Q> for DummyHashMap<K, V>
where
    K: Borrow<Q> + Clone + Eq + Hash,
    Q: Eq,
{
    fn get_entry<'c>(&'c mut self, key: &Q) -> Entry<Self::Occupied<'c>, Self::Vacant<'c>> {
        todo!()
    }

    fn vacate<'c>(&'c mut self, key: &Q) -> (Self::Vacant<'c>, Option<V>) {
        todo!()
    }
}

impl<'c, K, V> OccupiedEntry<'c> for DummyHashMapOccupiedEntry<'c, K, V> {
    type Value = V;

    fn get_value<'e>(&'e self) -> &'e Self::Value
    where
        'c: 'e,
    {
        todo!()
    }

    fn get_value_mut<'e>(&'e mut self) -> &mut Self::Value
    where
        'c: 'e,
    {
        todo!()
    }

    fn into_value_mut<'e>(self) -> &'c mut Self::Value {
        todo!()
    }
}

impl<'c, K, V> KeyedOccupiedEntry<'c> for DummyHashMapOccupiedEntry<'c, K, V> {
    type Key = K;

    type BorrowedKey = &'c K;

    fn get_pair<'e>(&'e self) -> (&'e Self::Key, &'e Self::Value)
    where
        'c: 'e,
    {
        todo!()
    }

    fn get_pair_mut<'e>(&'e mut self) -> (&'e Self::Key, &'e mut Self::Value)
    where
        'c: 'e,
    {
        todo!()
    }

    fn into_pair(self) -> (Self::BorrowedKey, &'c mut Self::Value) {
        todo!()
    }
}

impl<'c, K, V> RemovableOccupiedEntry<'c> for DummyHashMapOccupiedEntry<'c, K, V> {
    type Removed = DummyHashMapVacantEntry<'c, K, V>;

    fn remove(self) -> (V, Self::Removed) {
        todo!()
    }
}

impl<'c, K, V> EntryRemovableOccupiedEntry<'c> for DummyHashMapOccupiedEntry<'c, K, V> {
    type Vacant = DummyHashMapVacantEntry<'c, K, V>;

    fn recover_removed_entry(removed: Self::Removed) -> DummyHashMapEntry<'c, K, V> {
        Entry::Vacant(removed)
    }
}

impl<'c, K, V> VacantEntry<'c> for DummyHashMapVacantEntry<'c, K, V> {
    type Occupied = DummyHashMapOccupiedEntry<'c, K, V>;

    type Value = V;

    fn occupy(self, val: Self::Value) -> Self::Occupied {
        todo!()
    }
}

impl<'c, K, V> KeyedVacantEntry<'c> for DummyHashMapVacantEntry<'c, K, V> {
    type Key = K;

    fn get_key<'e>(&'e self) -> &'e Self::Key
    where
        'c: 'e,
    {
        todo!()
    }

    fn into_key(self) -> Self::Key {
        todo!()
    }
}

impl<'c, K, V> VacantEntry<'c> for DummyHashMapRawVacantEntry<'c, K, V> {
    type Occupied = DummyHashMapOccupiedEntry<'c, K, V>;

    type Value = (K, V);

    fn occupy(self, val: Self::Value) -> Self::Occupied {
        todo!()
    }
}

impl<'c, K, V> DummyHashMapRawVacantEntry<'c, K, V> {
    /// elevate a raw vacant entry to a regular vacant entry by providing a key.
    fn occupy_key(self, key: K) -> DummyHashMapVacantEntry<'c, K, V> {
        todo!()
    }
}

impl<K, V> DummyHashMap<K, V> {
    /// get a raw entry for a given hash.
    fn get_raw_entry(&mut self, hash: u64) -> DummyHashMapRawEntry<'_, K, V> {
        todo!()
    }
}
