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
        return Some(chain);
    }
    let current = chain.last().unwrap();

    let neighbours = get_neighbours(*current, dim);
    let neighbour = neighbours.choose(rng).unwrap();
    if chain.contains(neighbour) {
        record_failure(chain.len());
        return None;
    };

    let mut next_chain = chain.clone();
    next_chain.insert(*neighbour);
    create_cube_rec(next_chain, rng, dim)
}

pub struct SuperRandom;
impl Algorithm for SuperRandom {
    fn run(&self, seed: <XorShiftRng as rand::SeedableRng>::Seed, dim: u8) -> Option<Vec<Coord>> {
        let mut rng = XorShiftRng::from_seed(seed);

        loop {
            let mut start: Coord = [
                rng.gen_range(0..dim),
                rng.gen_range(0..dim),
                rng.gen_range(0..dim),
            ];
            let r = rng.gen_range(0..3);
            start[r] = 0;
            start[(r + 1) % 3] = 0;

            let mut chain = IndexSet::with_capacity((dim * dim * dim).into());
            chain.insert(start);


            if let Some(result) = create_cube_rec(chain, &mut rng, dim) {
                return Some(result.into_iter().collect());
            }
        }
    }
}
