//! Cond型の定義と実装

use super::{
    DayConfig, DayState, ScheduleConfig, Shift, ShiftState, StaffAttributeNameWrapper, StdScoreProp,
};

use kinmu_input_by_file::{Check, FromConfig, VecWrapper};
use kinmu_model::{StaffAttributeName, StaffConfig};

use anyhow::Context as _;
use std::fmt;

/// ScorePropに用いる条件を管理する型
/// tupleを使うのはFromConfigの実装のため
#[derive(Debug, PartialEq, Clone, Default)]
pub enum Cond {
    // 基本条件
    #[default]
    True,
    False,
    Not(Box<Cond>),
    Or((Box<Cond>, Box<Cond>)),
    And((Box<Cond>, Box<Cond>)),
    Any(Vec<Cond>),
    All(Vec<Cond>),

    // 日付についての条件
    Day(usize),
    DayInRange((usize, usize)),
    DayInList(Vec<usize>),
    NoBuffer,
    DayState(DayState),
    BeforeDayState(DayState),

    // スタッフについての条件
    Staff(usize),
    StaffInRange((usize, usize)),
    StaffWithAttribute((StaffAttributeName, i32)),
    StaffNamed(String),
}

impl Cond {
    pub fn eval(&self, staff: usize, day: usize, sc: &StaffConfig, dc: &DayConfig) -> bool {
        match self {
            Cond::True => true,
            Cond::False => false,
            Cond::Not(cond) => !cond.eval(staff, day, sc, dc),
            Cond::Or((cond1, cond2)) => {
                cond1.eval(staff, day, sc, dc) || cond2.eval(staff, day, sc, dc)
            }
            Cond::And((cond1, cond2)) => {
                cond1.eval(staff, day, sc, dc) && cond2.eval(staff, day, sc, dc)
            }
            Cond::Any(cs) => cs.iter().any(|c| c.eval(staff, day, sc, dc)),
            Cond::All(cs) => cs.iter().all(|c| c.eval(staff, day, sc, dc)),

            Cond::Day(d) => {
                // dはbufferを除いて1-indexedになっている
                // このためbufferを含めた0-indexedに戻して比較
                let di = *d + dc.buffer_count - 1;
                di == day
            }
            Cond::DayInRange((day_start, day_end)) => {
                // day_start, day_endはbufferを除いて1-indexedになっている
                // このためbufferを含めた0-indexedに戻して比較
                let di_start = *day_start + dc.buffer_count - 1;
                let di_end = *day_end + dc.buffer_count - 1;
                di_start <= day && day <= di_end
            }
            Cond::DayInList(ds) => ds.iter().any(|d| {
                // dはbufferを除いて1-indexedになっている
                // このためbufferを含めた0-indexedに戻して比較
                let di = *d + dc.buffer_count - 1;
                di == day
            }),
            Cond::NoBuffer => dc.buffer_count <= day,
            Cond::DayState(ds) => dc.days[day] == *ds,
            Cond::BeforeDayState(ds) => {
                if day + 1 >= dc.count {
                    false
                } else {
                    dc.days[day + 1] == *ds
                }
            }

            Cond::Staff(s) => *s == staff,
            Cond::StaffInRange((staff_start, staff_end)) => {
                *staff_start <= staff && staff <= *staff_end
            }
            Cond::StaffWithAttribute((attribute, value)) => {
                sc.get_attribute(staff, attribute) == *value
            }
            Cond::StaffNamed(s) => *s == sc.list[staff].name,
        }
    }

