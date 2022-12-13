pub use self::{
    std::RwLockStdHashMapTable,
    dashmap::DashMapTable,
    hashbrown::HashbrownHashMapTable,
    btreemap::RwLockBTreeMapTable,
    chashmap::CHashMapTable,
    flurry::FlurryTable,
};

mod std;
mod dashmap;
mod hashbrown;
mod btreemap;
mod chashmap;
mod flurry;

type Value = u32;