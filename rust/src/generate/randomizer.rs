use super::algorithm::Algorithm;
use super::common::get_neighbours;
use super::common::record_failure;
use super::common::Coord;
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
        let mut next_chain = chain.clone();
        if !next_chain.insert(neighbour) {
            record_failure(next_chain.len());
            continue;
        };

        return create_cube_rec(next_chain, rng, dim);
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