    pub fn eval_day(&self, day: usize, _sc: &StaffConfig, dc: &DayConfig) -> Option<bool> {
        match self {
            Cond::True => Some(true),
            Cond::False => Some(false),
            Cond::Not(cond) => Some(!cond.eval_day(day, _sc, dc)?),
            Cond::Or((cond1, cond2)) => {
                Some(cond1.eval_day(day, _sc, dc)? || cond2.eval_day(day, _sc, dc)?)
            }
            Cond::And((cond1, cond2)) => {
                Some(cond1.eval_day(day, _sc, dc)? && cond2.eval_day(day, _sc, dc)?)
            }
            Cond::Any(cs) => {
                let mut b = false;
                for c in cs {
                    b |= c.eval_day(day, _sc, dc)?;
                }
                Some(b)
            }
            Cond::All(cs) => {
                let mut b = true;
                for c in cs {
                    b &= c.eval_day(day, _sc, dc)?;
                }
                Some(b)
            }

            Cond::Day(d) => {
                // dはbufferを除いて1-indexedになっている
                // このためbufferを含めた0-indexedに戻して比較
                let di = *d + dc.buffer_count - 1;
                Some(di == day)
            }
            Cond::DayInRange((day_start, day_end)) => {
                // day_start, day_endはbufferを除いて1-indexedになっている
                // このためbufferを含めた0-indexedに戻して比較
                let di_start = *day_start + dc.buffer_count - 1;
                let di_end = *day_end + dc.buffer_count - 1;
                Some(di_start <= day && day <= di_end)
            }
            Cond::DayInList(ds) => Some(ds.iter().any(|d| {
                // dはbufferを除いて1-indexedになっている
                // このためbufferを含めた0-indexedに戻して比較
                let di = *d + dc.buffer_count - 1;
                di == day
            })),
            Cond::NoBuffer => Some(dc.buffer_count <= day),
            Cond::DayState(ds) => Some(dc.days[day] == *ds),
            Cond::BeforeDayState(ds) => {
                if day + 1 >= dc.count {
                    Some(false)
                } else {
                    Some(dc.days[day + 1] == *ds)
                }
            }

            Cond::Staff(_s) => None,
            Cond::StaffInRange((_staff_start, _staff_end)) => None,
            Cond::StaffWithAttribute((_attribute, _value)) => None,
            Cond::StaffNamed(_s) => None,
        }
    }

    pub fn eval_staff(&self, staff: usize, sc: &StaffConfig, _dc: &DayConfig) -> Option<bool> {
        match self {
            Cond::True => Some(true),
            Cond::False => Some(false),
            Cond::Not(cond) => Some(!cond.eval_staff(staff, sc, _dc)?),
            Cond::Or((cond1, cond2)) => {
                Some(cond1.eval_staff(staff, sc, _dc)? || cond2.eval_staff(staff, sc, _dc)?)
            }
            Cond::And((cond1, cond2)) => {
                Some(cond1.eval_staff(staff, sc, _dc)? && cond2.eval_staff(staff, sc, _dc)?)
            }
            Cond::Any(cs) => {
                let mut b = false;
                for c in cs {
                    b |= c.eval_staff(staff, sc, _dc)?;
                }
                Some(b)
            }
            Cond::All(cs) => {
                let mut b = true;
                for c in cs {
                    b &= c.eval_staff(staff, sc, _dc)?;
                }
                Some(b)
            }

            Cond::Day(_d) => None,
            Cond::DayInRange((_day_start, _day_end)) => None,
            Cond::DayInList(_ds) => None,
            Cond::NoBuffer => None,
            Cond::DayState(_ds) => None,
            Cond::BeforeDayState(_ds) => None,

            Cond::Staff(s) => Some(*s == staff),
            Cond::StaffInRange((staff_start, staff_end)) => {
                Some(*staff_start <= staff && staff <= *staff_end)
            }
            Cond::StaffWithAttribute((attribute, value)) => {
                Some(sc.get_attribute(staff, attribute) == *value)
            }
            Cond::StaffNamed(s) => Some(*s == sc.list[staff].name),
        }
    }
}

/// Condをメモ化して高速化するためのラッパー
#[derive(PartialEq, Clone, Default)]
pub struct CondWrapper {
    pub cond: Cond,
    memo: Vec<Vec<Option<bool>>>,
    day_memo: Vec<Option<bool>>,
    staff_memo: Vec<Option<bool>>,
}

impl CondWrapper {
    pub fn new(cond: Cond) -> Self {
        CondWrapper {
            cond,
            memo: <Vec<Vec<Option<bool>>>>::new(),
            day_memo: <Vec<Option<bool>>>::new(),
            staff_memo: <Vec<Option<bool>>>::new(),
        }
    }

    /// メモ化したCondの評価
    /// ScheduleConfigが焼きなましの過程で変化しない制限の上で
    pub fn eval_mut(&mut self, staff: usize, day: usize, sc: &StaffConfig, dc: &DayConfig) -> bool {
        if self.memo.is_empty() {
            self.memo = vec![vec![None; dc.count]; sc.count];
        }
        match self.memo[staff][day] {
            Some(ans) => ans,
            None => {
                let ans = self.cond.eval(staff, day, sc, dc);
                self.memo[staff][day] = Some(ans);
                ans
            }
        }
    }

    /// メモに記入しないCondの評価
    /// ただし、メモが記入されている場合は利用する
    pub fn eval_immut(&self, staff: usize, day: usize, sc: &StaffConfig, dc: &DayConfig) -> bool {
        if self.memo.is_empty() {
            return self.cond.eval(staff, day, sc, dc);
        }
        match self.memo[staff][day] {
            Some(ans) => ans,
            None => self.cond.eval(staff, day, sc, dc),
        }
    }

