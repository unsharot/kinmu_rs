//! スコアに関わる型の宣言

mod ng_pair;
mod no_same_pair;
mod pattern_fixed;
mod pattern_fixed_any;
mod pattern_general;
mod pattern_general_any;
mod shift_dir_priority;
mod shift_distance;
mod shift_half_balance;
mod shifts_balance;
mod shifts_count_at_most;
mod staff_count;
mod staff_count_at_least;
mod staff_count_regard_day_attribute;
mod staff_count_regard_staff_attribute;
mod staff_count_with_premise;
mod streak;

use self::ng_pair::NGPair;
use self::no_same_pair::NoSamePair;
use self::pattern_fixed::PatternFixed;
use self::pattern_fixed_any::PatternFixedAny;
use self::pattern_general::PatternGeneral;
use self::pattern_general_any::PatternGeneralAny;
use self::shift_dir_priority::ShiftDirPriority;
use self::shift_distance::ShiftDistance;
use self::shift_half_balance::ShiftHalfBalance;
use self::shifts_balance::ShiftsBalance;
use self::shifts_count_at_most::ShiftsCountAtMost;
use self::staff_count::StaffCount;
use self::staff_count_at_least::StaffCountAtLeast;
use self::staff_count_regard_day_attribute::StaffCountRegardDayAttribute;
use self::staff_count_regard_staff_attribute::StaffCountRegardStaffAttribute;
use self::staff_count_with_premise::StaffCountWithPremise;
use self::streak::Streak;

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState,
    StaffAttributeNameWrapper,
};

use kinmu_input_by_file::{Check, FromConfig, VecVecWrapper, VecWrapper};
use kinmu_macros::ScoreProp;
use kinmu_model::{DayAttributeName, Score, StaffAttributeName, StaffConfig};

use anyhow::Context as _;
use std::fmt;

/// 具体的なスコア
#[derive(Debug, PartialEq, Clone, ScoreProp)]
#[score_prop(<Shift, ShiftState, DayState>)]
pub enum StdScoreProp {
    PatternGeneral(PatternGeneral),
    PatternFixed(PatternFixed),
    PatternGeneralAny(PatternGeneralAny),
    PatternFixedAny(PatternFixedAny),
    Streak(Streak),
    ShiftsBalance(ShiftsBalance),
    ShiftHalfBalance(ShiftHalfBalance),
    ShiftDirPriority(ShiftDirPriority),
    ShiftDistance(ShiftDistance),
    ShiftsCountAtMost(ShiftsCountAtMost),
    StaffCountRegardStaffAttribute(StaffCountRegardStaffAttribute),
    StaffCountRegardDayAttribute(StaffCountRegardDayAttribute),
    StaffCount(StaffCount),
    StaffCountAtLeast(StaffCountAtLeast),
    StaffCountWithPremise(StaffCountWithPremise),
    NGPair(NGPair),
    NoSamePair(NoSamePair),
}

impl Default for StdScoreProp {
    fn default() -> Self {
        StdScoreProp::PatternGeneral(Default::default())
    }
}

impl fmt::Display for StdScoreProp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StdScoreProp::PatternGeneral(p) => write!(f, "PatternGeneral {:?}", p),
            StdScoreProp::PatternFixed(p) => write!(f, "PatternFixed {:?}", p),
            StdScoreProp::PatternGeneralAny(p) => write!(f, "PatternGeneralAny {:?}", p),
            StdScoreProp::PatternFixedAny(p) => write!(f, "PatternFixedAny {:?}", p),
            StdScoreProp::Streak(p) => write!(f, "Streak {:?}", p),
            StdScoreProp::ShiftsBalance(p) => write!(f, "ShiftsBalance {:?}", p),
            StdScoreProp::ShiftHalfBalance(p) => write!(f, "ShiftHalfBalance {:?}", p),
            StdScoreProp::ShiftDirPriority(p) => write!(f, "ShiftDirPriority {:?}", p),
            StdScoreProp::ShiftDistance(p) => {
                write!(f, "ShiftDistance {:?}", p)
            }
            StdScoreProp::ShiftsCountAtMost(p) => {
                write!(f, "ShiftsCountAtMost {:?}", p)
            }
            StdScoreProp::StaffCountRegardStaffAttribute(p) => {
                write!(f, "StaffCountRegardStaffAttribute {:?}", p)
            }
            StdScoreProp::StaffCountRegardDayAttribute(p) => {
                write!(f, "StaffCountRegardDayAttribute {:?}", p)
            }
            StdScoreProp::StaffCount(p) => write!(f, "StaffCount {:?}", p),
            StdScoreProp::StaffCountAtLeast(p) => write!(f, "StaffCountAtLeast {:?}", p),
            StdScoreProp::StaffCountWithPremise(p) => write!(f, "StaffCountWithPremise {:?}", p),
            StdScoreProp::NGPair(p) => write!(f, "NGPair {:?}", p),
            StdScoreProp::NoSamePair(p) => write!(f, "NoSamePair {:?}", p),
        }
    }
}

