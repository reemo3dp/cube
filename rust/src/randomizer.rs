use crate::algorithm::Algorithm;
use crate::common::get_neighbours;
use crate::common::record_failure;
use crate::common::Coord;
use indexmap::IndexSet;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;

fn create_cube_rec(
    chain: IndexSet<Coord>,
    rng: &mut XorShiftRng,
    dim: u8,
) -> Option<IndexSet<Coord>> {
    if chain.len() == (dim * dim * dim).into() {
        return Some(chain.clone());
    }
    let current = chain.last().unwrap();

    let mut neighbours = get_neighbours(*current, dim);
    neighbours.shuffle(rng);

    for neighbour in neighbours {
        let mut nextChain = chain.clone();
        if !nextChain.insert(neighbour) {
            record_failure(nextChain.len());
            continue;
        };

        return create_cube_rec(nextChain, rng, dim);
    }
    None
}

pub struct Randomizer;
impl Algorithm for Randomizer {
    fn run(&self, seed: <XorShiftRng as rand::SeedableRng>::Seed, dim: u8) -> Option<Vec<Coord>> {
        let mut rng = XorShiftRng::from_seed(seed);

        loop {
            let mut start: Coord = [0, 0, 0];
            start[rng.gen_range(0..3)] = rng.gen_range(0..dim);

            let mut chain = IndexSet::with_capacity((dim * dim * dim).into());
            chain.insert(start);

            if let Some(result) = create_cube_rec(chain, &mut rng, dim) {
                return Some(result.into_iter().collect());
            }
        }
    }
}
