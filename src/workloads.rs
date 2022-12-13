use crate::bench::Options;
use std::str::FromStr;
use bustle::{Workload, Mix};

#[derive(Debug)]
pub enum WorkloadKind {
    ReadHeavy,
    WriteHeavy,
    Mixed,
}

impl FromStr for WorkloadKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ReadHeavy" => Ok(Self::ReadHeavy),
            "WriteHeavy" => Ok(Self::WriteHeavy),
            "Mixed" => Ok(Self::Mixed),
            _ => Err("invalid workload kind"),
        }
    }
}

pub(crate) fn create(options: &Options, threads: usize) -> Workload {
    let mut workload = match options.workload {
        WorkloadKind::ReadHeavy => {
            let mix = Mix {
                read: 98,
                insert: 1,
                remove: 1,
                update: 0,
                upsert: 0,
            };
            *Workload::new(threads, mix)
                .initial_capacity_log2(10)
                .prefill_fraction(0.8)
        }
        WorkloadKind::WriteHeavy => {
            let mix = Mix {
                read: 5,
                insert: 80,
                remove: 5,
                update: 10,
                upsert: 0,
            };
            *Workload::new(threads, mix)
                .prefill_fraction(0.0)
        }
        WorkloadKind::Mixed => {
            let mix = Mix {
                read: 10,
                insert: 40,
                remove: 40,
                update: 10,
                upsert: 0,
            };
            *Workload::new(threads, mix)
                .prefill_fraction(0.8)
        }
    };
    workload.operations(options.operations);
    workload
}
