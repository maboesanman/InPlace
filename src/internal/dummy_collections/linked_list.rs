use crate::{internal::{
    entry::Entry,
    get_entry::{GetEntryByKey, GetEntryFromKey},
    occupied_entry::*,
    vacant_entry::*,
}, get_entry::{GetFirstEntry, GetLastEntry}};

use core::hash::Hash;
use core::{borrow::Borrow, marker::PhantomData};

/// stand in for std::collections::LinkedList
pub struct DummyLinkedList<V> {
    phantom: PhantomData<V>,
}

/// Occupied entry of a LinkedList.
/// 
/// This entry can be used to read the index, mutate the value, and remove.
/// removing this entry gives an occupied entry representing the item that was to the right.
/// if there was no item to the right, you get a vacant entry representing the end.
pub struct DummyLinkedListOccupiedEntry<'c, V> {
    _map: &'c mut DummyLinkedList<V>,
}

/// Vacant entry of a LinkedList.
/// 
/// This is either the beginning of the linked list, the end, or 
pub struct DummyLinkedListVacantEntry<'c, V> {
    _map: &'c mut DummyLinkedList<V>,
}

/// An entry of a LinkedList. This is either `DummyLinkedListOccupiedEntry` or `DummyLinkedListVacantEntry`.
pub type DummyLinkedListEntry<'c, V> =
    Entry<DummyLinkedListOccupiedEntry<'c, V>, DummyLinkedListVacantEntry<'c, V>>;

impl<V> DummyLinkedList<V> {
    fn get_head<'c>(&'c mut self) -> DummyLinkedListVacantEntry<'c, V> {
        todo!()
    }

    fn get_tail<'c>(&'c mut self) -> DummyLinkedListVacantEntry<'c, V> {
        todo!()
    }
}

impl<V> GetFirstEntry<V> for DummyLinkedList<V>
{
    type Occupied<'c> = DummyLinkedListOccupiedEntry<'c, V>
    where
        Self: 'c;

    fn get_first_occupied<'c>(&'c mut self) -> Option<Self::Occupied<'c>> {
        todo!()
    }
}

impl<V> GetLastEntry<V> for DummyLinkedList<V>
{
    type Occupied<'c> = DummyLinkedListOccupiedEntry<'c, V>
    where
        Self: 'c;

    fn get_last_occupied<'c>(&'c mut self) -> Option<Self::Occupied<'c>> {
        todo!()
    }
}

impl<'c, V> OccupiedEntry<'c> for DummyLinkedListOccupiedEntry<'c, V> {
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

impl<'c, V> KeyedOccupiedEntry<'c> for DummyLinkedListOccupiedEntry<'c, V> {
    type Key = usize;

    type BorrowedKey = usize;

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

impl<'c, V> RemovableOccupiedEntry<'c> for DummyLinkedListOccupiedEntry<'c, V> {
    type Removed = DummyLinkedListEntry<'c, V>;

    fn remove(self) -> (V, Self::Removed) {
        todo!()
    }
}

impl<'c, V> EntryRemovableOccupiedEntry<'c> for DummyLinkedListOccupiedEntry<'c, V> {
    type Vacant = DummyLinkedListVacantEntry<'c, V>;

    fn recover_removed_entry(removed: Self::Removed) -> DummyLinkedListEntry<'c, V> {
        removed
    }
}


impl<'c, V> NextOccupiedFromOccupied<'c> for DummyLinkedListOccupiedEntry<'c, V> {
    fn get_next_occupied(self) -> Option<Self> {
        todo!()
    }
}

impl<'c, V> PrevOccupiedFromOccupied<'c> for DummyLinkedListOccupiedEntry<'c, V> {
    fn get_prev_occupied(self) -> Option<Self> {
        todo!()
    }
}

impl<'c, V> VacantEntry<'c> for DummyLinkedListVacantEntry<'c, V> {
    type Occupied = DummyLinkedListOccupiedEntry<'c, V>;

    type Value = V;

    fn occupy(self, val: Self::Value) -> Self::Occupied {
        todo!()
    }
}

impl<'c, V> KeyedVacantEntry<'c> for DummyLinkedListVacantEntry<'c, V> {
    /// this may be the head of the linked list, which preceeds the 0 indexed item.
    type Key = Option<usize>;

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

impl<'c, V> NextOccupiedFromVacant<'c> for DummyLinkedListVacantEntry<'c, V>
{
    fn get_next_occupied(self) -> Option<Self::Occupied> {
        todo!()
    }
}

impl<'c, V> PrevOccupiedFromVacant<'c> for DummyLinkedListVacantEntry<'c, V>

{
    fn get_prev_occupied(self) -> Option<Self::Occupied> {
        todo!()
    }
}

impl<'c, V> DummyLinkedListOccupiedEntry<'c, V>
{
    fn get_next_entry(self) -> DummyLinkedListEntry<'c, V> {
        todo!()
    }
}

impl<'c, V> DummyLinkedListOccupiedEntry<'c, V>
{
    fn get_prev_entry(self) -> DummyLinkedListEntry<'c, V> {
        todo!()
    }
}

impl<'c, V> DummyLinkedListVacantEntry<'c, V>
{
    fn get_next_entry(self) -> Option<DummyLinkedListEntry<'c, V>> {
        todo!()
    }
}


impl<'c, V> DummyLinkedListVacantEntry<'c, V>
{
    fn get_prev_entry(self) -> Option<DummyLinkedListEntry<'c, V>> {
        todo!()
    }
}

