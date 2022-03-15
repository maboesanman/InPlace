use crate::entry::{OccupiedEntry, VacantEntry};

use super::{entry::Entry, in_place::InPlace};

/// A trait to represent an occupied entry of a collection.
///
/// this can be thought of as holding a `&K`, and a `&mut V` into the collection.
pub trait RenewableOccupiedEntry<'a, K, V, I>: OccupiedEntry<'a, K, V, I>
where
    K: Eq,
    I: InPlace<K, V, Occupied<'a> = Self> + ?Sized + 'a,
{
    /// get a completely new entry, as if from calling get_entry on the
    /// collection again.
    ///
    /// if you need to store an entry for some reason, it can be really frustrating
    /// if you determine you need to look for a new key. This allows you to get that entry
    /// easily.
    fn get_new_entry(self, k: K) -> Entry<'a, K, V, I>;

    /// Move an existing value in the collection to a new key, returning the old key,
    /// and possibly the value displaced from the new location.
    fn move_entry(&mut self, key: K) -> (K, Option<V>);
}

/// A trait to represent a vacant entry of a collection.
///
/// this can be thought of as holding a `K`.
///
/// The idea is that you've done hard work of finding your place in the collection,
/// so inserting shouldn't be a huge penalty at this point.
pub trait RenewableVacantEntry<'a, K, V, I>: VacantEntry<'a, K, V, I>
where
    K: Eq,
    I: InPlace<K, V, Vacant<'a> = Self> + ?Sized + 'a,
{
    /// get a completely new entry, as if from calling get_entry on the
    /// collection again.
    ///
    /// also return the key we own.
    ///
    /// if you need to store an entry for some reason, it can be really frustrating
    /// if you determine you need to look for a new key. This allows you to get that entry
    /// easily.
    fn get_new_entry_old_key(self, k: K) -> (Entry<'a, K, V, I>, K);
}
