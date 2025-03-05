//! シード値をもとに乱数生成器を作り出すモジュール

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

/// シード値からシード付き乱数生成器を生成するか、シードなし乱数生成器を生成する
pub fn gen_rng_from_seed(seed: Option<u64>) -> Box<dyn RngCore> {
    match seed {
        Some(s) => Box::new(StdRng::seed_from_u64(s)),
        None => Box::new(rand::thread_rng()),
    }
}
