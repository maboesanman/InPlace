use super::{
    occupied_entry::{IndexedOccupiedEntry, KeyedOccupiedEntry, OccupiedEntry},
    vacant_entry::{IndexedVacantEntry, KeyedVacantEntry, VacantEntry},
};

pub trait EntryCollection {
    type Value;

    type Occupied<'a>: OccupiedEntry<'a, Value = Self::Value>;
    type Vacant<'a>: VacantEntry<'a, Value = Self::Value, Occupied = Self::Occupied<'a>>;
}
