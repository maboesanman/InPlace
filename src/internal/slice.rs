use super::{get_entry::{EntryCollection, GetFirstEntry, GetLastEntry, GetEntryByIndex}, occupied_entry::{OccupiedEntry, IndexedOccupiedEntry}};

impl<V> EntryCollection<V> for [V] {
    type Occupied<'a>
    where Self: 'a = OccupiedMutSliceEntry<'a, V>;
}

impl<V> GetFirstEntry<V> for [V] {
    fn get_first_occupied<'a>(&'a mut self) -> Option<Self::Occupied<'a>> {
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
    fn get_last_occupied<'a>(&'a mut self) -> Option<Self::Occupied<'a>> {
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
    fn get_occupied<'a>(&'a mut self, index: usize) -> Option<Self::Occupied<'a>> {
        if index < self.len() {
            Some(OccupiedMutSliceEntry {
                slice: self,
                index,
            })
        } else {
            None
        }
    }
}

pub struct OccupiedMutSliceEntry<'a, V> {
    slice: &'a mut[V],
    index: usize,
}

impl<'a, V> OccupiedEntry<'a, V> for OccupiedMutSliceEntry<'a, V> {
    fn get_value(&self) -> &V {
        unsafe { self.slice.get_unchecked(self.index)}
    }

    fn get_value_mut(&mut self) -> &mut V {
        unsafe { self.slice.get_unchecked_mut(self.index)}
    }

    fn into_value_mut(self) -> &'a mut V {
        unsafe { self.slice.get_unchecked_mut(self.index)}
    }
}

impl<'a, V> IndexedOccupiedEntry<'a, V> for OccupiedMutSliceEntry<'a, V> {
    fn get_pair(&self) -> (usize, &V) {
        (self.index, unsafe { self.slice.get_unchecked(self.index) })
    }

    fn get_pair_mut(&mut self) -> (usize, &mut V) {
        (self.index, unsafe { self.slice.get_unchecked_mut(self.index) })
    }

    fn into_pair(self) -> (usize, &'a mut V) {
        (self.index, unsafe { self.slice.get_unchecked_mut(self.index) })
    }
}
