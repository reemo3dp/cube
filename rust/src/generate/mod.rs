mod algorithm;
mod common;
mod dynamic_programming;
mod randomizer;
mod randomizer_easier;
mod super_random;

use algorithm::Algorithm;
use clap::{builder::PossibleValue, Parser, ValueEnum};
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
    fmt,
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
    thread::{available_parallelism, sleep, spawn},
    time::{Duration, Instant},
};
use super_random::SuperRandom;

lazy_static! {
    pub static ref STARTED_AT: Instant = Instant::now();
    pub static ref NUM_SOLUTIONS_TRIED: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    pub static ref PRINT_DEBUG_EVERY_N_FAILURES: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    pub static ref SHOULD_STOP: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
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

pub fn generate(g: Generate) {
    initialize(&STARTED_AT);
    if g.verbose {
        PRINT_DEBUG_EVERY_N_FAILURES.store(1000000, std::sync::atomic::Ordering::Relaxed);
    }

    let (s, r) = unbounded::<Result>();

    let num_of_threads = match g.seed {
        Some(_) => 1,
        _ => available_parallelism().unwrap().into(),
    };
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

fn sub_u3(a: [u8; 3], b: [u8; 3]) -> [i16; 3] {
    let a_i: [i16; 3] = [a[0].into(), a[1].into(), a[2].into()];
    let b_i: [i16; 3] = [b[0].into(), b[1].into(), b[2].into()];

    [a_i[0] - b_i[0], a_i[1] - b_i[1], a_i[2] - b_i[2]]
}

fn mult_u3(a: [i16; 3], b: i16) -> [i16; 3] {
    [a[0] * b, a[1] * b, a[2] * b]
}

impl Result {
    fn format_path(&self) -> String {
        let mut result = String::from("1_A_START ");
        let mut dir_in = mult_u3(sub_u3(self.path[1], self.path[0]), -1);

        for n in 1..self.path.len() - 1 {
            let dir_out = sub_u3(self.path[n + 1], self.path[n]);
            let t = if mult_u3(dir_out, 1) == mult_u3(dir_in, -1) {
                "S"
            } else {
                "C"
            };

            let color = if (n + 1) % 2 == 1 { "A" } else { "B" };

            result.push_str(&format!("{}_{}_{} ", n + 1, color, t).to_string());
            dir_in = mult_u3(dir_out, -1);
        }
        let color = if (self.path.len() + 1) % 2 == 0 {
            "A"
        } else {
            "B"
        };
        result.push_str(&format!("{}_{}_{}", self.path.len(), color, "END").to_string());
        result
    }
    fn format_path_quick(&self) -> String {
        let mut result = String::from("");
        let mut dir_in = mult_u3(sub_u3(self.path[1], self.path[0]), -1);

        for n in 1..self.path.len() - 1 {
            let dir_out = sub_u3(self.path[n + 1], self.path[n]);
            let t = if mult_u3(dir_out, 1) == mult_u3(dir_in, -1) {
                "S"
            } else {
                "C"
            };
            result.push_str(t);
            dir_in = mult_u3(dir_out, -1);
        }
        result
    }
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "// Seed: {}", self.seed)?;
        writeln!(f, "// Randomizer: {}", self.algorithm.get_name())?;
        writeln!(f, "// Duration: {:?}", self.elapsed)?;
        writeln!(f, "// Assembly: {}", self.format_path())?;
        writeln!(f, "// Quick: {}", self.format_path_quick())?;
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
