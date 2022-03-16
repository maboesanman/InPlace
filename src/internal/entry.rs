use super::{
    occupied_entry::{
        EntryRemovableOccupiedEntry, InsertableOccupiedEntry,
        KeyedOccupiedEntry, OccupiedEntry, RemovableOccupiedEntry,
    },
    vacant_entry::VacantEntry,
};

/// An entry which is either Occupied or Vacant.
///
/// This is for manipulating key-value map collections,
/// for example HashMap, BTreeMap (also the Set variants, but those
/// just have `V` set to `()`).
pub enum Entry<'a, V, Vac>
where
    Vac: VacantEntry<'a, V>,
{
    Occupied(Vac::Occupied),
    Vacant(Vac),
}

impl<'a, V, Vac> Entry<'a, V, Vac>
where
    Vac: VacantEntry<'a, V>,
{
    pub fn from_occupied(entry: Vac::Occupied) -> Self {
        Entry::Occupied(entry)
    }

    pub fn from_vacant(entry: Vac) -> Self {
        Entry::Vacant(entry)
    }
}

impl<'a, K, V, Vac> Entry<'a, V, Vac>
where
    K: Eq,
    Vac: VacantEntry<'a, V>,
    Vac::Occupied: KeyedOccupiedEntry<'a, K, V>,
{
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

    pub fn into_pair(self) -> Result<(&'a K, &'a mut V), Vac> {
        match self {
            Entry::Occupied(e) => Ok(e.into_pair()),
            Entry::Vacant(e) => Err(e),
        }
    }

    pub fn insert_into_entry(self, value: V) -> (Self, Option<V>) {
        let (occupied, old_value) = self.occupy(value);
        (Self::from_occupied(occupied), old_value)
    }

    /// set the value, returning the OccupiedEntry,
    /// and the displaced value if there was one.
    pub fn occupy(self, value: V) -> (Vac::Occupied, Option<V>) {
        match self {
            Entry::Occupied(mut e) => {
                let value = e.replace_value(value);
                (e, Some(value))
            }
            Entry::Vacant(e) => (e.occupy(value), None),
        }
    }
}

// when we remove an item, we either get a Occupied::Removed, or a Vacant.
// we can convert this back into Entry if we can convert Removed into Entry.
impl<'a, V, Vac> Entry<'a, V, Vac>
where
    Vac: VacantEntry<'a, V>,
    Vac::Occupied: EntryRemovableOccupiedEntry<'a, V, Vacant = Vac>,
{
    pub fn remove_entry(self) -> (Self, Option<V>) {
        match self {
            Entry::Occupied(occupied) => {
                let (value, removed) = occupied.remove();
                let entry = Vac::Occupied::recover_removed_entry(removed);
                (entry, Some(value))
            }
            Entry::Vacant(vacant) => (Entry::Vacant(vacant), None),
        }
    }
}

// when we remove an item, we either get a Occupied::Removed, or a Vacant.
// we can convert this back into Entry if we can convert Removed into Entry.
impl<'a, V, Vac> Entry<'a, V, Vac>
where
    Vac: VacantEntry<'a, V>,
    Vac::Occupied: RemovableOccupiedEntry<'a, V, Removed = Vac>,
{
    // pub fn remove_with_key(self) -> (K, Option<V>) {
    //     let (vacant, value) = self.vacate();
    //     (vacant.into_key(), value)
    // }

    pub fn vacate(self) -> (Vac, Option<V>) {
        match self {
            Entry::Occupied(occupied) => {
                let (value, vacant) = occupied.remove();
                (vacant, Some(value))
            }
            Entry::Vacant(vacant) => (vacant, None),
        }
    }
}

impl<'a, V, Vac> Entry<'a, V, Vac>
where
    V: 'a,
    Vac: VacantEntry<'a, V>,
    Vac::Occupied: InsertableOccupiedEntry<'a, V>,
{
    pub fn insert_new(self, value: V) {
        self.insert_new_entry(value);
    }

    pub fn insert_new_entry(self, value: V) -> Vac::Occupied {
        match self {
            Entry::Occupied(e) => e.insert_new(value),
            Entry::Vacant(e) => e.occupy(value),
        }
    }
}

pub enum EntryWithSearchKey<'a, K, V, Vac>
where
    K: Eq,
    Vac: VacantEntry<'a, V>,
{
    Occupied(Vac::Occupied, K),
    Vacant(Vac),
}

impl<'a, K, V, Vac> From<EntryWithSearchKey<'a, K, V, Vac>> for Entry<'a, V, Vac>
where
    K: Eq,
    Vac: VacantEntry<'a, V>,
{
    fn from(entry: EntryWithSearchKey<'a, K, V, Vac>) -> Self {
        match entry {
            EntryWithSearchKey::Occupied(e, _) => Entry::Occupied(e),
            EntryWithSearchKey::Vacant(e) => Entry::Vacant(e),
        }
    }
}
