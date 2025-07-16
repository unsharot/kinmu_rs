//! fill_iak_safeのモジュール
//!
//! ### アルゴリズム
//! 1. K,Iの数が希望で出された数を越えないかチェックする
//! 2. Randomの場所をIAKのパターンで埋め、残りはNで埋める
//! 3. 指定されたIと今埋まっているIの差分を計算
//! 4. 余分なIをランダムに消す
//! 5. 孤立したAを消す
//! 6. 指定されたKと今埋まっているKの差分を計算
//! 7. 不足したKをランダムに足す
//! 8. 余分なKを孤立したものを優先にランダムに消す

use super::super::{Schedule, ScheduleConfig, Shift, ShiftState};

use rand::Rng;

/// 指定したシフトが指定した行にいくつ含まれるか
/// bufferは除いて数える
///
/// ### 例
/// ```ignore
/// let c = count_shift_row!(Shift::I, schedule_config, schedule, r);
/// ```
macro_rules! count_shift_row {
    ($shift:expr, $schedule_config: expr, $schedule:expr, $r:expr) => {{
        let mut count = 0;
        for i in $schedule_config.day.buffer_count..$schedule_config.day.count {
            if $schedule[$r][i] == $shift {
                count += 1;
            }
        }
        count
    }};
}

/// 指定したシフトをランダムにNに変える
fn remove_random<R: Rng>(
    shift: Shift,
    schedule_config: &ScheduleConfig,
    new_schedule: &mut Schedule,
    r: usize,
    rng: &mut R,
) {
    let mut is: Vec<usize> = Vec::new();
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if new_schedule[r][c] == shift
            && schedule_config.day.schedule_states[r][c] != ShiftState::Absolute
        {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    new_schedule[r][is[rnd]] = Shift::N;
}

/// 孤立したAをNで置き換える
/// IAのパターンを守るため
/// KAのようになっているAをKNに変える
fn remove_improper_a(schedule_config: &ScheduleConfig, new_schedule: &mut Schedule, r: usize) {
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if new_schedule[r][c] == Shift::A
            && new_schedule[r][c - 1] != Shift::I
            && schedule_config.day.schedule_states[r][c] != ShiftState::Absolute
        {
            new_schedule[r][c] = Shift::N;
        }
    }
}

