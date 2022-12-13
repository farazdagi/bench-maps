use std::fmt::Debug;
use structopt::StructOpt;

mod cmd;
mod maps;


#[derive(Debug, StructOpt)]
enum Options {
    Bench(cmd::bench::Options)
}


fn main() {
    match Options::from_args() {
        Options::Bench(options) => cmd::bench::run(&options),
    }
}
