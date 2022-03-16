use super::{
    collection::{EntryCollection},
    occupied_entry::{
        EntryRemovableOccupiedEntry, IndexedOccupiedEntry, InsertableOccupiedEntry,
        KeyedOccupiedEntry, OccupiedEntry, RemovableOccupiedEntry,
    },
    vacant_entry::{IndexedVacantEntry, KeyedVacantEntry, VacantEntry},
};

/// An entry which is either Occupied or Vacant.
///
/// This is for manipulating key-value map collections,
/// for example HashMap, BTreeMap (also the Set variants, but those
/// just have `V` set to `()`).
pub enum Entry<'a, C: EntryCollection> {
    Occupied(C::Occupied<'a>),
    Vacant(C::Vacant<'a>),
}

impl<'a, C> Entry<'a, C>
where
    C: EntryCollection,
{
    pub fn from_occupied(entry: C::Occupied<'a>) -> Self {
        Entry::Occupied(entry)
    }

    pub fn from_vacant(entry: C::Vacant<'a>) -> Self {
        Entry::Vacant(entry)
    }
}

impl<'a, K, V, C> Entry<'a, C>
where
V: 'a,
C: EntryCollection<Value = V>,
for<'b> <C as EntryCollection>::Vacant<'b>: KeyedVacantEntry<'b, Key = K>,
for<'b> <C as EntryCollection>::Occupied<'b>: KeyedOccupiedEntry<'b, Key = K>,
{
    pub fn get_key(&self) -> &K {
        match self {
            Entry::Occupied(e) => e.get_pair().0,
            Entry::Vacant(e) => e.get_key(),
        }
    }

    pub fn get_pair(&self) -> Result<(&K, &C::Value), &K> {
        match self {
            Entry::Occupied(e) => Ok(e.get_pair()),
            Entry::Vacant(e) => Err(e.get_key()),
        }
    }

    pub fn get_pair_mut(&mut self) -> Result<(&K, &mut C::Value), &K> {
        match self {
            Entry::Occupied(e) => Ok(e.get_pair_mut()),
            Entry::Vacant(e) => Err(e.get_key()),
        }
    }

    pub fn into_pair(self) -> Result<(&'a K, &'a mut C::Value), C::Vacant<'a>> {
        match self {
            Entry::Occupied(e) => Ok(e.into_pair()),
            Entry::Vacant(e) => Err(e),
        }
    }

    pub fn insert_into_entry(self, value: C::Value) -> (Self, Option<C::Value>) {
        let (occupied, old_value) = self.occupy(value);
        (Self::from_occupied(occupied), old_value)
    }

    /// set the value, returning the OccupiedEntry,
    /// and the displaced value if there was one.
    pub fn occupy(self, value: C::Value) -> (C::Occupied<'a>, Option<C::Value>) {
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
impl<'a, C> Entry<'a, C>
where
    C: EntryCollection,
    C::Occupied<'a>: EntryRemovableOccupiedEntry<'a, C>,
{
    pub fn remove_entry(self) -> (Self, Option<C::Value>) {
        match self {
            Entry::Occupied(occupied) => {
                let (value, removed) = occupied.remove();
                let entry = C::Occupied::recover_removed_entry(removed);
                (entry, Some(value))
            }
            Entry::Vacant(vacant) => (Entry::Vacant(vacant), None),
        }
    }
}

// when we remove an item, we either get a Occupied::Removed, or a Vacant.
// we can convert this back into Entry if we can convert Removed into Entry.
impl<'a, C> Entry<'a, C>
where
    C: EntryCollection,
    C::Occupied<'a>: RemovableOccupiedEntry<'a, Removed = C::Vacant<'a>>,
{
    // pub fn remove_with_key(self) -> (K, Option<V>) {
    //     let (vacant, value) = self.vacate();
    //     (vacant.into_key(), value)
    // }

    pub fn vacate(self) -> (C::Vacant<'a>, Option<C::Value>) {
        match self {
            Entry::Occupied(occupied) => {
                let (value, vacant) = occupied.remove();
                (vacant, Some(value))
            }
            Entry::Vacant(vacant) => (vacant, None),
        }
    }
}

impl<'a, C> Entry<'a, C>
where
    C: EntryCollection,
    C::Occupied<'a>: InsertableOccupiedEntry<'a>,
{
    pub fn insert_new(self, value: C::Value) {
        self.insert_new_entry(value);
    }

    pub fn insert_new_entry(self, value: C::Value) -> C::Occupied<'a> {
        match self {
            Entry::Occupied(e) => e.insert_new(value),
            Entry::Vacant(e) => e.occupy(value),
        }
    }
}

pub enum EntryWithSearchKey<'a, C>
where
C: EntryCollection,
for<'b> <C as EntryCollection>::Vacant<'b>: KeyedVacantEntry<'b>,
for<'b> <C as EntryCollection>::Occupied<'b>: KeyedOccupiedEntry<'b>,
{
    Occupied(C::Occupied<'a>, <<C as EntryCollection>::Occupied<'a> as KeyedOccupiedEntry<'a>>::Key),
    Vacant(C::Vacant<'a>),
}

impl<'a, K, C> From<EntryWithSearchKey<'a, C>> for Entry<'a, C>
where
C: EntryCollection,
for<'b> <C as EntryCollection>::Vacant<'b>: KeyedVacantEntry<'b>,
for<'b> <C as EntryCollection>::Occupied<'b>: KeyedOccupiedEntry<'b, Key = K>,
{
    fn from(entry: EntryWithSearchKey<'a, C>) -> Self {
        match entry {
            EntryWithSearchKey::Occupied(e, _) => Entry::Occupied(e),
            EntryWithSearchKey::Vacant(e) => Entry::Vacant(e),
        }
    }
}
