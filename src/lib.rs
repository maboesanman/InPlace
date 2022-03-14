#![feature(generic_associated_types)]

mod entry;
mod ext;
mod lazy_entry;
mod option;

use entry::Entry;

/// A trait for in-place modification of collections.
/// 
/// C: InPlace<K, V> indicates that C is some kind of map from keys `K` to values `V`,
/// and that it owns those values.
/// 
/// when calling `my_in_place_collection.get_entry()` you are given an entry
/// that is either vacant or occupied. occupied entries can be used to modify the value in place,
/// or remove it entirely. removal consumes the OccupiedEntry, and returns a VacantEntry and a value.
/// 
/// you can also insert a value into a VacantEntry, which returns an OccupiedEntry.
/// 
/// you are able to flip flop as much as you like without having to re-query the collection, or
/// unwrap values you know are present.
pub trait InPlace<K, V> {
    /// An occupied entry. This is a handle to read and modify an item which is
    /// currently in your collection. The key should be thought of as immutably borrowed,
    /// and the value should be thought of as mutably borrowed.
    /// 
    /// this can be turned into a vacant via `remove`
    type Occupied<'a>: OccupiedEntry<'a, K, V, Self>
    where
        Self: 'a;

    /// A vacant entry. This is a handle to insert an item into your collection,
    /// and does not allow the caller to borrow any keys or items from the collection,
    /// though it likely has references or pointers into it under the hood.
    /// The key should be thought of as owned, and there is no value
    /// 
    /// this can be turned into a occupied via `insert`
    type Vacant<'a>: VacantEntry<'a, K, V, Self>
    where
        Self: 'a;

    /// get the entry for k, whether vacant or occupied.
    /// 
    /// the entry can be thought of as "preserving the work you did to search",
    /// so you can insert and remove to your heart's content without re-traversing the
    /// internal data structure, or unwrapping values you know you inserted.
    fn get_entry<'a, Q>(&'a mut self, k: Q) -> Entry<'a, K, V, Self>
    where
        Q: ToOwned<Owned = K>;
}

pub trait OccupiedEntry<'a, K, V, I: InPlace<K, V> + ?Sized + 'a>: Sized {
    fn get_pair(&self) -> (&K, &V);
    fn get_pair_mut(&mut self) -> (&K, &mut V);
    fn into_pair_mut(self) -> (&'a K, &'a mut V);
    fn remove(self) -> (I::Vacant<'a>, V);
    fn get_new_entry<Q>(self, k: Q) -> Entry<'a, K, V, I>
    where
        Q: ToOwned<Owned = K>;
    
    fn move_entry<Q>(self, key: Q) -> (I::Occupied<'a>, K, Option<V>)
    where
        Q: ToOwned<Owned = K>,
    {
        let (vacant, val) = self.remove();
        let (new_entry, old_key) = vacant.get_new_entry_old_key(key);
        let (new_entry, old_value) = new_entry.insert(val);

        (new_entry, old_key, old_value)
    }
}

pub trait VacantEntry<'a, K, V, I: InPlace<K, V> + ?Sized + 'a>: Sized {
    fn get_key(&self) -> &K;
    fn into_key(self) -> K;
    fn insert(self, val: V) -> I::Occupied<'a>;
    fn get_new_entry_old_key<Q>(self, k: Q) -> (Entry<'a, K, V, I>, K)
    where
        Q: ToOwned<Owned = K>;
}

pub trait OccupiedEntryKeyMut<'a, K, V, I: InPlace<K, V> + ?Sized + 'a>:
    OccupiedEntry<'a, K, V, I>
{
    /// replace key with another one.
    ///
    /// the data structure's integrity is maintained. If the value would move in the collection
    /// (for example if the key changed hash in a hashmap) then the error state is returned,
    /// with the new_key not inserted.
    fn replace_key(&mut self, key: K) -> Result<K, K>;

    unsafe fn get_key_mut_pair_unchecked(&mut self) -> (&mut K, &mut V);

    unsafe fn into_key_mut_pair_unchecked(self) -> (&'a mut K, &'a mut V);
}

pub trait InPlaceOrd<K, V>: InPlace<K, V>
where
    K: Ord,
{
    fn get_first_entry<'a>(&'a mut self) -> Option<Self::Occupied<'a>>;
    fn get_last_entry<'a>(&'a mut self) -> Option<Self::Occupied<'a>>;
}

pub trait InPlaceOrdEntry<'a, K: Ord, V, I: InPlace<K, V> + ?Sized + 'a>: Sized {
    fn get_next_entry(self) -> Option<I::Occupied<'a>>;
    fn get_prev_entry(self) -> Option<I::Occupied<'a>>;
}
