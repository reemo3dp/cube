#[macro_use]
extern crate lazy_static;

use algorithm::Algorithm;
use clap::{Parser, ValueEnum};
use dynamic_programming::DynamicProgramming;
use lazy_static::initialize;
use rand::{distributions::Alphanumeric, prelude::*};
use rand_seeder::Seeder;
use rand_xorshift::XorShiftRng;
use randomizer::Randomizer;
use randomizer_easier::RandomizerEasier;
use serde::Serialize;
use std::time::Instant;
use super_random::SuperRandom;

mod algorithm;
mod common;
mod dynamic_programming;
mod randomizer;
mod randomizer_easier;
mod super_random;

lazy_static! {
    static ref STARTED: Instant = Instant::now();
}
static mut NUM_TRIED: u128 = 0;
static mut PRINT_EVERY: u128 = 0;

#[derive(clap::ValueEnum, Clone, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum AlgorithmArg {
    RandomizerEasier,
    Randomizer,
    SuperRandom,
    DynamicProgramming,
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The algorithm to use for finding a solution
    #[arg(short, long, value_enum, default_value_t = AlgorithmArg::RandomizerEasier)]
    algorithm: AlgorithmArg,

    /// Keep the program running or terminate after the first match
    #[arg(short, long, default_value_t = false, group = "looping")]
    r#loop: bool,

    /// Print Performance Information on STDERR
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Seed
    #[arg(short, long, group = "looping")]
    seed: Option<String>,

    /// Side length of the cube
    dim: u8,
}

fn main() {
    initialize(&STARTED);
    let args = Args::parse();

    if args.verbose {
        unsafe {
            PRINT_EVERY = 1000000;
        }
    }

    let algo: Box<dyn Algorithm> = match args.algorithm {
        AlgorithmArg::RandomizerEasier => Box::new(RandomizerEasier {}),
        AlgorithmArg::Randomizer => Box::new(Randomizer {}),
        AlgorithmArg::SuperRandom => Box::new(SuperRandom {}),
        AlgorithmArg::DynamicProgramming => Box::new(DynamicProgramming {}),
    };

    let mut last_start = *STARTED;
    loop {
        let seed_string: String = args.seed.clone().unwrap_or_else(|| {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect()
        });

        let seed: <XorShiftRng as SeedableRng>::Seed =
            Seeder::from(seed_string.clone()).make_seed();
        if let Some(cube) = algo.run(seed, args.dim) {
            println!("// Seed: {}", seed_string);
            println!("// Randomizer: {}", args.algorithm.to_possible_value().unwrap().get_name());
            println!("// Duration: {:?}", last_start.elapsed());
            println!("DIM = {};", cube.dim);
            println!("PATH = {:?};", cube.path);
            if !args.r#loop {
                break;
            };
            last_start = Instant::now();
        }
    }
}
