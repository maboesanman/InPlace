use core::hint::unreachable_unchecked;

use super::{
    entry::Entry,
    in_place::InPlace,
    occupied_entry::OccupiedEntry,
    renewable::{RenewableOccupiedEntry, RenewableVacantEntry},
    vacant_entry::VacantEntry,
};

impl<T> InPlace<(), T> for Option<T> {
    type Occupied<'a>
    where
        Self: 'a,
    = OptionOccupiedEntry<'a, T>;

    type Vacant<'a>
    where
        Self: 'a,
    = OptionVacantEntry<'a, T>;

    fn get_entry<'a>(&'a mut self, _k: ()) -> Entry<'a, (), T, Self> {
        match self {
            Some(_) => Entry::Occupied(OptionOccupiedEntry { option: self }),
            None => Entry::Vacant(OptionVacantEntry { option: self }),
        }
    }

    fn get_entry_clone_key<'a>(&'a mut self, _k: &()) -> Entry<'a, (), T, Self> {
        self.get_entry(())
    }
}

pub struct OptionOccupiedEntry<'a, T> {
    option: &'a mut Option<T>,
}

impl<'a, T> OccupiedEntry<'a, (), T, Option<T>> for OptionOccupiedEntry<'a, T> {
    fn get_pair(&self) -> (&(), &T) {
        (Box::leak(Box::new(())), unsafe {
            self.option.as_ref().unwrap_unchecked()
        })
    }

    fn get_pair_mut(&mut self) -> (&(), &mut T) {
        (Box::leak(Box::new(())), unsafe {
            self.option.as_mut().unwrap_unchecked()
        })
    }

    fn into_pair_mut(self) -> (&'a (), &'a mut T) {
        (Box::leak(Box::new(())), unsafe {
            self.option.as_mut().unwrap_unchecked()
        })
    }

    fn remove_entry(self) -> (OptionVacantEntry<'a, T>, T) {
        match core::mem::take(self.option) {
            Some(val) => (
                OptionVacantEntry {
                    option: self.option,
                },
                val,
            ),
            None => unsafe { unreachable_unchecked() },
        }
    }
}

impl<'a, T> RenewableOccupiedEntry<'a, (), T, Option<T>> for OptionOccupiedEntry<'a, T> {
    fn get_new_entry(self, _k: ()) -> Entry<'a, (), T, Option<T>> {
        Entry::Occupied(self)
    }

    fn move_entry(&mut self, _key: ()) -> ((), Option<T>) {
        ((), None)
    }
}

pub struct OptionVacantEntry<'a, T> {
    option: &'a mut Option<T>,
}

impl<'a, T> VacantEntry<'a, (), T, Option<T>> for OptionVacantEntry<'a, T> {
    fn get_key(&self) -> &() {
        Box::leak(Box::new(()))
    }

    fn into_key(self) -> () {}

    fn insert_entry(self, val: T) -> OptionOccupiedEntry<'a, T> {
        match core::mem::replace(self.option, Some(val)) {
            Some(_) => unsafe { unreachable_unchecked() },
            None => OptionOccupiedEntry {
                option: self.option,
            },
        }
    }
}

impl<'a, T> RenewableVacantEntry<'a, (), T, Option<T>> for OptionVacantEntry<'a, T> {
    fn get_new_entry_old_key(self, _k: ()) -> (Entry<'a, (), T, Option<T>>, ()) {
        (Entry::Vacant(self), ())
    }
}
