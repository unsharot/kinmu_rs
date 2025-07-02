use super::super::{Schedule, ScheduleConfig, ScheduleState, Shift, ShiftState};

use rand::Rng;

/// ランダムな1つの枠をN,O,Hのうちランダムな枠に変える Absoluteなら繰り返す
pub fn update_noh_repeat<R: Rng>(
    schedule_config: &ScheduleConfig,
    schedule_state: &ScheduleState,
    schedule: &Schedule,
    rng: &mut R,
) -> Schedule {
    let mut new_schedule = schedule.clone();
    let rx: usize = rng.gen_range(0..schedule_config.staff.count);
    let ry: usize = rng.gen_range(schedule_config.day.buffer_count..schedule_config.day.count);
    let b1 = schedule_state[rx][ry] != ShiftState::Absolute;
    let b2 = schedule[rx][ry] == Shift::N
        || schedule[rx][ry] == Shift::O
        || schedule[rx][ry] == Shift::H
        || schedule[rx][ry] == Shift::U;
    if b1 && b2 {
        new_schedule[rx][ry] = [Shift::N, Shift::O, Shift::H][rng.gen_range(0..3)];
        new_schedule
    } else {
        update_noh_repeat(schedule_config, schedule_state, schedule, rng)
        // 合わない場合表を何個も生成することになる
        // 更新確率をpとすると、更新に必要な平均の呼び出し回数は1/p回なのでそれほど問題はない
        // むしろ何も更新せずに評価するほうが問題
    }
}
