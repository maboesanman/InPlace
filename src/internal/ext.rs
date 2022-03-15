use super::{
    entry::Entry, in_place::InPlace, lazy_entry::LazyEntry, occupied_entry::OccupiedEntry,
    renewable::RenewableVacantEntry, vacant_entry::VacantEntry,
};

pub trait InPlaceExt<K: Eq, V>: InPlace<K, V> {
    fn get_lazy_entry<'a>(&'a mut self, k: K) -> LazyEntry<'a, K, V, Self>;

    fn insert_entry<'a>(&'a mut self, k: K, v: V) -> (Self::Occupied<'a>, Option<V>);

    fn remove_entry<'a, Q>(&'a mut self, k: &K) -> (Self::Vacant<'a>, Option<V>)
    where
        K: Clone;
}

impl<K: Eq, V, T: InPlace<K, V>> InPlaceExt<K, V> for T {
    fn get_lazy_entry<'a>(&'a mut self, k: K) -> LazyEntry<'a, K, V, Self> {
        LazyEntry::ContainerRef(self, k)
    }

    fn insert_entry<'a>(&'a mut self, k: K, v: V) -> (Self::Occupied<'a>, Option<V>) {
        self.get_entry(k).insert_entry(v)
    }

    fn remove_entry<'a, Q>(&'a mut self, k: &K) -> (Self::Vacant<'a>, Option<V>)
    where
        K: Clone,
    {
        self.get_entry_clone_key(k).remove_entry()
    }
}

pub trait OccupiedEntryExt<'a, K, V, I>: OccupiedEntry<'a, K, V, I> + Sized
where
    K: Eq,
    I: InPlace<K, V, Occupied<'a> = Self> + ?Sized + 'a,
{
    fn get<'b>(&'b self) -> &'b V
    where
        K: 'b;
    fn get_mut<'b>(&'b mut self) -> &'b mut V
    where
        K: 'b;
    fn replace_value(&mut self, val: V) -> V;
}

impl<'a, K, V, I, T> OccupiedEntryExt<'a, K, V, I> for T
where
    K: Eq,
    I: InPlace<K, V, Occupied<'a> = Self> + ?Sized + 'a,
    T: OccupiedEntry<'a, K, V, I>,
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

pub trait RenewableVacantEntryExt<'a, K, V, I>: VacantEntry<'a, K, V, I> + Sized
where
    K: Eq,
    I: InPlace<K, V, Vacant<'a> = Self> + ?Sized + 'a,
{
    fn get_new_entry<Q>(self, k: Q) -> Entry<'a, K, V, I>
    where
        Q: ToOwned<Owned = K>;
}

impl<'a, K, V, I, T> RenewableVacantEntryExt<'a, K, V, I> for T
where
    K: Eq,
    I: InPlace<K, V, Vacant<'a> = Self> + ?Sized + 'a,
    T: RenewableVacantEntry<'a, K, V, I>,
{
    fn get_new_entry<Q>(self, k: Q) -> Entry<'a, K, V, I>
    where
        Q: ToOwned<Owned = K>,
    {
        self.get_new_entry_old_key(k).0
    }
}
