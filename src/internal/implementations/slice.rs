use crate::entry::IntoCollectionMut;

use super::super::{
    get_entry::{GetEntryByIndex, GetFirstEntry, GetLastEntry},
    occupied_entry::{KeyedOccupiedEntry, OccupiedEntry},
};

pub struct OccupiedMutSliceEntry<'c, V> {
    slice: &'c mut [V],
    index: usize,
}

impl<'c, V> OccupiedEntry<'c> for OccupiedMutSliceEntry<'c, V> {
    type Value = V;

    fn get_value<'e>(&'e self) -> &'e Self::Value
    where
        'c: 'e,
    {
        // SAFETY: the bounds checks are done when creating an occupied entry so we know they passed.
        unsafe { self.slice.get_unchecked(self.index) }
    }

    fn get_value_mut<'e>(&'e mut self) -> &mut Self::Value
    where
        'c: 'e,
    {
        // SAFETY: the bounds checks are done when creating an occupied entry so we know they passed.
        unsafe { self.slice.get_unchecked_mut(self.index) }
    }

    fn into_value_mut<'e>(self) -> &'c mut Self::Value {
        // SAFETY: the bounds checks are done when creating an occupied entry so we know they passed.
        unsafe { self.slice.get_unchecked_mut(self.index) }
    }
}

impl<'c, V> KeyedOccupiedEntry<'c> for OccupiedMutSliceEntry<'c, V> {
    type Key = usize;

    type BorrowedKey = usize;

    fn get_pair<'e>(&'e self) -> (&'e Self::Key, &'e Self::Value)
    where
        'c: 'e,
    {
        // SAFETY: the bounds checks are done when creating an occupied entry so we know they passed.
        (&self.index, unsafe { self.slice.get_unchecked(self.index) })
    }

    fn get_pair_mut<'e>(&'e mut self) -> (&'e Self::Key, &'e mut Self::Value)
    where
        'c: 'e,
    {
        // SAFETY: the bounds checks are done when creating an occupied entry so we know they passed.
        (&self.index, unsafe {
            self.slice.get_unchecked_mut(self.index)
        })
    }

    fn into_pair(self) -> (Self::BorrowedKey, &'c mut Self::Value) {
        // SAFETY: the bounds checks are done when creating an occupied entry so we know they passed.
        (self.index, unsafe {
            self.slice.get_unchecked_mut(self.index)
        })
    }
}

impl<V> GetFirstEntry<V> for [V] {
    type Occupied<'c> = OccupiedMutSliceEntry<'c, V>
    where
        Self: 'c;

    fn get_first_occupied<'c>(&'c mut self) -> Option<Self::Occupied<'c>> {
        if self.is_empty() {
            None
        } else {
            Some(OccupiedMutSliceEntry {
                slice: self,
                index: 0,
            })
        }
    }
}

impl<V> GetLastEntry<V> for [V] {
    type Occupied<'c> = OccupiedMutSliceEntry<'c, V>
    where
        Self: 'c;

    fn get_last_occupied<'c>(&'c mut self) -> Option<Self::Occupied<'c>> {
        if self.is_empty() {
            None
        } else {
            Some(OccupiedMutSliceEntry {
                index: self.len() - 1,
                slice: self,
            })
        }
    }
}

impl<V> GetEntryByIndex<V> for [V] {
    type Occupied<'c> = OccupiedMutSliceEntry<'c, V>
    where
        Self: 'c;

    fn get_occupied<'c>(&'c mut self, index: usize) -> Option<Self::Occupied<'c>> {
        if index < self.len() {
            Some(OccupiedMutSliceEntry { slice: self, index })
        } else {
            None
        }
    }
}

impl<'c, V> IntoCollectionMut<'c> for OccupiedMutSliceEntry<'c, V> {
    type Collection = [V];

    fn into_collection_mut(self) -> &'c mut Self::Collection {
        self.slice
    }
}
