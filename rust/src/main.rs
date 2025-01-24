#[macro_use]
extern crate lazy_static;

use algorithm::Algorithm;
use clap::{builder::PossibleValue, Parser, Subcommand, ValueEnum};
use common::Coord;
use crossbeam_channel::{unbounded, Sender};
use dynamic_programming::DynamicProgramming;
use lazy_static::initialize;
use rand::{distributions::Alphanumeric, prelude::*};
use rand_seeder::Seeder;
use rand_xorshift::XorShiftRng;
use randomizer::Randomizer;
use randomizer_easier::RandomizerEasier;
use serde::Serialize;
use std::{
    fmt, io::{Cursor, Write}, string, sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    }, thread::{available_parallelism, sleep, spawn}, time::{Duration, Instant}
};
use super_random::SuperRandom;

mod solver;
mod algorithm;
mod common;
mod dynamic_programming;
mod randomizer;
mod randomizer_easier;
mod super_random;

lazy_static! {
    static ref STARTED_AT: Instant = Instant::now();
    static ref NUM_SOLUTIONS_TRIED: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    static ref PRINT_DEBUG_EVERY_N_FAILURES: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    static ref SHOULD_STOP: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

#[derive(clap::ValueEnum, Clone, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum AlgorithmArg {
    RandomizerEasier,
    Randomizer,
    SuperRandom,
    DynamicProgramming,
}

#[derive(Parser, Clone, Debug)]
pub struct Generate {
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

#[derive(Parser, Clone, Debug)]
pub struct Solve {
    /// A string of characters describing the cube, C meaning CURVE, S meaning straight. Start and end can be ommitted and are discarded if len(chain) === x^3
    chain: String,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Generate(Generate),
    Solve(Solve)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}


fn main() {
    initialize(&STARTED_AT);
    let args = Cli::parse();

    match &args.command {
        Command::Generate(g) => generate(g.clone()),
        Command::Solve(s) => solver::solve(s.clone()),
    }

   
}


fn generate(g: Generate) {
    if g.verbose {
        PRINT_DEBUG_EVERY_N_FAILURES.store(1000000, std::sync::atomic::Ordering::Relaxed);
    }

    let (s, r) = unbounded::<Result>();

    let num_of_threads = available_parallelism().unwrap().into();
    if g.verbose {
        eprintln!("//D Launching {} threads", num_of_threads);
    }
    for _ in 0..num_of_threads {
        let cp = g.clone();
        let sender = s.clone();
        spawn(move || {
            run(cp, sender);
        });
    }

    loop {
        match r.recv() {
            Ok(r) => {
                let _ = std::io::stderr().flush();
                println!("{}", r);
                if !g.r#loop {
                    return;
                }
            }
            Err(x) => eprintln!("//D {:?}", x),
        }
    }
}

#[derive(Clone, Debug)]
struct Result {
    seed: String,
    algorithm: PossibleValue,
    elapsed: Duration,
    dim: u8,
    path: Vec<Coord>,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "// Seed: {}", self.seed)?;
        writeln!(f, "// Randomizer: {}", self.algorithm.get_name())?;
        writeln!(f, "// Duration: {:?}", self.elapsed)?;
        writeln!(f, "DIM = {};", self.dim)?;
        writeln!(f, "PATH = {:?};", self.path)
    }
}

fn run(g: Generate, sender: Sender<Result>) {
    let algo: Box<dyn Algorithm + Send + Sync> = match g.algorithm {
        AlgorithmArg::RandomizerEasier => Box::new(RandomizerEasier {}),
        AlgorithmArg::Randomizer => Box::new(Randomizer {}),
        AlgorithmArg::SuperRandom => Box::new(SuperRandom {}),
        AlgorithmArg::DynamicProgramming => Box::new(DynamicProgramming {}),
    };
    sleep(Duration::from_secs(thread_rng().gen_range(1..5)));

    let mut last_start = *STARTED_AT;
    loop {
        let seed_string: String = g.seed.clone().unwrap_or_else(|| {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect()
        });

        let seed: <XorShiftRng as SeedableRng>::Seed =
            Seeder::from(seed_string.clone()).make_seed();
        if let Some(cube) = algo.run(seed, g.dim) {
            match sender.send(Result {
                seed: seed_string.clone(),
                algorithm: g.algorithm.to_possible_value().unwrap(),
                dim: g.dim,
                elapsed: last_start.elapsed(),
                path: cube.clone(),
            }) {
                Ok(_) => {}
                Err(_) => return,
            }
            if !g.r#loop {
                break;
            };
            last_start = Instant::now();
        }
    }
}
