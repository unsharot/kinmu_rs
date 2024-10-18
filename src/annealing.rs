// use rand::rngs::StdRng;
use rand::Rng;

// TODO: seed: i32を追加する
pub fn annealing<M, S, U, E, T, P, R>(
    initial_score: S,
    initial_model: &M,
    loop_count: usize,
    mut update: U,
    mut eval: E,
    temp_max: f32,
    temp_min: f32,
    mut temp_func: T,
    mut prob_func: P,
    rng: &mut R,
) -> (S, M)
where
    M: Clone,
    S: std::cmp::PartialOrd + Copy,
    U: FnMut(&M, &mut R) -> M,
    E: FnMut(&M) -> S,
    T: FnMut(f32, f32, usize, usize) -> f32,
    P: FnMut(S, S, f32) -> f32,
    R: Rng
{
    let mut best_model = initial_model.clone();
    let mut current_model = initial_model.clone();

    let mut best_score = initial_score;
    let mut current_score = initial_score;

    let mut temp;
    // let mut rng = rand::thread_rng();

    for loop_value in 0..loop_count {
        let next_model = update(&current_model, rng);
        let next_score = eval(&next_model);
        temp = temp_func(temp_max, temp_min, loop_count, loop_value);

        if next_score <= best_score {
            best_model = next_model.clone();
            best_score = next_score;

            current_model = next_model.clone();
            current_score = next_score;
        } else if rng.gen::<f32>() < prob_func(current_score, next_score, temp) {
            current_model = next_model.clone();
            current_score = next_score;
        }
    }

    (best_score, best_model)
}


pub fn basic_temp_func(temp_max: f32, temp_min: f32, loop_end: usize, loop_now: usize) -> f32 {
    let r: f32 = (loop_end - loop_now) as f32 / loop_end as f32;
    temp_max - ((temp_max - temp_min) * r)
}

pub fn basic_prob_func(score_now: f32, score_next: f32, temp: f32) -> f32 {
    ((score_now - score_next) / temp).exp()
}
