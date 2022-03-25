use std::borrow::Borrow;

use super::{
    entry::{Entry, EntryWithSearchKey},
    occupied_entry::{KeyedOccupiedEntry, OccupiedEntry},
    vacant_entry::KeyedVacantEntry,
};

pub trait GetEntryFromKey<K, V> {
    type Occupied<'c>: KeyedOccupiedEntry<'c, Key = K, Value = V>
    where
        Self: 'c;

    type Vacant<'c>: KeyedVacantEntry<'c, Key = K, Value = V, Occupied = Self::Occupied<'c>>
    where
        Self: 'c;

    fn get_entry_from_key<'c>(&'c mut self, key: K) -> Entry<Self::Occupied<'c>, Self::Vacant<'c>> {
        self.get_entry_with_key(key).into()
    }

    fn get_entry_with_key<'c>(
        &'c mut self,
        key: K,
    ) -> EntryWithSearchKey<Self::Occupied<'c>, Self::Vacant<'c>, K>;

    fn insert_into_entry<'c>(
        &'c mut self,
        key: K,
        value: V,
    ) -> (Entry<Self::Occupied<'c>, Self::Vacant<'c>>, Option<V>) {
        self.get_entry_from_key(key).insert_into_entry(value)
    }

    fn occupy<'c>(&'c mut self, key: K, value: V) -> (Self::Occupied<'c>, Option<V>) {
        self.get_entry_from_key(key).occupy(value)
    }
}

pub trait GetEntryByKey<K, V, Q>: GetEntryFromKey<K, V>
where
    K: Borrow<Q> + Eq,
    Q: Eq,
{
    fn get_entry<'c>(&'c mut self, key: &Q) -> Entry<Self::Occupied<'c>, Self::Vacant<'c>>
    where
        K: Clone;

    fn remove_entry<'c>(
        &'c mut self,
        key: &Q,
    ) -> (Entry<Self::Occupied<'c>, Self::Vacant<'c>>, Option<V>) {
        let (vacant, value) = self.vacate(key);
        (Entry::from_vacant(vacant), value)
    }

    fn vacate<'c>(&'c mut self, key: &Q) -> (Self::Vacant<'c>, Option<V>);
}

pub trait GetFirstEntry<V> {
    type Occupied<'c>: OccupiedEntry<'c, Value = V>
    where
        Self: 'c;

    fn get_first_occupied<'c>(&'c mut self) -> Option<Self::Occupied<'c>>;
}

pub trait GetLastEntry<V> {
    type Occupied<'c>: OccupiedEntry<'c, Value = V>
    where
        Self: 'c;

    fn get_last_occupied<'c>(&'c mut self) -> Option<Self::Occupied<'c>>;
}

pub trait GetEntryByIndex<V> {
    type Occupied<'c>: OccupiedEntry<'c, Value = V>
    where
        Self: 'c;

    fn get_occupied<'c>(&'c mut self, index: usize) -> Option<Self::Occupied<'c>>;
}
