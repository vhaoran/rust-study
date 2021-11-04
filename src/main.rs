mod a_mod_cmn;
mod deadpool_test;
#[allow(unused_imports)]
#[allow(dead_code)]
// use std::ops::Add;
mod leetcode;
mod redis_test;
mod smartptr;

mod t01;
mod t02_str;
mod t03_map;
mod t04_vec;
mod t05_ownership;
mod t06_struct;
mod t07_sort;
mod t08_thread;
mod t09_struct;
mod t10_file;
mod t11_json;
mod t12_toml;
mod t13_async;
mod t14_log;
mod t15_yaml;
mod t17_trait;
mod t18_iron;
mod t19_generic;
mod t20_enum;
mod t21_other;
mod t22_err;
mod test1;
mod tokio_demo;

mod a;
mod aa;
mod b;
mod macro_test;


#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: Option<bool>,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long)]
    verbose: Option<u8>,

    /// Set speed
    // #[structopt(short, long, default_value = "42")]
    #[structopt(short, long)]
    speed: Option<f64>,

    /// Output file
    // #[structopt(short, long, parse(from_os_str))]
    #[structopt(short, long)]
    output: Option<PathBuf>,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[structopt(short, long)]
    level: Vec<String>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

// use std::thread::{park_timeout, sleep};
fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
