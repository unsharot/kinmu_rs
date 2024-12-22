//! シード値をもとに乱数生成器を作り出すモジュール

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn gen_rng_from_seed(seed: usize) -> (usize, StdRng) {
    if seed == 0 {
        let seed = rand::thread_rng().gen();
        (seed, StdRng::seed_from_u64(seed as u64))
    } else {
        (seed, StdRng::seed_from_u64(seed as u64))
    }
}
