//! 焼きなましで使う更新関数のモジュール

use super::{DayState, Schedule, ScheduleConfig, ScheduleState, ScoreProp, Shift, ShiftState};

use kinmu_generator as gen;
use kinmu_model::Score;

use rand::Rng;

/// 生成器で用いる更新関数のための型
/// GeneratorWithAnnealingのUpdateを実装
#[derive(Debug, Clone)]
pub struct Update;

/// Updateの実装
impl gen::Update<ScoreProp, Shift, ShiftState, DayState> for Update {
    fn generate<'a, R: Rng>(
        &self,
        name: &str,
        schedule_config: &'a ScheduleConfig,
    ) -> anyhow::Result<Box<dyn FnMut(&Schedule, &mut R) -> Schedule + 'a>> {
        let schedule_state = &schedule_config.day.schedule_states;
        match name {
            "update1" => Ok(Box::new(move |schedule, rng| {
                update_randomly1(schedule_config, schedule_state, schedule, rng)
            })),
            "update2" => Ok(Box::new(move |schedule, rng| {
                update_randomly2(schedule_config, schedule_state, schedule, rng)
            })),
            "update4" => Ok(Box::new(move |schedule, rng| {
                update_randomly4(schedule_config, schedule_state, schedule, rng)
            })),
            "update5" => Ok(Box::new(move |schedule, rng| {
                update_randomly5(schedule_config, schedule_state, schedule, rng)
            })),
            _ => Err(anyhow::anyhow!(
                "Failed to generate update function {}",
                name
            )),
        }
    }
}

/// ランダムな1つの枠をランダムな枠に変える
/// Absoluteの場合繰り返す
fn update_randomly1<R: Rng>(
    schedule_config: &ScheduleConfig,
    schedule_state: &ScheduleState,
    schedule: &Schedule,
    rng: &mut R,
) -> Schedule {
    let mut new_schedule = schedule.clone();
    let rx: usize = rng.gen_range(0..schedule_config.staff.count);
    let ry: usize = rng.gen_range(schedule_config.day.buffer_count..schedule_config.day.count);
    if schedule_state[rx][ry] != ShiftState::Absolute {
        new_schedule[rx][ry] =
            [Shift::N, Shift::K, Shift::I, Shift::A, Shift::O, Shift::H][rng.gen_range(0..6)];
        new_schedule
    } else {
        update_randomly1(schedule_config, schedule_state, schedule, rng)
    }
}

/// ランダムな1つの枠をランダムな枠に変える
fn update_randomly2<R: Rng>(
    schedule_config: &ScheduleConfig,
    schedule_state: &ScheduleState,
    schedule: &Schedule,
    rng: &mut R,
) -> Schedule {
    let mut new_schedule = schedule.clone();
    let rx: usize = rng.gen_range(0..schedule_config.staff.count);
    let ry: usize = rng.gen_range(schedule_config.day.buffer_count..schedule_config.day.count);
    if schedule_state[rx][ry] != ShiftState::Absolute {
        new_schedule[rx][ry] =
            [Shift::N, Shift::K, Shift::I, Shift::A, Shift::O, Shift::H][rng.gen_range(0..6)];
    }
    new_schedule
}

/// ランダムな1つの枠をN,O,Hのうちランダムな枠に変える Absoluteなら繰り返す
fn update_randomly4<R: Rng>(
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
        update_randomly4(schedule_config, schedule_state, schedule, rng)
        // 合わない場合表を何個も生成することになる
        // 更新確率をpとすると、更新に必要な平均の呼び出し回数は1/p回なのでそれほど問題はない
        // むしろ何も更新せずに評価するほうが問題
    }
}

/*
各行について
1.  Iが入っていることを確認
2.  ランダムなIを取り除き、Nを代わりに置く
3.  孤立したAを取り除き、Nを代わりに置く
4.  ランダムなKを取り除き、Nを代わりに置く
5.  ランダムなNをIで置き換える
6.  Aを必要なら追加する (適当なものを置き換える あらゆる可能性あり)
7.  ランダムなNをKで置き換える
8.  K,Iの数が変わっていないことを確かめる
9.  Iの後にAが来ているか調べる
10. Absoluteが動いていないか調べる
*/

