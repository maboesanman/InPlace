use super::{
    occupied_entry::{
        EntryRemovableOccupiedEntry, InsertableOccupiedEntry, KeyedOccupiedEntry,
        RemovableOccupiedEntry,
    },
    vacant_entry::{KeyedVacantEntry, VacantEntry},
};

/// An entry which is either Occupied or Vacant.
///
/// This is for manipulating key-value map collections,
/// for example HashMap, BTreeMap (also the Set variants, but those
/// just have `V` set to `()`).
pub enum Entry<Occ, Vac> {
    Occupied(Occ),
    Vacant(Vac),
}

impl<Occ, Vac> Entry<Occ, Vac> {
    pub fn from_occupied(entry: Occ) -> Self {
        Entry::Occupied(entry)
    }

    pub fn from_vacant(entry: Vac) -> Self {
        Entry::Vacant(entry)
    }
}

impl<'c, 'e, Occ, Vac> Entry<Occ, Vac>
where
    'c: 'e,
    Occ: KeyedOccupiedEntry<'c>,
    Vac: KeyedVacantEntry<'c, Key = Occ::Key, Value = Occ::Value, Occupied = Occ>,
{
    pub fn get_key(&'e self) -> &'e Occ::Key {
        match self {
            Entry::Occupied(e) => e.get_pair().0,
            Entry::Vacant(e) => e.get_key(),
        }
    }

    pub fn get_pair(&'e self) -> Result<(&'e Occ::Key, &'e Occ::Value), &'e Occ::Key> {
        match self {
            Entry::Occupied(e) => Ok(e.get_pair()),
            Entry::Vacant(e) => Err(e.get_key()),
        }
    }

    pub fn get_pair_mut(&'e mut self) -> Result<(&'e Occ::Key, &'e mut Occ::Value), &'e Occ::Key> {
        match self {
            Entry::Occupied(e) => Ok(e.get_pair_mut()),
            Entry::Vacant(e) => Err(e.get_key()),
        }
    }

    pub fn into_pair(self) -> Result<(Occ::BorrowedKey, &'c mut Occ::Value), Vac> {
        match self {
            Entry::Occupied(e) => Ok(e.into_pair()),
            Entry::Vacant(e) => Err(e),
        }
    }

    pub fn insert_into_entry(self, value: Occ::Value) -> (Self, Option<Occ::Value>) {
        let (occupied, old_value) = self.occupy(value);
        (Self::from_occupied(occupied), old_value)
    }

    /// set the value, returning the OccupiedEntry,
    /// and the displaced value if there was one.
    pub fn occupy(self, value: Occ::Value) -> (Occ, Option<Occ::Value>) {
        match self {
            Entry::Occupied(e) => {
                let mut e = e;
                let value = e.replace_value(value);
                (e, Some(value))
            }
            Entry::Vacant(e) => (e.occupy(value), None),
        }
    }

    pub fn is_occupied(&'e self) -> bool {
        matches!(self, Entry::Occupied(_))
    }

    pub fn is_vacant(&'e self) -> bool {
        matches!(self, Entry::Vacant(_))
    }
}

// when we remove an item, we either get a Occupied::Removed, or a Vacant.
// we can convert this back into Entry if we can convert Removed into Entry.
impl<'c, Occ, Vac> Entry<Occ, Vac>
where
    Occ: EntryRemovableOccupiedEntry<'c, Vacant = Vac>,
{
    pub fn remove_entry(self) -> (Self, Option<Occ::Value>) {
        match self {
            Entry::Occupied(occupied) => {
                let (value, removed) = occupied.remove();
                let entry = Occ::recover_removed_entry(removed);
                (entry, Some(value))
            }
            Entry::Vacant(vacant) => (Entry::Vacant(vacant), None),
        }
    }
}

// when we remove an item, we either get a Occupied::Removed, or a Vacant.
// we can convert this back into Entry if we can convert Removed into Entry.
impl<'c, Occ, Vac> Entry<Occ, Vac>
where
    Occ: RemovableOccupiedEntry<'c, Removed = Vac>,
{
    pub fn vacate(self) -> (Vac, Option<Occ::Value>) {
        match self {
            Entry::Occupied(occupied) => {
                let (value, vacant) = occupied.remove();
                (vacant, Some(value))
            }
            Entry::Vacant(vacant) => (vacant, None),
        }
    }
}

// when we remove an item, we either get a Occupied::Removed, or a Vacant.
// we can convert this back into Entry if we can convert Removed into Entry.
impl<'c, Occ, Vac> Entry<Occ, Vac>
where
    Occ: RemovableOccupiedEntry<'c, Removed = Vac>,
    Occ: KeyedOccupiedEntry<'c>,
    Vac: KeyedVacantEntry<'c, Key = Occ::Key, Occupied = Occ>,
{
    pub fn remove_with_key(self) -> (Occ::Key, Option<Occ::Value>) {
        let (vacant, value) = self.vacate();
        (vacant.into_key(), value)
    }
}

impl<'c, Occ, Vac> Entry<Occ, Vac>
where
    Occ: InsertableOccupiedEntry<'c>,
    Vac: VacantEntry<'c, Occupied = Occ, Value = Occ::Value>,
{
    pub fn insert_new(self, value: Occ::Value) {
        self.occupy_new(value);
    }

    pub fn occupy_new(self, value: Occ::Value) -> Occ {
        match self {
            Entry::Occupied(e) => e.insert_new(value),
            Entry::Vacant(e) => e.occupy(value),
        }
    }
}

pub enum EntryWithSearchKey<Occ, Vac, K> {
    Occupied(Occ, K),
    Vacant(Vac),
}

impl<'c, Occ, Vac, K> From<EntryWithSearchKey<Occ, Vac, K>> for Entry<Occ, Vac> {
    fn from(entry: EntryWithSearchKey<Occ, Vac, K>) -> Self {
        match entry {
            EntryWithSearchKey::Occupied(e, _) => Entry::Occupied(e),
            EntryWithSearchKey::Vacant(e) => Entry::Vacant(e),
        }
    }
}

pub trait IntoCollectionMut<'c> {
    type Collection: 'c + ?Sized;

    fn into_collection_mut(self) -> &'c mut Self::Collection;
}

impl<'c, Occ, Vec> IntoCollectionMut<'c> for Entry<Occ, Vec>
where
    Occ: IntoCollectionMut<'c>,
    Vec: IntoCollectionMut<'c, Collection = Occ::Collection>,
{
    type Collection = Occ::Collection;

    fn into_collection_mut(self) -> &'c mut Self::Collection {
        match self {
            Entry::Occupied(e) => e.into_collection_mut(),
            Entry::Vacant(e) => e.into_collection_mut(),
        }
    }
}
