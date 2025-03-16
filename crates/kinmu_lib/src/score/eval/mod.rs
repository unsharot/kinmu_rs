//! 焼きなましで使う評価関数のモジュール

pub(super) mod day_count_regard_staff_attribute;
pub(super) mod ng_pair;
pub(super) mod no_same_pair;
pub(super) mod pattern_fixed;
pub(super) mod pattern_fixed_any;
pub(super) mod pattern_general;
pub(super) mod pattern_general_any;
pub(super) mod shift_dir_priority;
pub(super) mod shift_half_balance;
pub(super) mod shifts_balance;
pub(super) mod staff_count;
pub(super) mod staff_count_at_least;
pub(super) mod staff_count_regard_day_attribute;
pub(super) mod staff_count_with_premise;
pub(super) mod streak;

use super::super::DayConfig;
use super::{Schedule, ScoreProp};

use kinmu_model::{Score, ScorePropTrait, StaffConfig};

/// スコアをmutで評価する
pub fn eval_score_mut(
    sp: &mut ScoreProp,
    staff_config: &StaffConfig,
    day_config: &DayConfig,
    schedule: &Schedule,
) -> Score {
    match sp {
        ScoreProp::PatternGeneral(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::PatternFixed(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::PatternGeneralAny(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::PatternFixedAny(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::Streak(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::ShiftsBalance(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::ShiftHalfBalance(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::ShiftDirPriority(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::DayCountRegardStaffAttribute(p) => {
            p.eval_mut(staff_config, day_config, schedule)
        }
        ScoreProp::StaffCountRegardDayAttribute(p) => {
            p.eval_mut(staff_config, day_config, schedule)
        }
        ScoreProp::StaffCount(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::StaffCountAtLeast(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::StaffCountWithPremise(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::NGPair(p) => p.eval_mut(staff_config, day_config, schedule),
        ScoreProp::NoSamePair(p) => p.eval_mut(staff_config, day_config, schedule),
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
        ScoreProp::PatternGeneral(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::PatternFixed(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::PatternGeneralAny(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::PatternFixedAny(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::Streak(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::ShiftsBalance(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::ShiftHalfBalance(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::ShiftDirPriority(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::DayCountRegardStaffAttribute(p) => {
            p.eval_immut(staff_config, day_config, schedule)
        }
        ScoreProp::StaffCountRegardDayAttribute(p) => {
            p.eval_immut(staff_config, day_config, schedule)
        }
        ScoreProp::StaffCount(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::StaffCountAtLeast(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::StaffCountWithPremise(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::NGPair(p) => p.eval_immut(staff_config, day_config, schedule),
        ScoreProp::NoSamePair(p) => p.eval_immut(staff_config, day_config, schedule),
    }
}