macro_rules! count_waku_row {
    ($shift:expr, $schedule_config: expr, $schedule:expr, $r:expr) => {{
        let mut count: isize = 0;
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
        if new_schedule[r][c] == shift {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    new_schedule[r][is[rnd]] = Shift::N;
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
        if new_schedule[r][c] == Shift::N {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    new_schedule[r][is[rnd]] = shift;
}

fn iak_renzoku(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    r: usize,
    s: &Score,
) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_config.day.count - 1) {
        ans += match (schedule[r][i], schedule[r][i + 1]) {
            (Shift::A, Shift::K) => 0.0,
            (Shift::A, Shift::Y) => 0.0,
            (Shift::A, _) => *s,
            (Shift::I, Shift::A) => 0.0,
            (Shift::I, _) => *s,
            (_, Shift::A) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn remove_improper_a(schedule_config: &ScheduleConfig, new_schedule: &mut Schedule, r: usize) {
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if new_schedule[r][c] == Shift::A && new_schedule[r][c - 1] != Shift::I {
            new_schedule[r][c] = Shift::N;
        }
    }
}

fn add_proper_a(schedule_config: &ScheduleConfig, new_schedule: &mut Schedule, r: usize) {
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if new_schedule[r][c] != Shift::A && new_schedule[r][c - 1] == Shift::I {
            new_schedule[r][c] = Shift::A;
        }
    }
}

/// IAKを破壊せずに入れ替える
/// 前提として、Absolute以外はI,A,K,Nで、AbsoluteでないO,Hはないことが条件
fn update_randomly5<R: Rng>(
    schedule_config: &ScheduleConfig,
    schedule_state: &ScheduleState,
    schedule: &Schedule,
    rng: &mut R,
) -> Schedule {
    let mut new_schedule = schedule.clone();
    for r in 0..schedule_config.staff.count {
        // Iが入っていることを確認
        let i_count = count_waku_row!(Shift::I, schedule_config, schedule, r);
        if i_count == 0 {
            // ランダムなKを取り除き、Nを代わりに置く
            remove_random(Shift::K, schedule_config, &mut new_schedule, r, rng);
            // ランダムなNをKで置き換える
            add_random(Shift::K, schedule_config, &mut new_schedule, r, rng);
        } else {
            // ランダムなIを取り除き、Nを代わりに置く
            remove_random(Shift::I, schedule_config, &mut new_schedule, r, rng);
            // 孤立したAを取り除き、Nを代わりに置く
            remove_improper_a(schedule_config, &mut new_schedule, r);
            // ランダムなKを取り除き、Nを代わりに置く
            remove_random(Shift::K, schedule_config, &mut new_schedule, r, rng);
            // ランダムなNをIで置き換える
            add_random(Shift::I, schedule_config, &mut new_schedule, r, rng);
            // Aを必要なら追加する (適当なものを置き換える あらゆる可能性あり)
            add_proper_a(schedule_config, &mut new_schedule, r);
            // ランダムなNをKで置き換える
            add_random(Shift::K, schedule_config, &mut new_schedule, r, rng);
        }

        // 条件に合うかのチェック

        // 無駄あり 一回で走査できる
        let ic1 = count_waku_row!(Shift::I, schedule_config, schedule, r);
        let ic2 = count_waku_row!(Shift::I, schedule_config, new_schedule, r);
        let kc1 = count_waku_row!(Shift::K, schedule_config, schedule, r);
        let kc2 = count_waku_row!(Shift::K, schedule_config, new_schedule, r);

        // Iの数に変化ないか
        let b1 = ic1 == ic2;

        // Kの数に変化ないか
        let b2 = kc1 == kc2;

        // IAKの連続が崩れていないか
        let b3 = iak_renzoku(schedule_config, schedule, r, &1000.0)
            >= iak_renzoku(schedule_config, &new_schedule, r, &1000.0);

        // Absoluteが変化していないか
        let b4 = {
            let mut ans = true;
            for c in schedule_config.day.buffer_count..schedule_config.day.count {
                if schedule_state[r][c] == ShiftState::Absolute {
                    ans = ans && schedule[r][c] == new_schedule[r][c];
                }
            }
            ans
        };

        // もし変化が不適切なら
        if !(b1 && b2 && b3 && b4) {
            // 戻す
            for c in schedule_config.day.buffer_count..schedule_config.day.count {
                new_schedule[r][c] = schedule[r][c];
            }
        }
    }
    new_schedule
}
