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

use super::super::DayConfig;
use super::{Schedule, ScoreProp};

use ::kinmu_model::{Score, StaffConfig};

/// スコアをmutで評価する
pub fn eval_score_mut(
    sp: &mut ScoreProp,
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
) -> Score {
    match sp {
        ScoreProp::PatternGeneral(p) => {
            pattern_general::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::PatternFixed(p) => {
            pattern_fixed::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::PatternGeneralAny(p) => {
            pattern_general_any::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::PatternFixedAny(p) => {
            pattern_fixed_any::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::Streak(p) => streak::eval_mut(staff_config, day_config, schedule, p),
        ScoreProp::ShiftsBalance(p) => {
            shifts_balance::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::ShiftHalfBalance(p) => {
            shift_half_balance::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::ShiftDirPriority(p) => {
            shift_dir_priority::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::DayCountRegardStaffAttribute(p) => {
            day_count_regard_staff_attribute::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::StaffCountRegardDayAttribute(p) => {
            staff_count_regard_day_attribute::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::StaffCount(p) => staff_count::eval_mut(staff_config, day_config, schedule, p),
        ScoreProp::StaffCountAtLeast(p) => {
            staff_count_at_least::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::StaffCountWithPremise(p) => {
            staff_count_with_premise::eval_mut(staff_config, day_config, schedule, p)
        }
        ScoreProp::NGPair(p) => ng_pair::eval_mut(staff_config, day_config, schedule, p),
        ScoreProp::NoSamePair(p) => no_same_pair::eval_mut(staff_config, day_config, schedule, p),
    }
}

/// スコアをimmutで評価する
pub fn eval_score_immut(
    sp: &ScoreProp,
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
) -> Score {
    match sp {
        ScoreProp::PatternGeneral(p) => {
            pattern_general::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::PatternFixed(p) => {
            pattern_fixed::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::PatternGeneralAny(p) => {
            pattern_general_any::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::PatternFixedAny(p) => {
            pattern_fixed_any::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::Streak(p) => streak::eval_immut(staff_config, day_config, schedule, p),
        ScoreProp::ShiftsBalance(p) => {
            shifts_balance::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::ShiftHalfBalance(p) => {
            shift_half_balance::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::ShiftDirPriority(p) => {
            shift_dir_priority::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::DayCountRegardStaffAttribute(p) => {
            day_count_regard_staff_attribute::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::StaffCountRegardDayAttribute(p) => {
            staff_count_regard_day_attribute::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::StaffCount(p) => staff_count::eval_immut(staff_config, day_config, schedule, p),
        ScoreProp::StaffCountAtLeast(p) => {
            staff_count_at_least::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::StaffCountWithPremise(p) => {
            staff_count_with_premise::eval_immut(staff_config, day_config, schedule, p)
        }
        ScoreProp::NGPair(p) => ng_pair::eval_immut(staff_config, day_config, schedule, p),
        ScoreProp::NoSamePair(p) => no_same_pair::eval_immut(staff_config, day_config, schedule, p),
    }
}
