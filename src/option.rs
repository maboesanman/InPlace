use std::hint::unreachable_unchecked;

use crate::{entry::Entry, InPlace, OccupiedEntry, VacantEntry};

impl<T> InPlace<(), T> for Option<T> {
    type Occupied<'a>
    where
        Self: 'a,
    = OptionOccupiedEntry<'a, T>;

    type Vacant<'a>
    where
        Self: 'a,
    = OptionVacantEntry<'a, T>;

    fn get_entry<'a, 'q, Q>(&'a mut self, _k: Q) -> crate::entry::Entry<'a, (), T, Self>
    where
        Q: ToOwned<Owned = ()>,
    {
        match self {
            Some(_) => Entry::Occupied(OptionOccupiedEntry { option: self }),
            None => Entry::Vacant(OptionVacantEntry { option: self }),
        }
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

    fn remove(self) -> (OptionVacantEntry<'a, T>, T) {
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

    fn get_new_entry<Q>(self, _k: Q) -> Entry<'a, (), T, Option<T>>
    where
        Q: ToOwned<Owned = ()>,
    {
        Entry::Occupied(self)
    }

    fn move_entry<Q>(self, _key: Q) -> (OptionOccupiedEntry<'a, T>, (), Option<T>)
    where
        Q: ToOwned<Owned = ()>,
    {
        (self, (), None)
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

    fn insert(self, val: T) -> OptionOccupiedEntry<'a, T> {
        match core::mem::replace(self.option, Some(val)) {
            Some(_) => unsafe { unreachable_unchecked() },
            None => OptionOccupiedEntry {
                option: self.option,
            },
        }
    }

    fn get_new_entry_old_key<Q>(self, _k: Q) -> (Entry<'a, (), T, Option<T>>, ())
    where
        Q: ToOwned<Owned = ()>,
    {
        (Entry::Vacant(self), ())
    }
}
