//! シード値をもとに乱数生成器を作り出すモジュール

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

pub fn gen_rng_from_seed(seed: Option<u64>) -> Box<dyn RngCore> {
    match seed {
        Some(s) => Box::new(StdRng::seed_from_u64(s)),
        None => Box::new(rand::thread_rng()),
    }
}
