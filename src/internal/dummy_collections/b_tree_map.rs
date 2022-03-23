use crate::{internal::{
    entry::Entry,
    get_entry::{GetEntryByKey, GetEntryFromKey},
    occupied_entry::*,
    vacant_entry::*,
}, get_entry::{GetFirstEntry, GetLastEntry}};

use core::{borrow::Borrow, marker::PhantomData};

/// stand in for std::collections::BTreeMap
pub struct DummyBTreeMap<K, V> {
    phantom: PhantomData<(K, V)>,
}

/// Occupied entry of a BTreeMap.
/// 
/// This entry can be used to read the key, mutate the value,
/// or remove the key/value pair from the BTreeMap, without re-searching for the key.
pub struct DummyBTreeMapOccupiedEntry<'c, K, V> {
    _map: &'c mut DummyBTreeMap<K, V>,
}

/// Vacant entry of a BTreeMap.
/// 
/// This contains the key used to search for this entry,
/// and can read that key, or insert into the BTreeMap at that key,
/// when provided a value.
pub struct DummyBTreeMapVacantEntry<'c, K, V> {
    _map: &'c mut DummyBTreeMap<K, V>,
}

/// Raw vacant entry of a BTreeMap.
/// 
/// This is acquired from the `get_raw_entry` method, and can insert into
/// the BTreeMap when provided a `(K, V)` tuple where K sorts the same as the query used
/// to aquire this entry. Inserting with a (K, V) tuple gives a `DummyBTreeMapOccupiedEntry`
pub struct DummyBTreeMapRawVacantEntry<'c, K, V> {
    _map: &'c mut DummyBTreeMap<K, V>,
}

/// An entry of a BTreeMap. This is either `DummyBTreeMapOccupiedEntry` or `DummyBTreeMapVacantEntry`.
pub type DummyBTreeMapEntry<'c, K, V> =
    Entry<DummyBTreeMapOccupiedEntry<'c, K, V>, DummyBTreeMapVacantEntry<'c, K, V>>;

/// A raw entry of a BTreeMap. This is either `DummyBTreeMapOccupiedEntry` or `DummyBTreeMapRawVacantEntry`.
pub type DummyBTreeMapRawEntry<'c, K, V> =
    Entry<DummyBTreeMapOccupiedEntry<'c, K, V>, DummyBTreeMapRawVacantEntry<'c, K, V>>;

impl<K, V> GetEntryFromKey<K, V> for DummyBTreeMap<K, V>
where
    K: Ord,
{
    type Occupied<'c> = DummyBTreeMapOccupiedEntry<'c, K, V>
    where
        Self: 'c;

    type Vacant<'c> = DummyBTreeMapVacantEntry<'c, K, V>
    where
        Self: 'c;

    fn get_entry_with_key<'c>(
        &'c mut self,
        key: K,
    ) -> crate::internal::entry::EntryWithSearchKey<Self::Occupied<'c>, Self::Vacant<'c>, K> {
        todo!()
    }
}

impl<K, V, Q> GetEntryByKey<K, V, Q> for DummyBTreeMap<K, V>
where
    K: Borrow<Q> + Clone + Ord,
    Q: Ord,
{
    fn get_entry<'c>(&'c mut self, key: &Q) -> Entry<Self::Occupied<'c>, Self::Vacant<'c>> {
        todo!()
    }

    fn vacate<'c>(&'c mut self, key: &Q) -> (Self::Vacant<'c>, Option<V>) {
        todo!()
    }
}

impl<K, V> GetFirstEntry<V> for DummyBTreeMap<K, V>
where
    K: Ord,
{
    type Occupied<'c> = DummyBTreeMapOccupiedEntry<'c, K, V>
    where
        Self: 'c;

    fn get_first_occupied<'c>(&'c mut self) -> Option<Self::Occupied<'c>> {
        todo!()
    }
}

impl<K, V> GetLastEntry<V> for DummyBTreeMap<K, V>
where
    K: Ord,
{
    type Occupied<'c> = DummyBTreeMapOccupiedEntry<'c, K, V>
    where
        Self: 'c;

    fn get_last_occupied<'c>(&'c mut self) -> Option<Self::Occupied<'c>> {
        todo!()
    }
}

impl<'c, K, V> OccupiedEntry<'c> for DummyBTreeMapOccupiedEntry<'c, K, V> {
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

