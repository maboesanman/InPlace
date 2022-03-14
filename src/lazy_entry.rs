use crate::{Entry, InPlace};

pub enum LazyEntry<'a, K, V, I, Q>
where
    I: InPlace<K, V> + ?Sized,
    Q: ToOwned<Owned = K>,
{
    ContainerRef(&'a mut I, Q),
    Entry(Entry<'a, K, V, I>),
}

impl<'a, K, V, I, Q> From<Entry<'a, K, V, I>> for LazyEntry<'a, K, V, I, Q>
where
    I: InPlace<K, V> + ?Sized,
    Q: ToOwned<Owned = K>,
{
    fn from(e: Entry<'a, K, V, I>) -> Self {
        LazyEntry::Entry(e)
    }
}

impl<'a, K, V, I, Q> From<LazyEntry<'a, K, V, I, Q>> for Entry<'a, K, V, I>
where
    I: InPlace<K, V> + ?Sized,
    Q: ToOwned<Owned = K>,
{
    fn from(e: LazyEntry<'a, K, V, I, Q>) -> Self {
        match e {
            LazyEntry::ContainerRef(container, k) => container.get_entry(k),
            LazyEntry::Entry(e) => e,
        }
    }
}
