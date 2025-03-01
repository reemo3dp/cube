use rand_xorshift::XorShiftRng;

use super::common::Coord;



pub trait Algorithm {
    fn run(&self, seed: <XorShiftRng as rand::SeedableRng>::Seed, dim: u8) -> Option<Vec<Coord>>;
}