impl Check<StdScoreProp, Shift, ShiftState, DayState> for StdScoreProp {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        match self {
            StdScoreProp::PatternGeneral(p) => p.check(schedule_config),
            StdScoreProp::PatternFixed(p) => p.check(schedule_config),
            StdScoreProp::PatternGeneralAny(p) => p.check(schedule_config),
            StdScoreProp::PatternFixedAny(p) => p.check(schedule_config),
            StdScoreProp::Streak(p) => p.check(schedule_config),
            StdScoreProp::ShiftsBalance(p) => p.check(schedule_config),
            StdScoreProp::ShiftHalfBalance(p) => p.check(schedule_config),
            StdScoreProp::ShiftDirPriority(p) => p.check(schedule_config),
            StdScoreProp::ShiftDistance(p) => p.check(schedule_config),
            StdScoreProp::ShiftsCountAtMost(p) => p.check(schedule_config),
            StdScoreProp::StaffCountRegardStaffAttribute(p) => p.check(schedule_config),
            StdScoreProp::StaffCountRegardDayAttribute(p) => p.check(schedule_config),
            StdScoreProp::StaffCount(p) => p.check(schedule_config),
            StdScoreProp::StaffCountAtLeast(p) => p.check(schedule_config),
            StdScoreProp::StaffCountWithPremise(p) => p.check(schedule_config),
            StdScoreProp::NGPair(p) => p.check(schedule_config),
            StdScoreProp::NoSamePair(p) => p.check(schedule_config),
        }
        .with_context(|| format!("スコア {:?} の変換チェックに失敗しました", self))?;

        Ok(())
    }
}

impl FromConfig for StdScoreProp {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words: Vec<&str> = s.splitn(2, ' ').collect();
        anyhow::ensure!(words.len() >= 2, "Needs 2 fields, but not enough.");
        anyhow::ensure!(2 >= words.len(), "Needs 2 fields, but too much given.");
        helper_sp(words[0], words[1]).with_context(|| format!("Failed to parse {}", s))
    }
}

