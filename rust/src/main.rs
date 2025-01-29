#[macro_use]
extern crate lazy_static;

use clap::{Parser, Subcommand};

mod generate;
mod solver;
mod vector3d;

#[derive(Parser, Clone, Debug)]
pub struct Solve {
    /// A string of characters describing the cube, C meaning CURVE, S meaning straight. Start and end can be ommitted and are discarded if len(chain) === x^3
    chain: String,
    
    /// Print Performance Information on STDERR
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Generate(generate::Generate),
    Solve(Solve),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Command::Generate(g) => generate::generate(g.clone()),
        Command::Solve(s) => solver::solve(s.clone()),
    }
}
