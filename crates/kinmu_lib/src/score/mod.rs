//! スコアに関わる型の宣言

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

use self::day_count_regard_staff_attribute::DayCountRegardStaffAttribute;
use self::ng_pair::NGPair;
use self::no_same_pair::NoSamePair;
use self::pattern_fixed::PatternFixed;
use self::pattern_fixed_any::PatternFixedAny;
use self::pattern_general::PatternGeneral;
use self::pattern_general_any::PatternGeneralAny;
use self::shift_dir_priority::ShiftDirPriority;
use self::shift_half_balance::ShiftHalfBalance;
use self::shifts_balance::ShiftsBalance;
use self::staff_count::StaffCount;
use self::staff_count_at_least::StaffCountAtLeast;
use self::staff_count_regard_day_attribute::StaffCountRegardDayAttribute;
use self::staff_count_with_premise::StaffCountWithPremise;
use self::streak::Streak;

use super::{
    CondWrapper, DayConfig, DayState, Schedule, ScheduleConfig, Shift, ShiftState,
    StaffAttributeNameWrapper,
};

use kinmu_input::{Check, FromConfig};
use kinmu_macros::ScorePropTrait;
use kinmu_model::{DayAttributeName, Score, StaffAttributeName, StaffConfig};

use anyhow::Context as _;
use std::fmt;

/// 具体的なスコア
#[derive(Debug, PartialEq, Clone, ScorePropTrait)]
#[score_prop_trait(<Shift, ShiftState, DayState>)]
pub enum ScoreProp {
    PatternGeneral(PatternGeneral),
    PatternFixed(PatternFixed),
    PatternGeneralAny(PatternGeneralAny),
    PatternFixedAny(PatternFixedAny),
    Streak(Streak),
    ShiftsBalance(ShiftsBalance),
    ShiftHalfBalance(ShiftHalfBalance),
    ShiftDirPriority(ShiftDirPriority),
    DayCountRegardStaffAttribute(DayCountRegardStaffAttribute),
    StaffCountRegardDayAttribute(StaffCountRegardDayAttribute),
    StaffCount(StaffCount),
    StaffCountAtLeast(StaffCountAtLeast),
    StaffCountWithPremise(StaffCountWithPremise),
    NGPair(NGPair),
    NoSamePair(NoSamePair),
}

impl Default for ScoreProp {
    fn default() -> Self {
        ScoreProp::PatternGeneral(Default::default())
    }
}

impl fmt::Display for ScoreProp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoreProp::PatternGeneral(p) => write!(f, "PatternGeneral {:?}", p),
            ScoreProp::PatternFixed(p) => write!(f, "PatternFixed {:?}", p),
            ScoreProp::PatternGeneralAny(p) => write!(f, "PatternGeneralAny {:?}", p),
            ScoreProp::PatternFixedAny(p) => write!(f, "PatternFixedAny {:?}", p),
            ScoreProp::Streak(p) => write!(f, "Streak {:?}", p),
            ScoreProp::ShiftsBalance(p) => write!(f, "ShiftsBalance {:?}", p),
            ScoreProp::ShiftHalfBalance(p) => write!(f, "ShiftHalfBalance {:?}", p),
            ScoreProp::ShiftDirPriority(p) => write!(f, "ShiftDirPriority {:?}", p),
            ScoreProp::DayCountRegardStaffAttribute(p) => {
                write!(f, "DayCountRegardStaffAttribute {:?}", p)
            }
            ScoreProp::StaffCountRegardDayAttribute(p) => {
                write!(f, "StaffCountRegardDayAttribute {:?}", p)
            }
            ScoreProp::StaffCount(p) => write!(f, "StaffCount {:?}", p),
            ScoreProp::StaffCountAtLeast(p) => write!(f, "StaffCountAtLeast {:?}", p),
            ScoreProp::StaffCountWithPremise(p) => write!(f, "StaffCountWithPremise {:?}", p),
            ScoreProp::NGPair(p) => write!(f, "NGPair {:?}", p),
            ScoreProp::NoSamePair(p) => write!(f, "NoSamePair {:?}", p),
        }
    }
}

