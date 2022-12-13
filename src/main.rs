use std::fmt::Debug;
use structopt::StructOpt;

mod maps;
mod bench;
mod workloads;


#[derive(Debug, StructOpt)]
enum Options {
    Bench(bench::Options)
}


fn main() {
    match Options::from_args() {
        Options::Bench(options) => bench::run(&options),
    }
}
