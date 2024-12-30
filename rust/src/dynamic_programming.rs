use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_xorshift::XorShiftRng;
use std::time::Instant;
use crate::get_neighbours;
use crate::Coord;
use crate::Cube;
use crate::NUM_TRIED;
use crate::PRINT_EVERY;



fn create_cube_rec(
    chain: Vec<Coord>,
    rng: &mut XorShiftRng,
    dim: u8,
    run_start: Instant,
) -> Option<Vec<Coord>> {
    if chain.len() == (dim * dim * dim).try_into().unwrap() {
        return Some(chain);
    }
    let current = chain.last().unwrap();
    let mut neighbours = get_neighbours(*current, dim);
    neighbours.shuffle(rng);

    for neighbour in neighbours {
        if chain.contains(&neighbour) {
            unsafe {
                NUM_TRIED += 1;
                if NUM_TRIED % PRINT_EVERY == 0 {
                    println!("//D Stopping at {}", chain.len());
                    println!(
                        "//D {}",
                        NUM_TRIED as f64 / (run_start.elapsed().as_millis() as f64)
                    );
                }
            }
            continue;
        };

        let mut next_chain = chain.clone();
        next_chain.push(neighbour);
        let result = create_cube_rec(next_chain, rng, dim, run_start);
        if result.is_some() {
            return result;
        }
    }

    None
}
pub fn create_cube(
    seed: <XorShiftRng as rand::SeedableRng>::Seed,
    dim: u8,
    run_start: Instant,
) -> Option<Cube> {
    let mut rng = XorShiftRng::from_seed(seed);
    let mut start: Coord = [
        rng.gen_range(0..dim),
        rng.gen_range(0..dim),
        rng.gen_range(0..dim),
    ];
    let r = rng.gen_range(0..3);
    start[r] = 0;
    start[(r + 1) % 3] = 0;

    create_cube_rec(vec![start], &mut rng, dim, run_start).map(|path| Cube { dim, seed, path })
}
