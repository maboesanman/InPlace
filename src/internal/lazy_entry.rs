use core::borrow::Borrow;

use super::{entry::Entry, in_place::InPlace};

pub enum LazyEntry<'a, K, V, I>
where
    K: Eq,
    I: InPlace<K, V> + ?Sized,
{
    ContainerRef(&'a mut I, K),
    Entry(Entry<'a, K, V, I>),
    Poisioned,
}

impl<'a, K, V, I> LazyEntry<'a, K, V, I>
where
    K: Eq,
    I: InPlace<K, V> + ?Sized,
{
    pub fn get_mut<Q>(&mut self) -> &mut Entry<'a, K, V, I>
    where
        K: Borrow<Q>,
        Q: Clone + Eq + ?Sized,
    {
        match self {
            LazyEntry::ContainerRef(_, _) => {
                if let LazyEntry::ContainerRef(container, k) =
                    core::mem::replace(self, LazyEntry::Poisioned)
                {
                    let e = container.get_entry(k);
                    *self = LazyEntry::Entry(e);

                    if let LazyEntry::Entry(e) = self {
                        e
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            LazyEntry::Entry(e) => e,
            LazyEntry::Poisioned => unreachable!(),
        }
    }
}

impl<'a, K, V, I> From<Entry<'a, K, V, I>> for LazyEntry<'a, K, V, I>
where
    K: Eq,
    I: InPlace<K, V> + ?Sized,
{
    fn from(e: Entry<'a, K, V, I>) -> Self {
        LazyEntry::Entry(e)
    }
}

impl<'a, K, V, I> From<LazyEntry<'a, K, V, I>> for Entry<'a, K, V, I>
where
    K: Eq,
    I: InPlace<K, V> + ?Sized,
{
    fn from(e: LazyEntry<'a, K, V, I>) -> Self {
        match e {
            LazyEntry::ContainerRef(container, k) => container.get_entry(k),
            LazyEntry::Entry(e) => e,
            LazyEntry::Poisioned => unreachable!(),
        }
    }
}
