use crate::algorithm::Algorithm;
use crate::common::get_neighbours;
use crate::common::record_failure;
use crate::common::Coord;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_xorshift::XorShiftRng;

fn create_cube_rec(chain: Vec<Coord>, rng: &mut XorShiftRng, dim: u8) -> Option<Vec<Coord>> {
    if chain.len() == (dim * dim * dim).into() {
        return Some(chain);
    }
    let current = chain.last().unwrap();
    let mut neighbours = get_neighbours(*current, dim);
    neighbours.shuffle(rng);

    for neighbour in neighbours {
        if chain.contains(&neighbour) {
            record_failure(chain.len());
            continue;
        };

        let mut next_chain = chain.clone();
        next_chain.push(neighbour);
        let result = create_cube_rec(next_chain, rng, dim);
        if result.is_some() {
            return result;
        }
    }

    None
}

pub struct DynamicProgramming;
impl Algorithm for DynamicProgramming {
    fn run(&self, seed: <XorShiftRng as rand::SeedableRng>::Seed, dim: u8) -> Option<Vec<Coord>> {
        let mut rng = XorShiftRng::from_seed(seed);
        let mut start: Coord = [
            rng.gen_range(0..dim),
            rng.gen_range(0..dim),
            rng.gen_range(0..dim),
        ];
        let r = rng.gen_range(0..3);
        start[r] = 0;
        start[(r + 1) % 3] = 0;

        create_cube_rec(vec![start], &mut rng, dim)
    }
}