    /// メモを参照しないCondの評価
    pub fn eval_anyway(&self, staff: usize, day: usize, sc: &StaffConfig, dc: &DayConfig) -> bool {
        self.cond.eval(staff, day, sc, dc)
    }
}

impl fmt::Debug for CondWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cond)
    }
}

impl FromConfig for Cond {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words: Vec<&str> = s.splitn(2, ' ').collect();
        anyhow::ensure!(words.len() >= 2, "Needs 2 fields, but not enough.");
        anyhow::ensure!(2 >= words.len(), "Needs 2 fields, but too much given.");
        match (words[0], words[1]) {
            ("True", _) => Ok(Cond::True),
            ("False", _) => Ok(Cond::False),
            ("Not", p) => Ok(Cond::Not(Box::new(<Cond>::from_config(p)?))),
            ("Or", p) => Ok(Cond::Or(<(Box<Cond>, Box<Cond>)>::from_config(p)?)),
            ("And", p) => Ok(Cond::And(<(Box<Cond>, Box<Cond>)>::from_config(p)?)),
            ("Any", p) => Ok(Cond::Any(<VecWrapper<Cond>>::from_config(p)?.0)),
            ("All", p) => Ok(Cond::All(<VecWrapper<Cond>>::from_config(p)?.0)),

            ("Day", p) => Ok(Cond::Day(<usize>::from_config(p)?)),
            ("DayInRange", p) => Ok(Cond::DayInRange(<(usize, usize)>::from_config(p)?)),
            ("DayInList", p) => Ok(Cond::DayInList(<VecWrapper<usize>>::from_config(p)?.0)),
            ("NoBuffer", _) => Ok(Cond::NoBuffer),
            ("DayState", p) => Ok(Cond::DayState(<DayState>::from_config(p)?)),
            ("BeforeDayState", p) => Ok(Cond::BeforeDayState(<DayState>::from_config(p)?)),

            ("Staff", p) => Ok(Cond::Staff(<usize>::from_config(p)?)),
            ("StaffInRange", p) => Ok(Cond::StaffInRange(<(usize, usize)>::from_config(p)?)),
            ("StaffWithAttribute", p) => Ok(Cond::StaffWithAttribute(
                <(StaffAttributeName, i32)>::from_config(p)?,
            )),
            ("StaffNamed", p) => Ok(Cond::StaffNamed(String::from_config(p)?)),
            (s, p) => Err(anyhow::anyhow!("Failed to parse Cond: {} {}", s, p)),
        }
    }
}

impl FromConfig for Box<Cond> {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let cond = Cond::from_config(s)?;
        Ok(Box::new(cond))
    }
}

impl FromConfig for CondWrapper {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let cond = Cond::from_config(s)?;
        Ok(CondWrapper::new(cond))
    }
}

impl Check<StdScoreProp, Shift, ShiftState, DayState> for Cond {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        match self {
            Cond::True => Ok(()),
            Cond::False => Ok(()),
            Cond::Not(c) => c.check(schedule_config),
            Cond::Or((c1, c2)) => c1.check(schedule_config).and(c2.check(schedule_config)),
            Cond::And((c1, c2)) => c1.check(schedule_config).and(c2.check(schedule_config)),
            Cond::Any(cs) => cs.iter().try_for_each(|_| Ok(())),
            Cond::All(cs) => cs.iter().try_for_each(|_| Ok(())),

            Cond::Day(_) => Ok(()),
            Cond::DayInRange(_) => Ok(()),
            Cond::DayInList(_) => Ok(()),
            Cond::NoBuffer => Ok(()),
            Cond::DayState(_) => Ok(()),
            Cond::BeforeDayState(_) => Ok(()),

            Cond::Staff(_) => Ok(()),
            Cond::StaffInRange(_) => Ok(()),
            Cond::StaffWithAttribute((sa, _)) => {
                StaffAttributeNameWrapper(sa).check(schedule_config)
            }
            Cond::StaffNamed(_) => Ok(()),
        }
        .with_context(|| format!("Cond {:?} の変換チェックに失敗しました", self))?;
        Ok(())
    }
}

