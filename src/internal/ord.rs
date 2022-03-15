use super::in_place::InPlace;


/// A trait to extend InPlace, for collections whose keys are ordered.
pub trait InPlaceOrd<K, V>: InPlace<K, V>
where
    K: Ord,
{
    /// get the entry whose key is the lowest in the collection, or None if it's empty.
    fn get_min_entry<'a>(&'a mut self) -> Option<Self::Occupied<'a>>;

    /// get the entry whose key is the greatest in the collection, or None if it's empty.
    fn get_max_entry<'a>(&'a mut self) -> Option<Self::Occupied<'a>>;
}

/// A trait to extend both OccupiedEntry and VacantEntry, for collections whose keys are ordered.
/// 
/// this trait allows for moving between entries.
pub trait InPlaceOrdEntry<'a, K: Ord, V, I: InPlace<K, V> + ?Sized + 'a>: Sized {

    /// get the entry with the lowest key greater than self.get_key(), or None if there isn't one
    fn get_next_entry(self) -> Option<I::Occupied<'a>>;

    /// get the entry with the greatest key lower than self.get_key(), or None if there isn't one
    fn get_prev_entry(self) -> Option<I::Occupied<'a>>;
}