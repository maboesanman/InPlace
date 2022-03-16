use std::marker::PhantomData;

use crate::internal::{occupied_entry::{OccupiedEntry, IndexedOccupiedEntry, RemovableOccupiedEntry}, entry::Entry, vacant_entry::VacantEntry};


pub struct DummyLinkedList<V> {
    phantom: PhantomData<V>,
}
pub struct DummyLinkedListOccupiedEntry<'a, V> {
    _map: &'a mut DummyLinkedList<V>,
}
pub struct DummyLinkedListVacantEntry<'a, V> {
    _map: &'a mut DummyLinkedList<V>,
}

impl<'a, V> OccupiedEntry<'a, V> for DummyLinkedListOccupiedEntry<'a, V> {
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

impl<'a, V> IndexedOccupiedEntry<'a, V> for DummyLinkedListOccupiedEntry<'a, V> {
    fn get_pair(&self) -> (usize, &V) {
        todo!()
    }

    fn get_pair_mut(&mut self) -> (usize, &mut V) {
        todo!()
    }

    fn into_pair(self) -> (usize, &'a mut V) {
        todo!()
    }
}

impl<'a, V> RemovableOccupiedEntry<'a, V> for DummyLinkedListOccupiedEntry<'a, V> {
    type Removed = Entry<'a, (), V, DummyLinkedListVacantEntry<'a, V>>;

    fn remove(self) -> (V, Self::Removed) {
        todo!()
    }
}

impl<'a, V> VacantEntry<'a, (), V> for DummyLinkedListVacantEntry<'a, V> {
    type Occupied = DummyLinkedListOccupiedEntry<'a, V>;

    fn get_key(&self) -> &() {
        todo!()
    }

    fn into_key(self) -> () {
        todo!()
    }

    fn occupy(self, val: V) -> Self::Occupied {
        todo!()
    }
}