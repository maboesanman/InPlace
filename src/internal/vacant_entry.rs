use super::in_place::InPlace;

/// A trait to represent a vacant entry of a collection.
///
/// this can be thought of as holding a `K`.
///
/// The idea is that you've done hard work of finding your place in the collection,
/// so inserting shouldn't be a huge penalty at this point.
pub trait VacantEntry<'a, K, V, I>: Sized
where
    I: InPlace<K, V, Vacant<'a> = Self> + ?Sized + 'a,
{
    /// Get a reference to the key an item will be inserted with.
    fn get_key(&self) -> &K;

    /// Consume self and return the contained key.
    fn into_key(self) -> K;

    /// insert the `val` using the owned key.
    ///
    /// if you need to chain a future deletion, use `insert_entry`
    fn insert(self, val: V) {
        self.insert_entry(val);
    }

    /// insert the `val` using the owned key, returning the occupied entry
    fn insert_entry(self, val: V) -> I::Occupied<'a>;
}
