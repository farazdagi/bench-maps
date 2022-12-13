use structopt::StructOpt;
use crate::cmd::workloads::{self, WorkloadKind};
use bustle::{Collection, CollectionHandle, Measurement};
use std::{fmt::Debug, thread::sleep, time::Duration};
use std::collections::hash_map::RandomState;
use crate::maps::RwLockStdHashMapTable;
use num_cpus;
use crossbeam_epoch;
use fxhash::{FxBuildHasher};

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long)]
    pub workload: WorkloadKind,
    #[structopt(short, long, default_value = "1")]
    pub operations: f64,
    #[structopt(long)]
    pub threads: Option<Vec<u32>>,
    #[structopt(long)]
    pub collect_csv: bool,
}

pub fn run(options: &Options) {
    println!("options: {:?}", options);
    println!("== {:?}", options.workload);
    let h = &mut create_handler(options);

    bench::<RwLockStdHashMapTable<u64, RandomState>>("RWLock<StdHashMap>", options, h);
    bench::<RwLockStdHashMapTable<u64, FxBuildHasher>>("RWLock<FxHashMap>", options, h);
}

fn bench<C>(name: &str, options: &Options, handler: &mut Handler)
    where
        C: Collection,
        <C::Handle as CollectionHandle>::Key: Send + Debug,
{
    println!("-- {}", name);
    let threads = options.threads
        .as_ref()
        .cloned()
        .unwrap_or_else(|| (1..(num_cpus::get() * 3 / 2) as u32).collect());

    // let mut first_throughput = None;
    for n in &threads {
        let m = workloads::create(options, *n as usize).run_silently::<C>();
        handler(name, *n, &m);

        // if !options.complete_slow {
        //     let threshold = *first_throughput.get_or_insert(m.throughput) / 5.;
        //     if m.throughput <= threshold {
        //         println!("too long, skipped");
        //         break;
        //     }
        // }

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

type Handler = Box<dyn FnMut(&str, u32, &Measurement)>;

fn create_handler(options: &Options) -> Handler {
    let handler = if options.collect_csv {
        unimplemented!("not implemented")
    } else {
        Box::new(|_: &str, n, m: &Measurement| {
            eprintln!(
                "total_ops={}\tthreads={}\tspent={:.1?}\tlatency={:?}\tthroughput={:.0}op/s",
                m.total_ops, n, m.spent, m.latency, m.throughput,
            )
        }) as Handler
    };
    handler
}
