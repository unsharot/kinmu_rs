//! 焼きなましで使う評価関数のモジュール

mod day_count_regard_staff_attribute;
mod ng_pair;
mod no_same_pair;
mod pattern_fixed;
mod pattern_fixed_any;
mod pattern_general;
mod pattern_general_any;
mod shift_dir_priority;
mod shift_half_balance;
mod shifts_balance;
mod staff_count;
mod staff_count_at_least;
mod staff_count_regard_day_attribute;
mod staff_count_with_premise;
mod streak;

use super::types::{Schedule, ScheduleConfig, Score, ScoreProp};

/// 複数のスコアを評価する
#[allow(clippy::ptr_arg)]
pub fn eval_scores(
    sps: &mut Vec<ScoreProp>,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
) -> Score {
    sps.iter_mut()
        .map(|sp: &mut ScoreProp| eval_score(sp, schedule_config, schedule))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

/// スコアを評価する
fn eval_score(sp: &mut ScoreProp, schedule_config: &ScheduleConfig, schedule: &Schedule) -> Score {
    match sp {
        ScoreProp::PatternGeneral(p) => pattern_general::eval(schedule_config, schedule, p),
        ScoreProp::PatternFixed(p) => pattern_fixed::eval(schedule_config, schedule, p),
        ScoreProp::PatternGeneralAny(p) => pattern_general_any::eval(schedule_config, schedule, p),
        ScoreProp::PatternFixedAny(p) => pattern_fixed_any::eval(schedule_config, schedule, p),
        ScoreProp::Streak(p) => streak::eval(schedule_config, schedule, p),
        ScoreProp::ShiftsBalance(p) => shifts_balance::eval(schedule_config, schedule, p),
        ScoreProp::ShiftHalfBalance(p) => shift_half_balance::eval(schedule_config, schedule, p),
        ScoreProp::ShiftDirPriority(p) => shift_dir_priority::eval(schedule_config, schedule, p),
        ScoreProp::DayCountRegardStaffAttribute(p) => {
            day_count_regard_staff_attribute::eval(schedule_config, schedule, p)
        }
        ScoreProp::StaffCountRegardDayAttribute(p) => {
            staff_count_regard_day_attribute::eval(schedule_config, schedule, p)
        }
        ScoreProp::StaffCount(p) => staff_count::eval(schedule_config, schedule, p),
        ScoreProp::StaffCountAtLeast(p) => staff_count_at_least::eval(schedule_config, schedule, p),
        ScoreProp::StaffCountWithPremise(p) => {
            staff_count_with_premise::eval(schedule_config, schedule, p)
        }
        ScoreProp::NGPair(p) => ng_pair::eval(schedule_config, schedule, p),
        ScoreProp::NoSamePair(p) => no_same_pair::eval(schedule_config, schedule, p),
    }
}