/// ランダムな場所に指定したシフトを追加する
fn add_random<R: Rng>(
    shift: Shift,
    schedule_config: &ScheduleConfig,
    new_schedule: &mut Schedule,
    r: usize,
    rng: &mut R,
) {
    let mut is: Vec<usize> = Vec::new();
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if new_schedule[r][c] == Shift::N
            && schedule_config.day.schedule_states[r][c] != ShiftState::Absolute
        {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    new_schedule[r][is[rnd]] = shift;
}

/// 適切な夜勤の数で表を埋める
/// IAKの順番を守る
/// 夜勤の数や公休の数は指定された数通りになる
/// Uになっている枠のみ埋める
/// Absoluteなら埋めない
/// schedule_configは夜勤の数(IDayCount)と公休の数(KDayCount)を持つ必要がある
/// 夜勤か公休の数が自由度を超える場合、panicを起こす
pub fn fill_iak_safe<R: Rng>(
    schedule_config: &ScheduleConfig,
    rng: &mut R,
) -> anyhow::Result<Schedule> {
    let mut schedule = schedule_config.day.requested_schedule.clone();

    // Kの数が超過していないかチェック
    #[allow(clippy::needless_range_loop)]
    for r in 0..schedule_config.staff.count {
        let c1 = count_shift_row!(Shift::K, schedule_config, schedule, r);
        let c2 = schedule_config
            .staff
            .get_attribute(r, &"KDayCount".to_string());

        if c1 > c2 {
            // 超過
            return Err(anyhow::anyhow!(
                "希望で出したKの数({})が職員指定の数({})を超過しています",
                c1,
                c2
            ));
        }
    }

    // Iの数が超過していないかチェック
    #[allow(clippy::needless_range_loop)]
    for r in 0..schedule_config.staff.count {
        let c1 = count_shift_row!(Shift::I, schedule_config, schedule, r);
        let c2 = schedule_config
            .staff
            .get_attribute(r, &"IDayCount".to_string());

        if c1 > c2 {
            // 超過
            return Err(anyhow::anyhow!(
                "希望で出したIの数({})が職員指定の数({})を超過しています",
                c1,
                c2
            ));
        }
    }

    for r in 0..schedule_config.staff.count {
        let mut r_count = 0;
        for c in schedule_config.day.buffer_count..(schedule_config.day.count + 1) {
            // Randomが途切れることを検知して、途切れるなら入るだけIAKを入れる
            // なお、最後は途切れないとしてIAKが埋まるだけ埋める
            if c != schedule_config.day.count
                && schedule_config.day.schedule_states[r][c] == ShiftState::Random
            {
                r_count += 1;
                if r_count == 3 {
                    r_count = 0;
                    schedule[r][c - 2] = Shift::I;
                    schedule[r][c - 1] = Shift::A;
                    schedule[r][c] = Shift::K;
                }
            } else if c == schedule_config.day.count {
                if r_count == 1 {
                    schedule[r][c - 1] = Shift::I;
                } else if r_count == 2 {
                    schedule[r][c - 2] = Shift::I;
                    schedule[r][c - 1] = Shift::A;
                }
            } else {
                if r_count == 1 {
                    schedule[r][c - 1] = Shift::N;
                } else if r_count == 2 {
                    if schedule[r][c] == Shift::K || schedule[r][c] == Shift::Y {
                        schedule[r][c - 2] = Shift::I;
                        schedule[r][c - 1] = Shift::A;
                    } else {
                        schedule[r][c - 2] = Shift::N;
                        schedule[r][c - 1] = Shift::N;
                    }
                }
                r_count = 0;
            }
        }

        // Iの差分を計算
        let i_dif = count_shift_row!(Shift::I, schedule_config, schedule, r)
            - schedule_config
                .staff
                .get_attribute(r, &"IDayCount".to_string());

        // 余分なIをランダムに消す
        for _ in 0..i_dif {
            remove_random(Shift::I, schedule_config, &mut schedule, r, rng);
        }

        // 孤立したAを消す
        remove_improper_a(schedule_config, &mut schedule, r);

        // Kの差分を計算
        let k_dif = schedule_config
            .staff
            .get_attribute(r, &"KDayCount".to_string())
            - count_shift_row!(Shift::K, schedule_config, schedule, r);

        if k_dif > 0 {
            // 不足したKをランダムに足す
            for _ in 0..k_dif {
                add_random(Shift::K, schedule_config, &mut schedule, r, rng);
            }
        } else {
            // 孤立したKとそうでないKのインデックスをとる
            let mut k_nc_ids = Vec::new();
            let mut k_ng_ids = Vec::new();
            for c in schedule_config.day.buffer_count..schedule_config.day.count {
                if (schedule[r][c] == Shift::K)
                    && (schedule_config.day.schedule_states[r][c] == ShiftState::Random)
                {
                    if schedule[r][c - 1] == Shift::A {
                        k_ng_ids.push(c);
                    } else {
                        k_nc_ids.push(c);
                    }
                }
            }

            // Kを消す
            for _ in 0..-k_dif {
                if k_nc_ids.is_empty() {
                    let rnd = rng.gen_range(0..k_ng_ids.len());
                    schedule[r][k_ng_ids[rnd]] = Shift::N;
                    k_ng_ids.remove(rnd);
                } else {
                    let rnd = rng.gen_range(0..k_nc_ids.len());
                    schedule[r][k_nc_ids[rnd]] = Shift::N;
                    k_nc_ids.remove(rnd);
                }
            }
        }
    }
    Ok(schedule)
}
