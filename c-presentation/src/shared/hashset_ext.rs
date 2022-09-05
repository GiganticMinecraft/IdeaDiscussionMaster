use std::{collections::HashSet, hash::Hash};

pub trait HashSetExt<T> {
    fn update(&mut self, old: &T, new: T);
}

impl<T> HashSetExt<T> for HashSet<T>
where
    T: Eq + PartialEq + Hash,
{
    fn update(&mut self, old: &T, new: T) {
        self.remove(old);
        self.insert(new);
    }
}
