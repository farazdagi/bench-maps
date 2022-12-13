use std::sync::Arc;
use super::Value;
use bustle::{Collection, CollectionHandle};
use std::hash::{Hash, BuildHasher};
use seize::Collector;
use flurry::HashMap;

const BATCH_SIZE: usize = 2000;

#[derive(Clone)]
pub struct FlurryTable<K: 'static, H: 'static>(Arc<HashMap<K, Value, H>>);

impl<K, H> Collection for FlurryTable<K, H>
    where
        K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
        H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(
            HashMap::with_capacity_and_hasher(capacity, H::default()).with_collector(
                Collector::new()
                    .epoch_frequency(None)
                    .batch_size(BATCH_SIZE),
            ),
        ))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for FlurryTable<K, H>
    where
        K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
        H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.pin().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.pin().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.pin().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0
            .pin()
            .compute_if_present(key, |_, v| Some(v + 1))
            .is_some()
    }
}