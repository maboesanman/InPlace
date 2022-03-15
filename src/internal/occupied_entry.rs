use super::{in_place::InPlace, entry::Entry};


/// A trait to represent an occupied entry of a collection.
/// 
/// this can be thought of as holding a `&K`, and a `&mut V` into the collection.
pub trait OccupiedEntry<'a, K, V, I: InPlace<K, V, Occupied<'a> = Self> + ?Sized + 'a>: Sized {
    /// get the key value pair, immutably borrowed
    fn get_pair(&self) -> (&K, &V);

    /// get the key value pair.
    /// the key is immutably borrowed, and the value immutably.
    /// 
    /// note the lifetime of these borrows is independent from the entry.
    /// 
    /// only the value is allowed to be mutated, because mutating a key
    /// is likely to violate some invariants of the data structure.
    fn get_pair_mut(&mut self) -> (&K, &mut V);

    /// convert the entry into key value pair.
    /// the key is immutably borrowed, and the value immutably.
    /// 
    /// note the lifetime of these borrows is the same as the destroyed entry.
    /// 
    /// only the value is allowed to be mutated, because mutating a key
    /// is likely to violate some invariants of the data structure.
    fn into_pair_mut(self) -> (&'a K, &'a mut V);

    /// remove this entry from the collection, consuming the entry
    /// and returning the value that was there.
    /// 
    /// if you need to chain a future insertion, use `remove_entry`
    fn remove(self) -> V {
        self.remove_entry().1
    }

    /// remove this entry from the collection, consuming the entry
    /// and returning the vacant entry along with the value.
    fn remove_entry(self) -> (I::Vacant<'a>, V);

    /// get a completely new entry, as if from calling get_entry on the 
    /// collection again.
    /// 
    /// if you need to store an entry for some reason, it can be really frustrating
    /// if you determine you need to look for a new key. This allows you to get that entry
    /// easily.
    fn get_new_entry<Q>(self, k: Q) -> Entry<'a, K, V, I>
    where
        Q: ToOwned<Owned = K>;
    

    /// Move an existing value in the collection to a new key, returning the old key,
    /// and possibly the value displaced from the new location.
    fn move_entry<Q>(&mut self, key: Q) -> (K, Option<V>)
    where
        Q: ToOwned<Owned = K>;
}

/// A trait to extend OccupiedEntry, allowing the user to mutate keys.
/// 
/// They are only able to do so through a `replace_key` function, not a regular mutable reference.
pub trait OccupiedEntryKeyMut<'a, K, V, I: InPlace<K, V, Occupied<'a> = Self> + ?Sized + 'a>:
    OccupiedEntry<'a, K, V, I>
{
    /// replace key with another one.
    ///
    /// the data structure's integrity is maintained. If the value would move in the collection
    /// (for example if the key changed hash in a hashmap) then the error state is returned,
    /// with the new_key not inserted.
    fn replace_key(&mut self, key: K) -> Result<K, K>;
}