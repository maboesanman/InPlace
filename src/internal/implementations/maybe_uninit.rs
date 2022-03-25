use std::mem::MaybeUninit;

use crate::entry::{
    Entry, EntryRemovableOccupiedEntry, OccupiedEntry, RemovableOccupiedEntry, VacantEntry,
};

pub struct MaybeUninitEntry<'c, T, const OCCUPIED: bool>(&'c mut MaybeUninit<T>);

impl<'c, T> OccupiedEntry<'c> for MaybeUninitEntry<'c, T, true> {
    type Value = T;

    fn get_value<'e>(&'e self) -> &'e Self::Value
    where
        'c: 'e,
    {
        unsafe { self.0.assume_init_ref() }
    }

    fn get_value_mut<'e>(&'e mut self) -> &mut Self::Value
    where
        'c: 'e,
    {
        unsafe { self.0.assume_init_mut() }
    }

    fn into_value_mut<'e>(self) -> &'c mut Self::Value {
        unsafe { self.0.assume_init_mut() }
    }
}

impl<'c, T> RemovableOccupiedEntry<'c> for MaybeUninitEntry<'c, T, true> {
    type Removed = MaybeUninitEntry<'c, T, false>;

    fn remove(self) -> (Self::Value, Self::Removed) {
        let value = unsafe { self.0.assume_init_read() };
        let removed = MaybeUninitEntry::<'c, T, false>(self.0);
        (value, removed)
    }
}

impl<'c, T> EntryRemovableOccupiedEntry<'c> for MaybeUninitEntry<'c, T, true> {
    type Vacant = MaybeUninitEntry<'c, T, false>;

    fn recover_removed_entry(removed: Self::Removed) -> crate::entry::Entry<Self, Self::Vacant> {
        Entry::Vacant(removed)
    }
}

impl<'c, T> VacantEntry<'c> for MaybeUninitEntry<'c, T, false> {
    type Value = T;

    type Occupied = MaybeUninitEntry<'c, T, true>;

    fn occupy(self, value: Self::Value) -> Self::Occupied {
        self.0.write(value);
        MaybeUninitEntry::<'c, T, true>(self.0)
    }
}
