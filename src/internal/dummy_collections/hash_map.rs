use crate::internal::get_entry::{GetEntryFromKey, GetEntryByKey, EntryCollection, KeyedCollection};

use super::super::{
    entry::Entry,
    occupied_entry::{
        EntryRemovableOccupiedEntry, KeyedOccupiedEntry, OccupiedEntry,
        RemovableOccupiedEntry,
    },
    vacant_entry::VacantEntry,
};
use std::{marker::PhantomData, borrow::Borrow};

pub struct DummyHashMap<K, V> {
    phantom: PhantomData<(K, V)>,
}
pub struct DummyHashMapOccupiedEntry<'a, K, V> {
    _map: &'a mut DummyHashMap<K, V>,
}
pub struct DummyHashMapVacantEntry<'a, K, V> {
    _map: &'a mut DummyHashMap<K, V>,
}

impl<K, V> EntryCollection<V> for DummyHashMap<K, V> {
    type Occupied<'a>
    where K: 'a, V: 'a = DummyHashMapOccupiedEntry<'a, K, V>;
}

impl<K: Eq, V> KeyedCollection<K, V> for DummyHashMap<K, V> {
    type Occupied<'a>
    where K: 'a, V: 'a = DummyHashMapOccupiedEntry<'a, K, V>;

    type Vacant<'a>
    where K: 'a, V: 'a = DummyHashMapVacantEntry<'a, K, V>;
}

impl<K, V> GetEntryFromKey<K, V> for DummyHashMap<K, V>
where
    K: Eq
{
    fn get_entry_with_key<'a>(&'a mut self, key: K) -> crate::internal::entry::EntryWithSearchKey<'a, K, V, Self::Vacant<'a>> {
        todo!()
    }
}

impl<K, V, Q> GetEntryByKey<K, V, Q> for DummyHashMap<K, V>
where
    K: Borrow<Q> + Eq + Clone,
    Q: Eq
{
    fn get_entry<'a>(&'a mut self, key: &Q) -> Entry<'a, K, V, Self::Vacant<'a>> {
        todo!()
    }

    fn vacate<'a>(&'a mut self, key: &Q) -> (Self::Vacant<'a>, Option<V>) {
        todo!()
    }
}

impl<'a, K, V> OccupiedEntry<'a, V> for DummyHashMapOccupiedEntry<'a, K, V> {
    fn get_value(&self) -> &V {
        todo!()
    }

    fn get_value_mut(&mut self) -> &mut V {
        todo!()
    }

    fn into_value_mut(self) -> &'a mut V {
        todo!()
    }
}

impl<'a, K: Eq, V> KeyedOccupiedEntry<'a, K, V> for DummyHashMapOccupiedEntry<'a, K, V> {
    fn get_pair(&self) -> (&K, &V) {
        todo!()
    }

    fn get_pair_mut(&mut self) -> (&K, &mut V) {
        todo!()
    }

    fn into_pair(self) -> (&'a K, &'a mut V) {
        todo!()
    }
}

impl<'a, K: Eq, V> RemovableOccupiedEntry<'a, V> for DummyHashMapOccupiedEntry<'a, K, V> {
    type Removed = DummyHashMapVacantEntry<'a, K, V>;

    fn remove(self) -> (V, Self::Removed) {
        todo!()
    }
}

impl<'a, K: Eq, V> EntryRemovableOccupiedEntry<'a, K, V>
    for DummyHashMapOccupiedEntry<'a, K, V>
{
    type Vacant = DummyHashMapVacantEntry<'a, K, V>;

    fn recover_removed_entry(removed: Self::Removed) -> Entry<'a, K, V, Self::Vacant> {
        Entry::Vacant(removed)
    }
}

impl<'a, K: Eq, V> VacantEntry<'a, K, V> for DummyHashMapVacantEntry<'a, K, V> {
    type Occupied = DummyHashMapOccupiedEntry<'a, K, V>;

    fn get_key(&self) -> &K {
        todo!()
    }

    fn into_key(self) -> K {
        todo!()
    }

    fn occupy(self, val: V) -> Self::Occupied {
        todo!()
    }
}
