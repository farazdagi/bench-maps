use structopt::StructOpt;
use crate::workloads::{self, WorkloadKind};
use bustle::{Collection, CollectionHandle, Measurement};
use std::{fmt::Debug, thread::sleep, time::Duration};
use std::collections::hash_map::RandomState;
use ahash::RandomState as AhashRandomState;
use crate::maps::*;
use crossbeam_epoch;
use fxhash::{FxBuildHasher};
use thousands::Separable;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long)]
    pub workload: WorkloadKind,
    #[structopt(short, long, default_value = "1")]
    pub operations: f64,
}

pub fn run(options: &Options) {
    println!("options: {:?}", options);
    println!("== {:?}", options.workload);

    let h = &mut create_handler();

    bench::<HashbrownHashMapTable<u64, RandomState>>("HashbrownHashMap - StdHasher", options, h);
    bench::<HashbrownHashMapTable<u64, AhashRandomState>>("HashbrownHashMap - AhashHasher", options, h);
    bench::<RwLockStdHashMapTable<u64, RandomState>>("RWLock<HashMap> - StdHasher", options, h);
    bench::<RwLockStdHashMapTable<u64, FxBuildHasher>>("RWLock<HashMap> - FxHasher", options, h);
    bench::<RwLockStdHashMapTable<u64, AhashRandomState>>("RWLock<HashMap> - AhashHasher", options, h);
    bench::<DashMapTable<u64, RandomState>>("DashMap - StdHasher", options, h);
    bench::<DashMapTable<u64, FxBuildHasher>>("DashMap - FxHasher", options, h);
    bench::<DashMapTable<u64, AhashRandomState>>("DashMap - AhashHasher", options, h);
}

fn bench<C>(name: &str, options: &Options, handler: &mut Handler)
    where
        C: Collection,
        <C::Handle as CollectionHandle>::Key: Send + Debug,
{
    println!("-- {}", name);
    let threads = [1, 2, 4, 8, 16, 32];
    for n in threads {
        let m = workloads::create(options, n as usize).run_silently::<C>();
        handler(n, &m);
        gc_cycle();
    }
    println!();
}

fn gc_cycle() {
    sleep(Duration::from_millis(2000));
    let mut new_guard = crossbeam_epoch::pin();
    new_guard.flush();
    for _ in 0..32 {
        new_guard.repin();
    }
}

type Handler = Box<dyn FnMut(u32, &Measurement)>;

fn create_handler() -> Handler {
    Box::new(|n, m: &Measurement| {
        eprintln!(
            "\tthreads={}\ttotal_ops={}\tspent={:.1?}\tlatency={:.2?}\tthroughput={} op/s",
            n, m.total_ops.separate_with_commas(), m.spent, m.latency, m.throughput.floor().separate_with_commas(),
        )
    }) as Handler
}
