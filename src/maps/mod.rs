pub use self::{
    std::RwLockStdHashMapTable,
    dashmap::DashMapTable,
    hashbrown::HashbrownHashMapTable,
    btreemap::RwLockBTreeMapTable,
};

mod std;
mod dashmap;
mod hashbrown;
mod btreemap;

type Value = u32;