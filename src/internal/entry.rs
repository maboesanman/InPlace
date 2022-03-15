use std::hint::unreachable_unchecked;

use super::{in_place::InPlace, ord::InPlaceOrdEntry, occupied_entry::OccupiedEntry, ext::{OccupiedEntryExt, VacantEntryExt}, vacant_entry::VacantEntry};


pub enum Entry<'a, K, V, I: InPlace<K, V> + ?Sized + 'a> {
    Occupied(I::Occupied<'a>),
    Vacant(I::Vacant<'a>),
    Poisioned,
}

impl<'a, K, V, I: InPlace<K, V> + ?Sized + 'a> Entry<'a, K, V, I> {
    pub fn get_new_entry<Q>(self, k: Q) -> Entry<'a, K, V, I>
    where
        Q: ToOwned<Owned = K>,
    {
        match self {
            Entry::Occupied(e) => e.get_new_entry(k),
            Entry::Vacant(e) => e.get_new_entry(k),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    pub fn get_key(&self) -> &K {
        match self {
            Entry::Occupied(e) => e.get_pair().0,
            Entry::Vacant(e) => e.get_key(),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    pub fn get_pair(&self) -> Result<(&K, &V), &K> {
        match self {
            Entry::Occupied(e) => Ok(e.get_pair()),
            Entry::Vacant(e) => Err(e.get_key()),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    pub fn get_pair_mut(&mut self) -> Result<(&K, &mut V), &K> {
        match self {
            Entry::Occupied(e) => Ok(e.get_pair_mut()),
            Entry::Vacant(e) => Err(e.get_key()),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    pub fn into_pair_mut(self) -> Result<(&'a K, &'a mut V), I::Vacant<'a>> {
        match self {
            Entry::Occupied(e) => Ok(e.into_pair_mut()),
            Entry::Vacant(e) => Err(e),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    pub fn into_key(self) -> Result<K, I::Occupied<'a>> {
        match self {
            Entry::Occupied(e) => Err(e),
            Entry::Vacant(e) => Ok(e.into_key()),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    pub fn insert(&mut self, val: V) -> Option<V> {
        let owned_self = core::mem::replace(self, Entry::Poisioned);
        let (occupied, prev) = owned_self.insert_entry(val);
        *self = Entry::Occupied(occupied);
        prev
    }

    pub fn insert_entry(self, val: V) -> (I::Occupied<'a>, Option<V>) {
        match self {
            Entry::Occupied(mut e) => {
                let val = e.replace_value(val);
                (e, Some(val))
            }
            Entry::Vacant(e) => (e.insert_entry(val), None),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    pub fn remove(&mut self) -> Option<V> {
        let owned_self = core::mem::replace(self, Entry::Poisioned);
        let (vacant, val) = owned_self.remove_entry();
        *self = Entry::Vacant(vacant);
        val
    }

    pub fn remove_entry(self) -> (I::Vacant<'a>, Option<V>) {
        match self {
            Entry::Occupied(occupied) => {
                let (vacant, val) = occupied.remove_entry();
                (vacant, Some(val))
            }
            Entry::Vacant(vacant) => (vacant, None),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    pub fn is_occupied(&self) -> bool {
        matches!(self, Entry::Occupied(_))
    }

    pub fn is_vacant(&self) -> bool {
        matches!(self, Entry::Vacant(_))
    }
}

impl<'a, K, V, I> InPlaceOrdEntry<'a, K, V, I> for Entry<'a, K, V, I>
where
    K: Ord,
    I: InPlace<K, V> + ?Sized + 'a,
    I::Occupied<'a>: InPlaceOrdEntry<'a, K, V, I>,
    I::Vacant<'a>: InPlaceOrdEntry<'a, K, V, I>,
{
    fn get_next_entry(self) -> Option<I::Occupied<'a>> {
        match self {
            Entry::Occupied(e) => e.get_next_entry(),
            Entry::Vacant(e) => e.get_next_entry(),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }

    fn get_prev_entry(self) -> Option<I::Occupied<'a>> {
        match self {
            Entry::Occupied(e) => e.get_prev_entry(),
            Entry::Vacant(e) => e.get_prev_entry(),
            Entry::Poisioned => unsafe { unreachable_unchecked() }
        }
    }
}
