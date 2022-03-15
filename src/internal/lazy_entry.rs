use super::{entry::Entry, in_place::InPlace};

pub enum LazyEntry<'a, K, V, I, Q>
where
    I: InPlace<K, V> + ?Sized,
    Q: ToOwned<Owned = K>,
{
    ContainerRef(&'a mut I, Q),
    Entry(Entry<'a, K, V, I>),
    Poisioned,
}

impl<'a, K, V, I, Q> LazyEntry<'a, K, V, I, Q>
where
    I: InPlace<K, V> + ?Sized,
    Q: ToOwned<Owned = K>,
{
    pub fn get_mut(&mut self) -> &mut Entry<'a, K, V, I> {
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
            LazyEntry::Poisioned => unreachable!(),
        }
    }
}
