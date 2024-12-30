use crate::get_neighbours;
use crate::Coord;
use crate::Cube;
use crate::NUM_TRIED;
use crate::PRINT_EVERY;
use indexmap::IndexSet;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::time::Instant;

fn create_cube_rec(
    chain: &mut IndexSet<Coord>,
    rng: &mut XorShiftRng,
    dim: u8,
    run_start: Instant,
) -> Option<IndexSet<Coord>> {
    if chain.len() == (dim * dim * dim).try_into().unwrap() {
        return Some(chain.clone());
    }
    let current = chain.last().unwrap();

    let mut neighbours = get_neighbours(*current, dim);
    neighbours.shuffle(rng);

    for neighbour in neighbours {
        if !chain.insert(neighbour) {
            unsafe {
                NUM_TRIED += 1;
                if NUM_TRIED % PRINT_EVERY == 0 {
                    println!("//D Stopping at {}", chain.len());
                    println!(
                        "//D {:.0} \t({} for {:?})",
                        NUM_TRIED as f64 / (run_start.elapsed().as_millis() as f64),
                        NUM_TRIED,
                        run_start.elapsed()
                    );
                }
            }
            continue;
        };

        return create_cube_rec(chain, rng, dim, run_start);
    }
    None
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
        let result = create_cube_rec(&mut chain, &mut rng, dim, run_start).map(|path| Cube {
            dim,
            seed,
            path: path.into_iter().collect(),
        });

        if result.is_some() {
            return result;
        };
    }
}
