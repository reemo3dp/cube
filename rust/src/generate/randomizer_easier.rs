use super::common::get_neighbours;
use super::common::record_failure;
use super::common::Coord;
use super::algorithm::Algorithm;
use indexmap::IndexSet;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;

fn create_cube_rec(
    chain: &mut IndexSet<Coord>,
    rng: &mut XorShiftRng,
    dim: u8,
) -> Option<IndexSet<Coord>> {
    let cap: u16 = dim.into();
    if chain.len() == (cap * cap * cap).into() {
        return Some(chain.clone());
    }
    let current = chain.last().unwrap();

    let (mut same_plane, next_plane): (Vec<Coord>, Vec<Coord>) = get_neighbours(*current, dim)
        .into_iter()
        .filter(|n| n[2] >= current[2])
        .partition(|n| n[2] == current[2]);
    same_plane.shuffle(rng);
    let neighbours = [same_plane, next_plane].concat();

    for neighbour in neighbours {
        if !chain.insert(neighbour) {
            record_failure(chain.len());
            continue;
        };

        return create_cube_rec(chain, rng, dim);
    }
    None
}

pub struct RandomizerEasier;
impl Algorithm for RandomizerEasier {
    fn run(&self, seed: <XorShiftRng as rand::SeedableRng>::Seed, dim: u8) -> Option<Vec<Coord>> {
        let mut rng = XorShiftRng::from_seed(seed);

        loop {
            let mut start: Coord = [0, 0, 0];
            start[rng.gen_range(0..2)] = rng.gen_range(0..dim);

            let cap: u16 = dim.into();
            let mut chain = IndexSet::with_capacity((cap * cap * cap).into());
            chain.insert(start);

            if let Some(result) = create_cube_rec(&mut chain, &mut rng, dim) {
                return Some(result.into_iter().collect());
            }
        }
    }
}
