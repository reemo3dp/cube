use rand::prelude::*;
use rand_seeder::Seeder;
use rand_xorshift::XorShiftRng;
use std::{ops::Deref, time::Instant};

mod dynamic_programming;
mod randomizer;
mod randomizer_easier;
mod super_random;

type Coord = [u8; 3];

static mut NUM_TRIED: u128 = 0;
static PRINT_EVERY: u128 = 1000000;

const VALID_NEIGHBOURS: [[i8; 3]; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

struct Cube {
    dim: u8,
    seed: <XorShiftRng as rand::SeedableRng>::Seed,
    path: Vec<Coord>,
}

fn get_neighbours(coord: Coord, dim: u8) -> Vec<Coord> {
    let mut neighbours = Vec::with_capacity(6); // Pre-allocate for max possible neighbours
    let [x, y, z] = coord;

    // Since we know x, y, z are u8, we can do cheaper bounds checking
    for &[dx, dy, dz] in crate::VALID_NEIGHBOURS.iter() {
        // Handle negative results first
        if (dx < 0 && x == 0) || (dy < 0 && y == 0) || (dz < 0 && z == 0) {
            continue;
        }

        // Safe unsigned arithmetic for positive offsets
        let nx = if dx < 0 {
            x - (-dx as u8)
        } else {
            x + (dx as u8)
        };
        let ny = if dy < 0 {
            y - (-dy as u8)
        } else {
            y + (dy as u8)
        };
        let nz = if dz < 0 {
            z - (-dz as u8)
        } else {
            z + (dz as u8)
        };

        // Check upper bounds
        if nx < dim && ny < dim && nz < dim {
            neighbours.push([nx, ny, nz]);
        }
    }

    neighbours
}

fn main() {
    let start = Instant::now();

    let seed: <XorShiftRng as SeedableRng>::Seed = std::env::var("SEED").map_or_else(
        |_| {
            let mut seed: <XorShiftRng as SeedableRng>::Seed = Default::default();
            thread_rng().fill(&mut seed);
            seed
        },
        |v| Seeder::from(v).make_seed(),
    );

    let dim = std::env::args()
        .nth(1)
        .and_then(|v| v.parse().ok())
        .unwrap();

    if let Some(cube) = randomizer_easier::create_cube(seed, dim, start) {
        println!("// Seed: {:?}", cube.seed);
        println!("DIM = {};", cube.dim);
        println!("PATH = {:?};", cube.path);
    } else {
        println!("No solution found!");
    }
}
