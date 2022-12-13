pub use self::{
    std::RwLockStdHashMapTable,
    dashmap::DashMapTable,
    hashbrown::HashbrownHashMapTable,
};

mod std;
mod dashmap;
mod hashbrown;

type Value = u32;