use crate::{
    ext::{OccupiedEntryExt, VacantEntryExt},
    InPlace, InPlaceOrdEntry, OccupiedEntry, VacantEntry,
};

pub enum Entry<'a, K, V, I: InPlace<K, V> + ?Sized + 'a> {
    Occupied(I::Occupied<'a>),
    Vacant(I::Vacant<'a>),
}

impl<'a, K, V, I: InPlace<K, V> + ?Sized + 'a> Entry<'a, K, V, I> {
    pub fn get_new_entry<Q>(self, k: Q) -> Entry<'a, K, V, I>
    where
        Q: ToOwned<Owned = K>,
    {
        match self {
            Entry::Occupied(e) => e.get_new_entry(k),
            Entry::Vacant(e) => e.get_new_entry(k),
        }
    }

    pub fn get_key(&self) -> &K {
        match self {
            Entry::Occupied(e) => e.get_pair().0,
            Entry::Vacant(e) => e.get_key(),
        }
    }

    pub fn get_pair(&self) -> Result<(&K, &V), &K> {
        match self {
            Entry::Occupied(e) => Ok(e.get_pair()),
            Entry::Vacant(e) => Err(e.get_key()),
        }
    }

    pub fn get_pair_mut(&mut self) -> Result<(&K, &mut V), &K> {
        match self {
            Entry::Occupied(e) => Ok(e.get_pair_mut()),
            Entry::Vacant(e) => Err(e.get_key()),
        }
    }

    pub fn into_pair_mut(self) -> Result<(&'a K, &'a mut V), I::Vacant<'a>> {
        match self {
            Entry::Occupied(e) => Ok(e.into_pair_mut()),
            Entry::Vacant(e) => Err(e),
        }
    }

    pub fn into_key(self) -> Result<K, I::Occupied<'a>> {
        match self {
            Entry::Occupied(e) => Err(e),
            Entry::Vacant(e) => Ok(e.into_key()),
        }
    }

    pub fn insert(self, val: V) -> (I::Occupied<'a>, Option<V>) {
        match self {
            Entry::Occupied(mut e) => {
                let val = e.replace_value(val);
                (e, Some(val))
            }
            Entry::Vacant(e) => (e.insert(val), None),
        }
    }

    pub fn remove(self) -> (I::Vacant<'a>, Option<V>) {
        match self {
            Entry::Occupied(occupied) => {
                let (vacant, val) = occupied.remove();
                (vacant, Some(val))
            }
            Entry::Vacant(vacant) => (vacant, None),
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
        }
    }

    fn get_prev_entry(self) -> Option<I::Occupied<'a>> {
        match self {
            Entry::Occupied(e) => e.get_prev_entry(),
            Entry::Vacant(e) => e.get_prev_entry(),
        }
    }
}
