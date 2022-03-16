use std::{marker::PhantomData, borrow::Borrow};

use crate::internal::get_entry::{GetEntryFromKey, GetEntryByKey, EntryCollection, KeyedCollection};

use super::super::{
    entry::Entry,
    occupied_entry::{
        EntryRemovableOccupiedEntry, KeyedOccupiedEntry, OccupiedEntry,
        RemovableOccupiedEntry,
    },
    vacant_entry::VacantEntry,
};

pub struct DummyBTreeMap<K, V> {
    phantom: PhantomData<(K, V)>,
}
pub struct DummyBTreeMapOccupiedEntry<'a, K, V> {
    _map: &'a mut DummyBTreeMap<K, V>,
}
pub struct DummyBTreeMapVacantEntry<'a, K, V> {
    _map: &'a mut DummyBTreeMap<K, V>,
}

pub trait GetFirstEntry<V, Q>: EntryCollection<V> {
    fn get_first_occupied<'a>(&'a mut self) -> Option<Self::Occupied<'a>>;
}

pub trait GetLastEntry<V, Q>: EntryCollection<V> {
    fn get_last_occupied<'a>(&'a mut self) -> Option<Self::Occupied<'a>>;
}

impl<K, V> EntryCollection<V> for DummyBTreeMap<K, V> {
    type Occupied<'a>
    where K: 'a, V: 'a = DummyBTreeMapOccupiedEntry<'a, K, V>;
}

impl<K: Eq, V> KeyedCollection<K, V> for DummyBTreeMap<K, V> {
    type Occupied<'a>
    where K: 'a, V: 'a = DummyBTreeMapOccupiedEntry<'a, K, V>;

    type Vacant<'a>
    where K: 'a, V: 'a = DummyBTreeMapVacantEntry<'a, K, V>;
}

impl<K, V> GetEntryFromKey<K, V> for DummyBTreeMap<K, V>
where
    K: Eq
{
    fn get_entry_with_key<'a>(&'a mut self, key: K) -> crate::internal::entry::EntryWithSearchKey<'a, K, V, Self::Vacant<'a>> {
        todo!()
    }
}

impl<K, V, Q> GetEntryByKey<K, V, Q> for DummyBTreeMap<K, V>
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

impl<'a, K, V> OccupiedEntry<'a, V> for DummyBTreeMapOccupiedEntry<'a, K, V> {
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

impl<'a, K: Eq, V> KeyedOccupiedEntry<'a, K, V> for DummyBTreeMapOccupiedEntry<'a, K, V> {
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

impl<'a, K: Eq, V> RemovableOccupiedEntry<'a, V> for DummyBTreeMapOccupiedEntry<'a, K, V> {
    type Removed = DummyBTreeMapVacantEntry<'a, K, V>;

    fn remove(self) -> (V, Self::Removed) {
        todo!()
    }
}

impl<'a, K: Eq, V> EntryRemovableOccupiedEntry<'a, K, V>
    for DummyBTreeMapOccupiedEntry<'a, K, V>
{
    type Vacant = DummyBTreeMapVacantEntry<'a, K, V>;

    fn recover_removed_entry(removed: Self::Removed) -> Entry<'a, K, V, Self::Vacant> {
        Entry::Vacant(removed)
    }
}

impl<'a, K: Eq, V> VacantEntry<'a, K, V> for DummyBTreeMapVacantEntry<'a, K, V> {
    type Occupied = DummyBTreeMapOccupiedEntry<'a, K, V>;

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
