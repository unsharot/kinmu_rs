//! スコアに関わる型の宣言

use super::super::score::{eval_score_immut, eval_score_mut};
use super::cond::CondWrapper;
use super::schedule::{DayState, Shift, ShiftState};
use super::Cond;

use ::kinmu_input::{Check, FromConfig};
use ::kinmu_model::{DayAttributeName, ScheduleConfig, Score, ScorePropTrait, StaffAttributeName};

use anyhow::Context as _;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum ScoreProp {
    PatternGeneral((CondWrapper, Vec<Vec<Shift>>, Score)),
    PatternFixed((CondWrapper, Vec<Shift>, Score)),
    PatternGeneralAny((CondWrapper, Vec<Vec<Shift>>, Score)),
    PatternFixedAny((CondWrapper, Vec<Shift>, Score)),
    Streak((CondWrapper, Vec<Shift>, i32, Score)),
    ShiftsBalance((CondWrapper, Shift, Shift, Score)),
    ShiftHalfBalance((CondWrapper, Shift, Score)),
    ShiftDirPriority((CondWrapper, Shift, Score)),
    DayCountRegardStaffAttribute((CondWrapper, Shift, StaffAttributeName, Score)),
    StaffCountRegardDayAttribute((CondWrapper, Shift, DayAttributeName, Score)),
    StaffCount((CondWrapper, Shift, i32, Score)),
    StaffCountAtLeast((CondWrapper, Shift, i32, Score)),
    StaffCountWithPremise((CondWrapper, Shift, i32, CondWrapper, Shift, i32, Score)),
    NGPair((CondWrapper, Shift, Score)),
    NoSamePair((CondWrapper, i32, Shift, Score)),
}

