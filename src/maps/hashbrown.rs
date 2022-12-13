use std::sync::Arc;
use super::Value;
use bustle::{Collection, CollectionHandle};
use std::hash::{Hash, BuildHasher};
use hashbrown::HashMap;
use parking_lot::RwLock;

#[derive(Clone)]
pub struct HashbrownHashMapTable<K, H>(Arc<RwLock<HashMap<K, Value, H>>>);

impl<K, H> Collection for HashbrownHashMapTable<K, H>
    where
        K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
        H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        ))))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for HashbrownHashMapTable<K, H>
    where
        K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
        H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let mut map = self.0.write();
        map.get_mut(key).map(|v| *v += 1).is_some()
    }
}