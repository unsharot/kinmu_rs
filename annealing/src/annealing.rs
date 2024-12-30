use rand::Rng;

/// 焼きなましの実行
/// 一般化されたもの
///
/// # 例
///
/// ```
/// use rand::Rng;
/// use annealing::annealing;
///
/// fn updatef<R: Rng>(x: &f32, rng: &mut R) -> f32 {
///    x + rng.gen::<f32>() / 100.0
/// }
///
/// fn evalf(x: &f32) -> f32 {
///     (x - 5.0) * (x - 5.0)
/// }
///
/// let best_score: f32;
/// let best_model: f32;
///
/// (best_score, best_model) = annealing::run(
///     10000.0,
///     &0.0,
///     100000,
///     updatef,
///     evalf,
///     10.0,
///     0.0,
///     annealing::basic_temp_func,
///     annealing::basic_prob_func,
///     &mut rand::thread_rng(),
/// );
///```
pub fn run<M, S, U, E, T, P, R>(
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
    R: Rng,
{
    let mut best_model = initial_model.clone();
    let mut current_model = initial_model.clone();

    let mut best_score = initial_score;
    let mut current_score = initial_score;

    let mut temp;

    for loop_value in 0..loop_count {
        let next_model = update(&current_model, rng);
        let next_score = eval(&next_model);
        temp = temp_func(temp_max, temp_min, loop_count, loop_value);

        if next_score <= best_score {
            best_model = next_model.clone();
            best_score = next_score;

            current_model = next_model;
            current_score = next_score;
        } else if rng.gen::<f32>() < prob_func(current_score, next_score, temp) {
            current_model = next_model;
            current_score = next_score;
        }
    }

    (best_score, best_model)
}

/// 標準の温度関数
/// 与えられた最大温度と最低温度から、ステップに対して線形な温度を返す
pub fn basic_temp_func(temp_max: f32, temp_min: f32, loop_end: usize, loop_now: usize) -> f32 {
    let r: f32 = (loop_end - loop_now) as f32 / loop_end as f32;
    temp_max - ((temp_max - temp_min) * r)
}

/// 標準の確率関数
/// 前後のスコアと温度から、スコアが悪化した場合に更新する確率を返す
pub fn basic_prob_func(score_now: f32, score_next: f32, temp: f32) -> f32 {
    ((score_now - score_next) / temp).exp()
}
