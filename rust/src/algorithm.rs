use rand_xorshift::XorShiftRng;

use crate::common::Cube;

pub trait Algorithm {
    fn run(&self, seed: <XorShiftRng as rand::SeedableRng>::Seed, dim: u8) -> Option<Cube>;
}
