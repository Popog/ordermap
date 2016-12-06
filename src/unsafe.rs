
#[cfg(feature="test_unsafe_index")]
macro_rules! i {
    ($e:ident$(.$e2:ident)*[$i:expr]) => {*unsafe { $e$(.$e2)*.get_unchecked($i) }};
    (&$e:ident$(.$e2:ident)*[$i:expr]) => {unsafe { $e$(.$e2)*.get_unchecked($i) }};
}

#[cfg(feature="test_unsafe_index")]
macro_rules! im {
    ($e:ident$(.$e2:ident)*[$i:expr]) => {*unsafe { $e$(.$e2)*.get_unchecked_mut($i) }};
    (&mut $e:ident$(.$e2:ident)*[$i:expr]) => {unsafe { $e$(.$e2)*.get_unchecked_mut($i) }};
}


#[cfg(feature="test_efficient_enum")]
use efficient_enum::tag::TagMSB;
#[cfg(feature="test_efficient_enum")]
use efficient_enum::option::{EfficientOption, EfficientOptionInnerSome};


impl HashValue {
    #[cfg(feature="test_efficient_enum")]
    fn new(v: usize) -> HashValue {
        HashValue(v & (usize::max_value() >> 1))
    }
}


#[cfg(feature="test_efficient_enum")]
#[derive(Copy,Clone,Debug)]
struct Bucket<K, V> {
    option: EfficientOption<usize, (K, V), TagMSB>,
}

/// A type which can take values from a Bucket, leaving the bucket empty
#[cfg(feature="test_efficient_enum")]
struct BucketTaker<'a, K: 'a, V: 'a>(EfficientOptionInnerSome<'a, usize, (K, V), TagMSB>);


#[cfg(feature="test_efficient_enum")]
impl<K, V> Bucket<K, V> {
    fn new(hash: HashValue, key: K, value: V) -> Self {
        Bucket { option: EfficientOption::some((hash.0, (key, value))) }
    }

    fn unwrap_hash_key(&self) -> (HashValue, &K) {
        debug_assert!(self.option.is_some());
        let hash = self.option.clone_0().unwrap();
        (HashValue(hash), &self.option.ref_1().unwrap().0)
    }

    fn hash(&self) -> Option<HashValue> {
        self.option.clone_0().map(HashValue)
    }

    fn kv(&self) -> Option<(&K, &V)> {
        self.option.ref_1().map(|e| (&e.0, &e.1))
    }

    fn kv_mut(&mut self) -> Option<(&mut K, &mut V)> {
        self.option.mut_1().map(|e| (&mut e.0, &mut e.1))
    }

    fn taker<'a>(&'a mut self) -> Option<BucketTaker<'a, K, V>> {
        use efficient_enum::option::EfficientOptionInner::IsSome;
        if let IsSome(e) = self.option.inner() {
            Some(BucketTaker(e))
        } else { None }
    }

    fn take(&mut self) -> Option<(K, V)> {
        self.option.take_1()
    }

    fn into_kv(self) -> Option<(K, V)> {
        debug_assert!(self.option.is_some());
        self.option.destructure_1()
    }
}

#[cfg(all(feature="test_efficient_enum", not(feature="test_unsafe_efficient_enum")))]
impl<K, V> Bucket<K, V> {
    fn unwrap_hash(&self) -> HashValue {
        debug_assert!(self.option.is_some());
        HashValue(self.option.clone_0().unwrap())
    }

    fn unwrap_kv(&self) -> (&K, &V) {
        debug_assert!(self.option.is_some());
        self.kv().unwrap()
    }

    fn unwrap_kv_mut(&mut self) -> (&mut K, &mut V) {
        debug_assert!(self.option.is_some());
        self.kv_mut().unwrap()
    }

    fn unwrap_taker<'a>(&'a mut self) -> BucketTaker<'a, K, V> {
        debug_assert!(self.option.is_some());
        self.taker().unwrap()
    }

    fn unwrap_into_kv(self) -> (K, V) {
        debug_assert!(self.option.is_some());
        self.into_kv().unwrap()
    }
}

#[cfg(feature="test_efficient_enum")]
impl<'a, K, V> BucketTaker<'a, K, V> {
    fn hash(&self) -> HashValue {
        HashValue(self.0.clone_0())
    }
    fn key(&self) -> &K {
        &self.0.ref_1().0
    }
    fn value(&self) -> &V {
        &self.0.ref_1().1
    }
    fn value_mut(&mut self) -> &mut V {
        &mut self.0.mut_1().1
    }
    fn into_value_mut(self) -> &'a mut V {
        &mut self.0.destructure_1().1
    }
    #[allow(dead_code)]
    fn kv_mut(&mut self) -> (&mut K, &mut V) {
        let e = self.0.mut_1();
        (&mut e.0, &mut e.1)
    }
    fn take(self) -> (K, V) {
        self.0.take_1().1
    }
}

#[cfg(feature="test_unsafe_efficient_enum")]
impl<K, V> Bucket<K, V> {
    fn unwrap_hash(&self) -> HashValue {
        debug_assert!(self.option.is_some());
        unsafe { HashValue(*self.option.unwrap_ref_0()) }
    }

    fn unwrap_kv(&self) -> (&K, &V) {
        debug_assert!(self.option.is_some());
        let kv = unsafe { self.option.unwrap_ref_1() };
        (&kv.0, &kv.1)
    }

    fn unwrap_kv_mut(&mut self) -> (&mut K, &mut V) {
        debug_assert!(self.option.is_some());
        let kv = unsafe { self.option.unwrap_mut_1() };
        (&mut kv.0, &mut kv.1)
    }

    fn unwrap_taker<'a>(&'a mut self) -> BucketTaker<'a, K, V> {
        debug_assert!(self.option.is_some());
        unsafe { BucketTaker(self.option.inner_some()) }
    }

    fn unwrap_into_kv(self) -> (K, V) {
        debug_assert!(self.option.is_some());
        unsafe { self.option.unwrap_destructure_1() }
    }
}