impl Default for ScoreProp {
    fn default() -> Self {
        ScoreProp::PatternGeneral((Default::default(), Default::default(), Default::default()))
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

impl ScorePropTrait<Shift, ShiftState, DayState> for ScoreProp {
    fn eval_mut(
        &mut self,
        schedule_config: &kinmu_model::ScheduleConfig<Self, Shift, ShiftState, DayState>,
        schedule: &kinmu_model::Schedule<Shift>,
    ) -> Score {
        eval_score_mut(self, schedule_config, schedule)
    }

    fn eval_immut(
        &self,
        schedule_config: &kinmu_model::ScheduleConfig<Self, Shift, ShiftState, DayState>,
        schedule: &kinmu_model::Schedule<Shift>,
    ) -> Score {
        eval_score_immut(self, schedule_config, schedule)
    }
}

impl Check<Shift, ShiftState, DayState> for ScoreProp {
    fn check(
        &self,
        schedule_config: &kinmu_model::ScheduleConfig<Self, Shift, ShiftState, DayState>,
    ) -> anyhow::Result<()> {
        match self {
            ScoreProp::PatternGeneral((c, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::PatternFixed((c, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::PatternGeneralAny((c, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::PatternFixedAny((c, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::Streak((c, _, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::ShiftsBalance((c, _, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::ShiftHalfBalance((c, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::ShiftDirPriority((c, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::DayCountRegardStaffAttribute((c, _, sa, _)) => {
                check_cond_wrapper(c, schedule_config)
                    .and(check_staff_attribute_exists(sa, schedule_config))
            }
            ScoreProp::StaffCountRegardDayAttribute((c, _, da, _)) => {
                check_cond_wrapper(c, schedule_config)
                    .and(check_day_attribute_exists(da, schedule_config))
            }
            ScoreProp::StaffCount((c, _, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::StaffCountAtLeast((c, _, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::StaffCountWithPremise((c1, _, _, c2, _, _, _)) => {
                check_cond_wrapper(c1, schedule_config).and(check_cond_wrapper(c2, schedule_config))
            }
            ScoreProp::NGPair((c, _, _)) => check_cond_wrapper(c, schedule_config),
            ScoreProp::NoSamePair((c, _, _, _)) => check_cond_wrapper(c, schedule_config),
        }
        .with_context(|| format!("スコア {:?} の変換チェックに失敗しました", self))?;

        Ok(())
    }
}

/// CondWrapperの中のStaffAttributeNameやDayAttributeNameが有効か
fn check_cond_wrapper(
    c: &CondWrapper,
    sc: &ScheduleConfig<ScoreProp, Shift, ShiftState, DayState>,
) -> anyhow::Result<()> {
    check_cond(&c.cond, sc)
        .with_context(|| format!("CondWrapper {:?} の変換チェックに失敗しました", &c.cond))?;
    Ok(())
}

/// Condの中のStaffAttributeNameやDayAttributeNameが有効か
fn check_cond(
    c: &Cond,
    sc: &ScheduleConfig<ScoreProp, Shift, ShiftState, DayState>,
) -> anyhow::Result<()> {
    match c {
        Cond::Every => Ok(()),
        Cond::Or((c1, c2)) => check_cond(c1, sc).and(check_cond(c2, sc)),
        Cond::And((c1, c2)) => check_cond(c1, sc).and(check_cond(c2, sc)),
        Cond::Not(c) => check_cond(c, sc),

        Cond::DayExceptBuffer => Ok(()),
        Cond::DayInRange(_) => Ok(()),
        Cond::ParticularDayState(_) => Ok(()),
        Cond::BeforeDayState(_) => Ok(()),
        Cond::ParticularDay(_) => Ok(()),

        Cond::StaffInRange(_) => Ok(()),
        Cond::StaffWithAttribute((sa, _)) => check_staff_attribute_exists(sa, sc),
        Cond::ParticularStaff(_) => Ok(()),
    }
    .with_context(|| format!("Cond {:?} の変換チェックに失敗しました", c))?;
    Ok(())
}

/// StaffAttributeNameが有効か
fn check_staff_attribute_exists(
    sa: &StaffAttributeName,
    sc: &ScheduleConfig<ScoreProp, Shift, ShiftState, DayState>,
) -> anyhow::Result<()> {
    if sc.staff.attribute_map.names.contains(sa) {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "{} はstaff.attributesに登録されていません",
            sa
        ))
    }
}

/// DayAttributeNameが有効か
fn check_day_attribute_exists(
    da: &DayAttributeName,
    sc: &ScheduleConfig<ScoreProp, Shift, ShiftState, DayState>,
) -> anyhow::Result<()> {
    if sc.day.attributes.contains_key(da) {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "{} はday.attributesに登録されていません",
            da
        ))
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

#[inline(always)]
fn helper_sp(w1: &str, w2: &str) -> anyhow::Result<ScoreProp> {
    match (w1, w2) {
        ("PatternGeneral", p) => Ok(ScoreProp::PatternGeneral({
            let (cw, VecVecShiftWrapper(vvs), s) =
                <(CondWrapper, VecVecShiftWrapper, Score)>::from_config(p)?;
            (cw, vvs, s)
        })),
        ("PatternFixed", p) => Ok(ScoreProp::PatternFixed({
            let (cw, VecShiftWrapper(vs), s) =
                <(CondWrapper, VecShiftWrapper, Score)>::from_config(p)?;
            (cw, vs, s)
        })),
        ("PatternGeneralAny", p) => Ok(ScoreProp::PatternGeneralAny({
            let (cw, VecVecShiftWrapper(vvs), s) =
                <(CondWrapper, VecVecShiftWrapper, Score)>::from_config(p)?;
            (cw, vvs, s)
        })),
        ("PatternFixedAny", p) => Ok(ScoreProp::PatternFixedAny({
            let (cw, VecShiftWrapper(vs), s) =
                <(CondWrapper, VecShiftWrapper, Score)>::from_config(p)?;
            (cw, vs, s)
        })),
        ("Streak", p) => Ok(ScoreProp::Streak({
            let (cw, VecShiftWrapper(vs), i, s) =
                <(CondWrapper, VecShiftWrapper, i32, Score)>::from_config(p)?;
            (cw, vs, i, s)
        })),
        ("ShiftsBalance", p) => Ok(ScoreProp::ShiftsBalance(<(
            CondWrapper,
            Shift,
            Shift,
            Score,
        )>::from_config(p)?)),
        ("ShiftHalfBalance", p) => Ok(ScoreProp::ShiftHalfBalance(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        )),
        ("ShiftDirPriority", p) => Ok(ScoreProp::ShiftDirPriority(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        )),
        ("DayCountRegardStaffAttribute", p) => Ok(ScoreProp::DayCountRegardStaffAttribute(
            <(CondWrapper, Shift, StaffAttributeName, Score)>::from_config(p)?,
        )),
        ("StaffCountRegardDayAttribute", p) => Ok(ScoreProp::StaffCountRegardDayAttribute(
            <(CondWrapper, Shift, DayAttributeName, Score)>::from_config(p)?,
        )),
        ("StaffCount", p) => Ok(ScoreProp::StaffCount(
            <(CondWrapper, Shift, i32, Score)>::from_config(p)?,
        )),
        ("StaffCountAtLeast", p) => Ok(ScoreProp::StaffCountAtLeast(<(
            CondWrapper,
            Shift,
            i32,
            Score,
        )>::from_config(p)?)),
        ("StaffCountWithPremise", p) => Ok(ScoreProp::StaffCountWithPremise(<(
            CondWrapper,
            Shift,
            i32,
            CondWrapper,
            Shift,
            i32,
            Score,
        )>::from_config(
            p
        )?)),
        ("NGPair", p) => Ok(ScoreProp::NGPair(
            <(CondWrapper, Shift, Score)>::from_config(p)?,
        )),
        ("NoSamePair", p) => Ok(ScoreProp::NoSamePair(
            <(CondWrapper, i32, Shift, Score)>::from_config(p)?,
        )),
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
