pub trait Registry<K, I> {
    fn register(&self, key: &K, item: &I) -> bool;
    fn get(&self, key: &K) -> Option<&I>;
}
