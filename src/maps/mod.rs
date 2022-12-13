pub use self::{
    std::RwLockStdHashMapTable,
    dashmap::DashMapTable,
};

mod std;
mod dashmap;

type Value = u32;