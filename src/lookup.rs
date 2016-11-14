
use std::hash::Hash;
use std::borrow::Borrow;

/// Key lookup trait.
///
/// This trait allows hash table lookup to be customized.
/// It has one blanket implementation that uses the regular `Borrow` solution,
/// just like `HashMap` and `BTreeMap` do, so that you can pass `&str` to lookup
/// into a map with `String` keys and so on.
///
/// # Contract
///
/// The implementor must hash like `K`.
pub trait Lookup<K: ?Sized> : Hash {
    /// Compare self to `key` and return `true` if they are equal.
    fn equal(&self, key: &K) -> bool;
}

impl<Q: ?Sized, K: ?Sized> Lookup<K> for Q
    where Q: Eq + Hash,
          K: Borrow<Q>,
{
    #[inline]
    fn equal(&self, key: &K) -> bool {
        *self == *key.borrow()
    }
}