impl Check<ScoreProp, Shift, ShiftState, DayState> for ScoreProp {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        match self {
            ScoreProp::PatternGeneral(p) => p.check(schedule_config),
            ScoreProp::PatternFixed(p) => p.check(schedule_config),
            ScoreProp::PatternGeneralAny(p) => p.check(schedule_config),
            ScoreProp::PatternFixedAny(p) => p.check(schedule_config),
            ScoreProp::Streak(p) => p.check(schedule_config),
            ScoreProp::ShiftsBalance(p) => p.check(schedule_config),
            ScoreProp::ShiftHalfBalance(p) => p.check(schedule_config),
            ScoreProp::ShiftDirPriority(p) => p.check(schedule_config),
            ScoreProp::DayCountRegardStaffAttribute(p) => p.check(schedule_config),
            ScoreProp::StaffCountRegardDayAttribute(p) => p.check(schedule_config),
            ScoreProp::StaffCount(p) => p.check(schedule_config),
            ScoreProp::StaffCountAtLeast(p) => p.check(schedule_config),
            ScoreProp::StaffCountWithPremise(p) => p.check(schedule_config),
            ScoreProp::NGPair(p) => p.check(schedule_config),
            ScoreProp::NoSamePair(p) => p.check(schedule_config),
        }
        .with_context(|| format!("スコア {:?} の変換チェックに失敗しました", self))?;

        Ok(())
    }
}

impl FromConfig for ScoreProp {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words: Vec<&str> = s.splitn(2, ' ').collect();
        anyhow::ensure!(words.len() >= 2, "Needs 2 fields, but not enough.");
        anyhow::ensure!(2 >= words.len(), "Needs 2 fields, but too much given.");
        helper_sp(words[0], words[1]).with_context(|| format!("Failed to parse {}", s))
    }
}

