//! 初めに勤務表を埋める関数のモジュール

/*
fill1はUをランダムな枠に
fill2はIとKの数合わせてうまいこと埋める
*/

use super::{DayState, Schedule, ScheduleConfig, Shift, ShiftState, StdScoreProp};

use kinmu_generator_with_annealing::Fill;

use rand::Rng;

/// 生成器で用いるFill関数のための型
/// GeneratorWithAnnealingのFillを実装
#[derive(Debug, Clone)]
pub struct StdFill;

/// Fillの実装
impl Fill<StdScoreProp, Shift, ShiftState, DayState> for StdFill {
    fn run<R: Rng>(
        &self,
        name: &str,
        schedule_config: &ScheduleConfig,
        mut rng: &mut R,
    ) -> anyhow::Result<Schedule> {
        match name {
            "no_fill" => Ok(no_fill(schedule_config, &mut rng)),
            "fill1" => Ok(fill_randomly1(schedule_config, &mut rng)),
            "fill2" => Ok(fill_randomly2(schedule_config, &mut rng)),
            _ => Err(anyhow::anyhow!("Failed to parse fill function {}", name)),
        }
    }
}

fn no_fill<R: Rng>(schedule_config: &ScheduleConfig, _rng: &mut R) -> Schedule {
    schedule_config.day.requested_schedule.clone()
}

#[allow(clippy::needless_range_loop)]
fn fill_randomly1<R: Rng>(schedule_config: &ScheduleConfig, rng: &mut R) -> Schedule {
    let mut schedule = schedule_config.day.requested_schedule.clone();
    for r in 0..schedule_config.staff.count {
        for c in schedule_config.day.buffer_count..schedule_config.day.count {
            if schedule_config.day.schedule_states[r][c] != ShiftState::Absolute
                && schedule[r][c] == Shift::U
            {
                schedule[r][c] = [Shift::N, Shift::O, Shift::H][rng.gen_range(0..3)];
            }
        }
    }
    schedule
}

/*
fill2のアルゴリズム
1.  Randomの場所をIAKのパターンで埋め、残りはNで埋める
2.  指定されたIと今埋まっているIの差分を計算
3.  余分なIをランダムに消す
4.  孤立したAを消す
5.  指定されたKと今埋まっているKの差分を計算
6.  不足したKをランダムに足す
7.  余分なKを孤立したものを優先にランダムに消す
*/

macro_rules! count_waku_row {
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

fn fill_randomly2<R: Rng>(schedule_config: &ScheduleConfig, rng: &mut R) -> Schedule {
    let mut schedule = schedule_config.day.requested_schedule.clone();
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
        let i_dif = count_waku_row!(Shift::I, schedule_config, schedule, r)
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
            - count_waku_row!(Shift::K, schedule_config, schedule, r);

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
    schedule
}
