use super::super::{Schedule, ScheduleConfig, ScheduleState, Shift, ShiftState};

use kinmu_annealing::Update;
use kinmu_model::Score;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct IAKSafe {
    pub schedule_config: ScheduleConfig,
    pub schedule_state: ScheduleState,
}

// Wrapperが持った法がよさそう
// CondWrapperみたいなイメージで、Wrapperがschedule_config,stateもつ
// だからUpdateWrapperになるかな？
// あとScorePropトレイトみたいなのも追加したほうがいい
// modelでScoreProp宣言されてるけど上位である意味はあるのか？
// -> 下である必要がないから上でいいんじゃね？ 頻繁に変わらないし

impl Update<Schedule> for IAKSafe {
    fn run<R: Rng>(&self, schedule: &Schedule, rng: &mut R) -> Schedule {
        update_iak_safe(&self.schedule_config, &self.schedule_state, schedule, rng)
    }
}

// 各行について
// 1.  Iが入っていることを確認
// 2.  ランダムなIを取り除き、Nを代わりに置く
// 3.  孤立したAを取り除き、Nを代わりに置く
// 4.  ランダムなKを取り除き、Nを代わりに置く
// 5.  ランダムなNをIで置き換える
// 6.  Aを必要なら追加する (適当なものを置き換える あらゆる可能性あり)
// 7.  ランダムなNをKで置き換える
// 8.  K,Iの数が変わっていないことを確かめる
// 9.  Iの後にAが来ているか調べる
// 10. Absoluteが動いていないか調べる

/// 指定したシフトが指定した行にいくつ含まれるか
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
        if new_schedule[r][c] == shift {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    new_schedule[r][is[rnd]] = Shift::N;
}

/// ランダムなNを指定したシフトに変更する
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

/// IAKが連続で出現しているなら0.0
/// どこかで崩れているならその分ペナルティを返す
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

/// Iの後に来ないAをNで置き換える
fn remove_improper_a(schedule_config: &ScheduleConfig, new_schedule: &mut Schedule, r: usize) {
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if new_schedule[r][c] == Shift::A && new_schedule[r][c - 1] != Shift::I {
            new_schedule[r][c] = Shift::N;
        }
    }
}

/// Iの後にAがない場合、それをAにする
fn add_proper_a(schedule_config: &ScheduleConfig, new_schedule: &mut Schedule, r: usize) {
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if new_schedule[r][c] != Shift::A && new_schedule[r][c - 1] == Shift::I {
            new_schedule[r][c] = Shift::A;
        }
    }
}

/// IAKを破壊せずに入れ替える
/// 前提として、Absolute以外はI,A,K,Nで、AbsoluteでないO,Hはないことが条件
fn update_iak_safe<R: Rng>(
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
