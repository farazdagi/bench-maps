pub use self::{
    std::RwLockStdHashMapTable,
    dashmap::DashMapTable,
    hashbrown::HashbrownHashMapTable,
    btreemap::RwLockBTreeMapTable,
    chashmap::CHashMapTable,
};

mod std;
mod dashmap;
mod hashbrown;
mod btreemap;
mod chashmap;

type Value = u32;