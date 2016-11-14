
#[macro_use] extern crate ordermap;

use ordermap::{Lookup};

use std::hash::{Hash};

#[derive(Debug, Hash)]
pub struct Pair<A, B>(pub A, pub B);

impl<A, B, C, D> PartialEq<(A, B)> for Pair<C, D>
    where C: PartialEq<A>,
          D: PartialEq<B>,
{
    fn eq(&self, rhs: &(A, B)) -> bool {
        self.0 == rhs.0 &&
        self.1 == rhs.1 &&
        true
    }
}

impl<A, B, X: ?Sized> Lookup<X> for Pair<A, B>
    where Pair<A, B>: PartialEq<X>,
          A: Hash + Eq,
          B: Hash + Eq,
{
    fn equal(&self, other: &X) -> bool {
        *self == *other
    }
}

#[test]
fn test_lookup() {
    let map = ordermap! {
        (String::from("a"), String::from("b")) => 1,
        (String::from("a"), String::from("x")) => 2,
    };

    assert!(map.contains_key(&Pair("a", "b")));
    assert!(!map.contains_key(&Pair("b", "a")));
}
