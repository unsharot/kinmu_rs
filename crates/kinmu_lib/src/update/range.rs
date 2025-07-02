use super::super::{Schedule, ScheduleConfig, ScheduleState, ShiftState};

use rand::Rng;

/// ランダムなスタッフ同士の連続する1~3日分の範囲を入れ替える
pub fn update_range<R: Rng>(
    schedule_config: &ScheduleConfig,
    schedule_state: &ScheduleState,
    schedule: &Schedule,
    rng: &mut R,
) -> Schedule {
    let mut new_schedule = schedule.clone();
    let range_size = rng.gen_range(1..3);

    let staff1 = rng.gen_range(0..schedule_config.staff.count);
    let staff2 = rng.gen_range(0..schedule_config.staff.count);

    let day1_begin = rng.gen_range(0..schedule_config.day.count - range_size + 1);
    let day2_begin = rng.gen_range(0..schedule_config.day.count - range_size + 1);

    let mut cancel = false;

    for dd in 0..range_size {
        // 交換
        new_schedule[staff1][day1_begin + dd] = schedule[staff2][day2_begin + dd];
        new_schedule[staff2][day2_begin + dd] = schedule[staff1][day1_begin + dd];

        // Absoluteかつ一致しないなら
        let abs1 = schedule_state[staff1][day1_begin + dd] == ShiftState::Absolute;
        let abs2 = schedule_state[staff2][day2_begin + dd] == ShiftState::Absolute;
        let same1 = new_schedule[staff1][day1_begin + dd] == schedule[staff1][day1_begin + dd];
        let same2 = new_schedule[staff1][day1_begin + dd] == schedule[staff1][day1_begin + dd];
        if (abs1 && !same1) || (abs2 && !same2) {
            cancel = true;
        }
    }

    if cancel {
        schedule.clone()
    } else {
        new_schedule
    }
}
