use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::time::Instant;

mod dynamic_programming;
mod randomizer;
mod super_random;


type Coord = [u32; 3];

static mut NUM_TRIED: u128 = 0;
static PRINT_EVERY: u128 = 1000000;

const VALID_NEIGHBOURS: [[i32; 3]; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

struct Cube {
    dim: u32,
    seed: <XorShiftRng as rand::SeedableRng>::Seed,
    path: Vec<Coord>,
}

fn get_neighbours(coord: Coord, dim: u32) -> Vec<Coord> {
    let [x, y, z] = coord;

    crate::VALID_NEIGHBOURS
        .iter()
        .flat_map(|[dx, dy, dz]| {
            let nx: i32 = <u32 as TryInto<i32>>::try_into(x).unwrap() + dx;
            let ny: i32 = <u32 as TryInto<i32>>::try_into(y).unwrap() + dy;
            let nz: i32 = <u32 as TryInto<i32>>::try_into(z).unwrap() + dz;

            let dimu: i32 = dim.try_into().unwrap();

            if nx < dimu && ny < dimu && nz < dimu && nx >= 0 && ny >= 0 && nz >= 0 {
                return Some([
                    nx.try_into().unwrap(),
                    ny.try_into().unwrap(),
                    nz.try_into().unwrap(),
                ]);
            }
            None
        })
        .collect()
}

fn main() {
    let start = Instant::now();

    let mut seed: <XorShiftRng as SeedableRng>::Seed = Default::default();
    thread_rng().fill(&mut seed);
    let dim = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    println!("// Seed: {:?}", seed);

    if let Some(cube) = randomizer::create_cube(seed, dim, start) {
        println!("DIM = {};", cube.dim);
        println!("PATH = {:?};", cube.path);
    } else {
        println!("No solution found!");
    }
}