/// FromConfigの実装のためのヘルパー関数
#[inline(always)]
fn helper_sp(w1: &str, w2: &str) -> anyhow::Result<StdScoreProp> {
    match (w1, w2) {
        ("PatternGeneral", p) => Ok(StdScoreProp::PatternGeneral({
            let (cw, VecVecWrapper::<Shift>(vvs), s) =
                <(CondWrapper, VecVecWrapper<Shift>, Score)>::from_config(p)?;
            PatternGeneral::new((cw, vvs, s))
        })),
        ("PatternFixed", p) => Ok(StdScoreProp::PatternFixed({
            let (cw, VecWrapper::<Shift>(vs), s) =
                <(CondWrapper, VecWrapper<Shift>, Score)>::from_config(p)?;
            PatternFixed::new((cw, vs, s))
        })),
        ("PatternGeneralAny", p) => Ok(StdScoreProp::PatternGeneralAny({
            let (cw, VecVecWrapper::<Shift>(vvs), s) =
                <(CondWrapper, VecVecWrapper<Shift>, Score)>::from_config(p)?;
            PatternGeneralAny::new((cw, vvs, s))
        })),
        ("PatternFixedAny", p) => Ok(StdScoreProp::PatternFixedAny({
            let (cw, VecWrapper::<Shift>(vs), s) =
                <(CondWrapper, VecWrapper<Shift>, Score)>::from_config(p)?;
            PatternFixedAny::new((cw, vs, s))
        })),
        ("Streak", p) => Ok(StdScoreProp::Streak({
            let (cw, VecWrapper::<Shift>(vs), i, s) =
                <(CondWrapper, VecWrapper<Shift>, i32, Score)>::from_config(p)?;
            Streak::new((cw, vs, i, s))
        })),
        ("ShiftsBalance", p) => Ok(StdScoreProp::ShiftsBalance(ShiftsBalance::new(
            <(CondWrapper, Shift, Shift, Score)>::from_config(p)?,
        ))),
        ("ShiftHalfBalance", p) => Ok(StdScoreProp::ShiftHalfBalance(ShiftHalfBalance::new(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        ))),
        ("ShiftDirPriority", p) => Ok(StdScoreProp::ShiftDirPriority(ShiftDirPriority::new(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        ))),
        ("ShiftDistance", p) => Ok(StdScoreProp::ShiftDistance(ShiftDistance::new(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        ))),
        ("ShiftsCountAtMost", p) => Ok(StdScoreProp::ShiftsCountAtMost({
            let (cw, VecWrapper(vs), i, s) =
                <(CondWrapper, VecWrapper<Shift>, i32, Score)>::from_config(p)?;
            ShiftsCountAtMost::new((cw, vs, i, s))
        })),
        ("StaffCountRegardStaffAttribute", p) => Ok(StdScoreProp::StaffCountRegardStaffAttribute(
            StaffCountRegardStaffAttribute::new(
                <(CondWrapper, Shift, StaffAttributeName, Score)>::from_config(p)?,
            ),
        )),
        ("StaffCountRegardDayAttribute", p) => Ok(StdScoreProp::StaffCountRegardDayAttribute(
            StaffCountRegardDayAttribute::new(
                <(CondWrapper, Shift, DayAttributeName, Score)>::from_config(p)?,
            ),
        )),
        ("StaffCount", p) => Ok(StdScoreProp::StaffCount(StaffCount::new(<(
            CondWrapper,
            Shift,
            i32,
            Score,
        )>::from_config(
            p
        )?))),
        ("StaffCountAtLeast", p) => Ok(StdScoreProp::StaffCountAtLeast(StaffCountAtLeast::new(
            <(CondWrapper, Shift, i32, Score)>::from_config(p)?,
        ))),
        ("StaffCountWithPremise", p) => Ok(StdScoreProp::StaffCountWithPremise(
            StaffCountWithPremise::new(
                <(CondWrapper, Shift, i32, CondWrapper, Shift, i32, Score)>::from_config(p)?,
            ),
        )),
        ("NGPair", p) => Ok(StdScoreProp::NGPair(NGPair::new(<(
            CondWrapper,
            Shift,
            Score,
        )>::from_config(
            p
        )?))),
        ("NoSamePair", p) => Ok(StdScoreProp::NoSamePair(NoSamePair::new(<(
            CondWrapper,
            i32,
            Shift,
            Score,
        )>::from_config(
            p
        )?))),
        (s, _) => Err(anyhow::anyhow!("Unexpected ScoreProp {}", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::Cond;

    #[test]
    fn vec_shift_test() {
        let v1 = <VecWrapper<Shift>>::from_config("[N, I, K]").unwrap();

        assert_eq!(v1.0, vec![Shift::N, Shift::I, Shift::K]);
    }

    #[test]
    fn vec_vec_shift_test() {
        let v2 = <VecVecWrapper<Shift>>::from_config("[[N, I, K], [O, H, A]]").unwrap();

        assert_eq!(
            v2.0,
            vec![
                vec![Shift::N, Shift::I, Shift::K],
                vec![Shift::O, Shift::H, Shift::A]
            ]
        );
    }

    #[test]
    fn score_prop_test() {
        let s = "PatternGeneral (True (), [[N,O,H], [O,H], [K, Y]], 123)";
        println!("{:?}", StdScoreProp::from_config(s).unwrap());
        assert_eq!(
            StdScoreProp::PatternGeneral(PatternGeneral::new((
                CondWrapper::new(Cond::True),
                vec![
                    vec![Shift::N, Shift::O, Shift::H],
                    vec![Shift::O, Shift::H],
                    vec![Shift::K, Shift::Y]
                ],
                123.0
            ))),
            StdScoreProp::from_config(s).unwrap()
        );
    }
}
