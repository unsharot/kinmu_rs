//! シード値をもとに乱数生成器を作り出すモジュール

use rand::{SeedableRng, RngCore};
use rand::rngs::StdRng;

pub fn gen_rng_from_seed(seed: usize) -> Box<dyn RngCore> {
    if seed == 0 {
        Box::new(rand::thread_rng())
    } else {
        Box::new(StdRng::seed_from_u64(seed as u64))
    }
}