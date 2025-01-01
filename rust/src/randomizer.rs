use crate::algorithm::Algorithm;
use crate::common::get_neighbours;
use crate::common::record_failure;
use crate::common::Coord;
use crate::common::Cube;
use indexmap::IndexSet;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;

fn create_cube_rec(
    chain: &mut IndexSet<Coord>,
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
        if !chain.insert(neighbour) {
            record_failure(chain.len());
            continue;
        };

        return create_cube_rec(chain, rng, dim);
    }
    None
}

pub struct Randomizer;
impl Algorithm for Randomizer {
    fn run(&self, seed: <XorShiftRng as rand::SeedableRng>::Seed, dim: u8) -> Option<Cube> {
        let mut rng = XorShiftRng::from_seed(seed);

        loop {
            let mut start: Coord = [0, 0, 0];
            start[rng.gen_range(0..3)] = rng.gen_range(0..dim);

            let mut chain = IndexSet::with_capacity((dim * dim * dim).into());
            chain.insert(start);

            let result = create_cube_rec(&mut chain, &mut rng, dim).map(|path| Cube {
                dim,
                path: path.into_iter().collect(),
            });

            if result.is_some() {
                return result;
            };
        }
    }
}