/// FromConfigの実装のためのヘルパー関数
#[inline(always)]
fn helper_sp(w1: &str, w2: &str) -> anyhow::Result<ScoreProp> {
    match (w1, w2) {
        ("PatternGeneral", p) => Ok(ScoreProp::PatternGeneral({
            let (cw, VecVecShiftWrapper(vvs), s) =
                <(CondWrapper, VecVecShiftWrapper, Score)>::from_config(p)?;
            PatternGeneral::new((cw, vvs, s))
        })),
        ("PatternFixed", p) => Ok(ScoreProp::PatternFixed({
            let (cw, VecShiftWrapper(vs), s) =
                <(CondWrapper, VecShiftWrapper, Score)>::from_config(p)?;
            PatternFixed::new((cw, vs, s))
        })),
        ("PatternGeneralAny", p) => Ok(ScoreProp::PatternGeneralAny({
            let (cw, VecVecShiftWrapper(vvs), s) =
                <(CondWrapper, VecVecShiftWrapper, Score)>::from_config(p)?;
            PatternGeneralAny::new((cw, vvs, s))
        })),
        ("PatternFixedAny", p) => Ok(ScoreProp::PatternFixedAny({
            let (cw, VecShiftWrapper(vs), s) =
                <(CondWrapper, VecShiftWrapper, Score)>::from_config(p)?;
            PatternFixedAny::new((cw, vs, s))
        })),
        ("Streak", p) => Ok(ScoreProp::Streak({
            let (cw, VecShiftWrapper(vs), i, s) =
                <(CondWrapper, VecShiftWrapper, i32, Score)>::from_config(p)?;
            Streak::new((cw, vs, i, s))
        })),
        ("ShiftsBalance", p) => Ok(ScoreProp::ShiftsBalance(ShiftsBalance::new(<(
            CondWrapper,
            Shift,
            Shift,
            Score,
        )>::from_config(
            p
        )?))),
        ("ShiftHalfBalance", p) => Ok(ScoreProp::ShiftHalfBalance(ShiftHalfBalance::new(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        ))),
        ("ShiftDirPriority", p) => Ok(ScoreProp::ShiftDirPriority(ShiftDirPriority::new(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        ))),
        ("DayCountRegardStaffAttribute", p) => Ok(ScoreProp::DayCountRegardStaffAttribute(
            DayCountRegardStaffAttribute::new(
                <(CondWrapper, Shift, StaffAttributeName, Score)>::from_config(p)?,
            ),
        )),
        ("StaffCountRegardDayAttribute", p) => Ok(ScoreProp::StaffCountRegardDayAttribute(
            StaffCountRegardDayAttribute::new(
                <(CondWrapper, Shift, DayAttributeName, Score)>::from_config(p)?,
            ),
        )),
        ("StaffCount", p) => Ok(ScoreProp::StaffCount(StaffCount::new(<(
            CondWrapper,
            Shift,
            i32,
            Score,
        )>::from_config(
            p
        )?))),
        ("StaffCountAtLeast", p) => Ok(ScoreProp::StaffCountAtLeast(StaffCountAtLeast::new(
            <(CondWrapper, Shift, i32, Score)>::from_config(p)?,
        ))),
        ("StaffCountWithPremise", p) => Ok(ScoreProp::StaffCountWithPremise(
            StaffCountWithPremise::new(
                <(CondWrapper, Shift, i32, CondWrapper, Shift, i32, Score)>::from_config(p)?,
            ),
        )),
        ("NGPair", p) => Ok(ScoreProp::NGPair(NGPair::new(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        ))),
        ("NoSamePair", p) => Ok(ScoreProp::NoSamePair(NoSamePair::new(<(
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

/// Vecを読み込む
/// 2重入れ子構造になったVecにも対応
fn format_str_vec_to_words(s: &str) -> anyhow::Result<Vec<&str>> {
    let trimmed_s = s.trim();
    if !trimmed_s.starts_with('[') {
        return Err(anyhow::anyhow!("\'[\' not found"));
    }
    if !trimmed_s.ends_with(']') {
        return Err(anyhow::anyhow!("\']\' not found"));
    }
    let bare_s = &trimmed_s[1..(trimmed_s.len() - 1)];
    let mut words = Vec::new();
    let mut bracket_flag = false;
    let mut start_idx = 0;
    let mut end_idx = 0;
    for c in bare_s.chars() {
        if !bracket_flag && c == ',' {
            words.push(bare_s[start_idx..end_idx].trim());
            start_idx = end_idx + c.len_utf8();
        }
        if c == '[' {
            bracket_flag = true;
        }
        if c == ']' {
            bracket_flag = false;
        }
        end_idx += c.len_utf8();
    }
    if !bare_s[start_idx..end_idx].trim().is_empty() {
        words.push(bare_s[start_idx..end_idx].trim());
    }

    Ok(words)
}

/// Vec<Shift>にFromConfigを実装するためのWrapper
struct VecShiftWrapper(pub Vec<Shift>);

impl FromConfig for VecShiftWrapper {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_vec_to_words(s)?;
        let mut ans = Vec::new();
        for w in words {
            ans.push(<Shift>::from_config(w)?);
        }
        Ok(VecShiftWrapper(ans))
    }
}

/// Vec<Vec<Shift>>にFromConfigを実装するためのWrapper
struct VecVecShiftWrapper(pub Vec<Vec<Shift>>);

impl FromConfig for VecVecShiftWrapper {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_vec_to_words(s)?;
        let mut ans = Vec::new();
        for w in words {
            ans.push(VecShiftWrapper::from_config(w)?.0);
        }
        Ok(VecVecShiftWrapper(ans))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::Cond;

    #[test]
    fn vec_shift_test() {
        let v1 = <VecShiftWrapper>::from_config("[N, I, K]").unwrap();

        assert_eq!(v1.0, vec![Shift::N, Shift::I, Shift::K]);
    }

    #[test]
    fn vec_vec_shift_test() {
        let v2 = <VecVecShiftWrapper>::from_config("[[N, I, K], [O, H, A]]").unwrap();

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
        let s = "PatternGeneral (Every (), [[N,O,H], [O,H], [K, Y]], 123)";
        println!("{:?}", ScoreProp::from_config(s).unwrap());
        assert_eq!(
            ScoreProp::PatternGeneral(PatternGeneral::new((
                CondWrapper::new(Cond::Every),
                vec![
                    vec![Shift::N, Shift::O, Shift::H],
                    vec![Shift::O, Shift::H],
                    vec![Shift::K, Shift::Y]
                ],
                123.0
            ))),
            ScoreProp::from_config(s).unwrap()
        );
    }
}