impl<'c, K, V> KeyedOccupiedEntry<'c> for DummyBTreeMapOccupiedEntry<'c, K, V> {
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

impl<'c, K, V> RemovableOccupiedEntry<'c> for DummyBTreeMapOccupiedEntry<'c, K, V> {
    type Removed = DummyBTreeMapVacantEntry<'c, K, V>;

    fn remove(self) -> (V, Self::Removed) {
        todo!()
    }
}

impl<'c, K, V> EntryRemovableOccupiedEntry<'c> for DummyBTreeMapOccupiedEntry<'c, K, V> {
    type Vacant = DummyBTreeMapVacantEntry<'c, K, V>;

    fn recover_removed_entry(removed: Self::Removed) -> DummyBTreeMapEntry<'c, K, V> {
        Entry::Vacant(removed)
    }
}


impl<'c, K, V> NextOccupiedFromOccupied<'c> for DummyBTreeMapOccupiedEntry<'c, K, V>
where K: Ord
{
    fn get_next_occupied(mut self) -> Option<Self> {
        todo!()
    }
}

impl<'c, K, V> PrevOccupiedFromOccupied<'c> for DummyBTreeMapOccupiedEntry<'c, K, V>
where K: Ord
{
    fn get_prev_occupied(mut self) -> Option<Self> {
        todo!()
    }
}

impl<'c, K, V> VacantEntry<'c> for DummyBTreeMapVacantEntry<'c, K, V> {
    type Occupied = DummyBTreeMapOccupiedEntry<'c, K, V>;

    type Value = V;

    fn occupy(self, val: Self::Value) -> Self::Occupied {
        todo!()
    }
}

impl<'c, K, V> KeyedVacantEntry<'c> for DummyBTreeMapVacantEntry<'c, K, V> {
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

impl<'c, K, V> NextOccupiedFromVacant<'c> for DummyBTreeMapVacantEntry<'c, K, V>
where K: Ord
{
    fn get_next_occupied(mut self) -> Option<Self::Occupied> {
        todo!()
    }
}

impl<'c, K, V> PrevOccupiedFromVacant<'c> for DummyBTreeMapVacantEntry<'c, K, V>
where K: Ord
{
    fn get_prev_occupied(mut self) -> Option<Self::Occupied> {
        todo!()
    }
}

impl<'c, K, V> VacantEntry<'c> for DummyBTreeMapRawVacantEntry<'c, K, V> {
    type Occupied = DummyBTreeMapOccupiedEntry<'c, K, V>;

    type Value = (K, V);

    fn occupy(self, val: Self::Value) -> Self::Occupied {
        todo!()
    }
}

impl<'c, K, V> NextOccupiedFromVacant<'c> for DummyBTreeMapRawVacantEntry<'c, K, V>
where K: Ord
{
    fn get_next_occupied(mut self) -> Option<Self::Occupied> {
        todo!()
    }
}

impl<'c, K, V> PrevOccupiedFromVacant<'c> for DummyBTreeMapRawVacantEntry<'c, K, V>
where K: Ord
{
    fn get_prev_occupied(mut self) -> Option<Self::Occupied> {
        todo!()
    }
}

impl<'c, K, V> DummyBTreeMapRawVacantEntry<'c, K, V> {

    /// Elevate a raw vacant entry to a regular vacant entry by providing a key.
    /// 
    /// This key must evaluate to `Ordering::Equal` when passed to the comparator and query
    /// to maintain BTreeMap invariants.
    fn occupy_key(self, key: K) -> DummyBTreeMapVacantEntry<'c, K, V> {
        todo!()
    }
}

impl<K, V> DummyBTreeMap<K, V> {

    /// get a raw entry for a given query value and comparator.
    /// 
    /// comparator must observe the transitive property for K and Q.
    /// (k1 < q) && (q < k2) => (k1 < k2).
    /// 
    /// when inserting into this entry, `comparator(&inserted_key, &query)` must evaluate to `Ordering::Equal`.
    fn get_raw_entry<Q, Cmp>(&mut self, query: Q, comparator: Cmp) -> DummyBTreeMapRawEntry<'_, K, V>
    where Cmp: Fn(&K, &Q) -> std::cmp::Ordering
    {
        todo!()
    }
}