impl Check<StdScoreProp, Shift, ShiftState, DayState> for CondWrapper {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        self.cond.check(schedule_config).with_context(|| {
            format!("CondWrapper {:?} の変換チェックに失敗しました", &self.cond)
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use kinmu_model::{Staff, StaffAttributeNameIndexMap};

    use super::*;

    /// 表の全ての判定結果が同じになるようなCondをテストする
    /// propsはstaff_count, day_count, staff_config, day_configのタプル
    /// propsは可読性のためにまとめた
    macro_rules! test_all {
        ($ans:expr,$cond:expr,$props:expr) => {
            for s in 0..$props.0 {
                for d in 0..$props.1 {
                    assert_eq!($ans, $cond.eval(s, d, &$props.2, &$props.3));
                }
            }
        };
    }

    /// 基本的なCondのテスト
    #[test]
    fn test_basic_cond() {
        let staff_count: usize = 2;
        let day_count: usize = 2;
        let sc = StaffConfig {
            attribute_map: StaffAttributeNameIndexMap {
                names: vec![String::from("職員A")],
                name_to_index: HashMap::from([(String::from("職員A"), 0)]),
            },
            list: vec![Staff {
                name: String::from("職員A"),
                attributes: vec![],
            }],
            ng_list: vec![],
            count: 1,
        };
        let dc = DayConfig {
            count: 2,
            buffer_count: 0,
            days: vec![DayState::Holiday, DayState::Weekday],
            requested_schedule: Default::default(),
            schedule_states: Default::default(),
            attributes: HashMap::new(),
        };
        let props = (staff_count, day_count, sc, dc);

        // Trueのテスト
        test_all!(true, Cond::True, props);

        // Falseのテスト
        test_all!(false, Cond::False, props);

        // Notのテスト
        test_all!(false, Cond::Not(Box::new(Cond::True)), props);
        test_all!(true, Cond::Not(Box::new(Cond::False)), props);

        // Orのテスト
        test_all!(
            true,
            Cond::Or((Box::new(Cond::False), Box::new(Cond::True))),
            props
        );
        test_all!(
            false,
            Cond::Or((Box::new(Cond::False), Box::new(Cond::False))),
            props
        );

        // Andのテスト
        test_all!(
            false,
            Cond::And((Box::new(Cond::False), Box::new(Cond::False))),
            props
        );
        test_all!(
            false,
            Cond::And((Box::new(Cond::True), Box::new(Cond::False))),
            props
        );
        test_all!(
            true,
            Cond::And((Box::new(Cond::True), Box::new(Cond::True))),
            props
        );

        // Anyのテスト
        test_all!(false, Cond::Any(vec![]), props);
        test_all!(false, Cond::Any(vec![Cond::False]), props);
        test_all!(true, Cond::Any(vec![Cond::True]), props);
        test_all!(
            true,
            Cond::Any(vec![Cond::False, Cond::True, Cond::False]),
            props
        );
        test_all!(
            false,
            Cond::Any(vec![Cond::False, Cond::False, Cond::False]),
            props
        );

        // Allのテスト
        test_all!(true, Cond::All(vec![]), props);
        test_all!(false, Cond::All(vec![Cond::False]), props);
        test_all!(true, Cond::All(vec![Cond::True]), props);
        test_all!(
            true,
            Cond::All(vec![Cond::True, Cond::True, Cond::True]),
            props
        );
        test_all!(
            false,
            Cond::All(vec![Cond::False, Cond::True, Cond::False]),
            props
        );
    }

    /// 最初の職員についてCondの結果が与えられたリストと一致するかテストする
    /// 日付に依存するCondのテストのため
    macro_rules! test_first_staff {
        ($ans:expr,$cond:expr,$sc:expr,$dc:expr) => {
            for d in 0..$dc.count {
                assert_eq!($ans[d], $cond.eval(0, d, &$sc, &$dc));
            }
        };
    }

    /// 日付に関するCondのテスト
    #[test]
    fn test_day_conds() {
        let staff_count: usize = 1;
        let day_count: usize = 6;
        let sc = StaffConfig {
            attribute_map: Default::default(),
            list: Default::default(),
            ng_list: Default::default(),
            count: staff_count,
        };
        let dc = DayConfig {
            count: day_count,
            buffer_count: 3,
            days: Default::default(),
            requested_schedule: Default::default(),
            schedule_states: Default::default(),
            attributes: HashMap::new(),
        };

        // Dayがbufferを除いて1-indexedになっている
        test_first_staff!(
            [false, false, false, false, true, false],
            Cond::Day(2),
            sc,
            dc
        );

        // DayInRangeがbufferを除いて1-indexedになっている
        test_first_staff!(
            [false, false, false, true, true, false],
            Cond::DayInRange((1, 2)),
            sc,
            dc
        );

        // DayInListがbufferを除いて1-indexedになっている
        test_first_staff!(
            [false, false, false, true, false, true],
            Cond::DayInList(vec![1, 3]),
            sc,
            dc
        );
    }
}
