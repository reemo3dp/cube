use crate::get_neighbours;
use crate::Coord;
use crate::Cube;
use crate::NUM_TRIED;
use crate::PRINT_EVERY;
use indexmap::IndexMap;
use indexmap::IndexSet;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::time::Instant;

fn create_cube_rec(
    chain: IndexSet<Coord>,
    rng: &mut XorShiftRng,
    dim: u8,
    run_start: Instant,
) -> Option<IndexSet<Coord>> {
    if chain.len() == (dim * dim * dim).try_into().unwrap() {
        return Some(chain);
    }
    let current = chain.last().unwrap();

    let neighbours = get_neighbours(*current, dim);
    let neighbour = neighbours.choose(rng).unwrap();
    if chain.contains(neighbour) {
        unsafe {
            NUM_TRIED += 1;
            if NUM_TRIED % 10000 == 0 {
                println!("//D Stopping at {}", chain.len());
                println!(
                    "//D {}",
                    NUM_TRIED as f64 / (run_start.elapsed().as_millis() as f64)
                );
            }
        }
        return None;
    };

    let mut next_chain = chain.clone();
    next_chain.insert(*neighbour);
    create_cube_rec(next_chain, rng, dim, run_start)
}

pub fn create_cube(
    seed: <XorShiftRng as rand::SeedableRng>::Seed,
    dim: u8,
    run_start: Instant,
) -> Option<Cube> {
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

        let mut chain = IndexSet::with_capacity((dim * dim * dim).try_into().unwrap());
        chain.insert(start);

        let result = create_cube_rec(chain, &mut rng, dim, run_start).map(|path| Cube {
            dim,
            seed,
            path: path.into_iter().collect(),
        });
        unsafe {
            NUM_TRIED += 1;
            if NUM_TRIED % PRINT_EVERY == 0 {
                println!(
                    "{}",
                    NUM_TRIED as f64 / (run_start.elapsed().as_millis() as f64)
                );
            }
        }
        if result.is_some() {
            return result;
        };
    }
}
