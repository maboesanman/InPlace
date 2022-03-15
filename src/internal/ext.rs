use super::{entry::Entry, lazy_entry::LazyEntry, in_place::InPlace, occupied_entry::OccupiedEntry, vacant_entry::VacantEntry};

pub trait InPlaceExt<K, V>: InPlace<K, V> {
    fn get_lazy_entry<'a, 'q, Q>(&'a mut self, k: Q) -> LazyEntry<'a, K, V, Self, Q>
    where
        Q: ToOwned<Owned = K>;

    fn insert_entry<'a>(&'a mut self, k: K, v: V) -> (Self::Occupied<'a>, Option<V>)
    where
        K: ToOwned<Owned = K>;

    fn remove_entry<'a, 'q, Q>(&'a mut self, k: Q) -> (Self::Vacant<'a>, Option<V>)
    where
        Q: ToOwned<Owned = K>;
}

impl<K, V, T: InPlace<K, V>> InPlaceExt<K, V> for T {
    fn get_lazy_entry<'a, 'q, Q>(&'a mut self, k: Q) -> LazyEntry<'a, K, V, Self, Q>
    where
        Q: ToOwned<Owned = K>,
    {
        LazyEntry::ContainerRef(self, k)
    }

    fn insert_entry<'a>(&'a mut self, k: K, v: V) -> (Self::Occupied<'a>, Option<V>)
    where
        K: ToOwned<Owned = K>
    {
        self.get_entry(k).insert_entry(v)
    }

    fn remove_entry<'a, 'q, Q>(&'a mut self, k: Q) -> (Self::Vacant<'a>, Option<V>)
    where
        Q: ToOwned<Owned = K>
    {
        self.get_entry(k).remove_entry()
    }
}

pub trait OccupiedEntryExt<'a, K, V, I: InPlace<K, V, Occupied<'a> = Self> + ?Sized + 'a>:
    OccupiedEntry<'a, K, V, I> + Sized
{
    fn get<'b>(&'b self) -> &'b V
    where
        K: 'b;
    fn get_mut<'b>(&'b mut self) -> &'b mut V
    where
        K: 'b;
    fn replace_value(&mut self, val: V) -> V;
}

impl<'a, K, V, I: InPlace<K, V, Occupied<'a> = Self> + ?Sized + 'a, T: OccupiedEntry<'a, K, V, I>>
    OccupiedEntryExt<'a, K, V, I> for T
{
    fn get<'b>(&'b self) -> &'b V
    where
        K: 'b,
    {
        self.get_pair().1
    }

    fn get_mut<'b>(&'b mut self) -> &'b mut V
    where
        K: 'b,
    {
        self.get_pair_mut().1
    }

    fn replace_value(&mut self, val: V) -> V {
        std::mem::replace(self.get_pair_mut().1, val)
    }
}

pub trait VacantEntryExt<'a, K, V, I: InPlace<K, V, Vacant<'a> = Self> + ?Sized + 'a>:
    VacantEntry<'a, K, V, I> + Sized
{
    fn get_new_entry<Q>(self, k: Q) -> Entry<'a, K, V, I>
    where
        Q: ToOwned<Owned = K>;
}

impl<'a, K, V, I: InPlace<K, V, Vacant<'a> = Self> + ?Sized + 'a, T: VacantEntry<'a, K, V, I>>
    VacantEntryExt<'a, K, V, I> for T
{
    fn get_new_entry<Q>(self, k: Q) -> Entry<'a, K, V, I>
    where
        Q: ToOwned<Owned = K>,
    {
        self.get_new_entry_old_key(k).0
    }
}
