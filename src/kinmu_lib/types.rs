//! 勤務表に使う型の宣言

use std::str::FromStr;
use std::fmt;
use rand::RngCore;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Copy)]
pub enum Shift {
    N,
    K,
    I,
    A,
    O,
    H,
    Y,
    D,
    U,
}

impl FromStr for Shift {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Shift::N),
            "K" => Ok(Shift::K),
            "I" => Ok(Shift::I),
            "A" => Ok(Shift::A),
            "O" => Ok(Shift::O),
            "H" => Ok(Shift::H),
            "Y" => Ok(Shift::Y),
            "D" => Ok(Shift::D),
            "U" => Ok(Shift::U),
            " " => Ok(Shift::U),
            _ => Err(format!("Failed to parse Shift: {}", s))
        }
    }
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Shift::N => "N",
            Shift::K => "K",
            Shift::I => "I",
            Shift::A => "A",
            Shift::O => "O",
            Shift::H => "H",
            Shift::Y => "Y",
            Shift::D => "D",
            Shift::U => "U",
        };
        write!(f, "{}", s)
    }
}

pub type Schedule = Vec<Vec<Shift>>;

pub type Score = f32;

#[derive(PartialEq)]
pub enum ShiftState {
    Absolute,
    Random,
}

pub type ScheduleState = Vec<Vec<ShiftState>>;

pub struct Staff {
    pub name: String,
    pub ability: isize,
    pub k_day_count: isize,
    pub i_day_count: isize,
    pub o_day_count: isize,
    pub h_day_count: isize,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DayState {
    Weekday,
    Holiday,
    Bath,
    Bath2,
    Measure,
}

impl fmt::Display for DayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            DayState::Weekday => "W",
            DayState::Holiday => "H",
            DayState::Bath => "B",
            DayState::Bath2 => "2",
            DayState::Measure => "M",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for DayState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(DayState::Weekday),
            "H" => Ok(DayState::Holiday),
            "B" => Ok(DayState::Bath),
            "2" => Ok(DayState::Bath2),
            "M" => Ok(DayState::Measure),
            _ => Err(format!("Failed to parse DayState: {}", s))
        }
    }
}

pub type Days = Vec<DayState>;

pub type NG = (usize, usize);

pub type NGList = Vec<NG>;

pub enum ScoreProp {
    IAKpattern(Score),
    KIApattern(Score),
    KNIApattern(Score),
    NNIApattern(Score),
    ONpattern(Score),
    NHpattern(Score),
    OHpattern(Score),
    WorkingDayStreak4((Score, Score)),
    WorkingDayStreak5((Score, Score)),
    WorkingDayStreak6((Score, Score)),
    HolidayReward(Score),
    Need2Holidays(Score),
    Need2HolidaysNoBf(Score),
    OHBalance(Score),
    ShiftHalfBalance((Shift,Score)),
    ShiftDirPriority((Shift,Score)),
    KDayCount(Score),
    IDayCount(Score),
    ODayCount(Score),
    HDayCount(Score),
    IStaffCount(Score),
    NStaffCount((DayState,isize,Score)),
    OStaffCount((isize,Score)),
    HStaffCount((isize,Score)),
    NGPair(Score),
    LeaderAbility((isize,Score)),
    IAloneAbility((isize,Score)),
    IAloneBeforeBath(Score),
    NStaffCountWithAbility((isize,isize,Score)),
    NoSamePair3(Score),
    NoSamePair2(Score),
    NoUndef(Score),
}

impl fmt::Display for ScoreProp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ScoreProp::IAKpattern(p) => format!("IAKpattern {:?}", p),
            ScoreProp::KIApattern(p) => format!("KIApattern {:?}", p),
            ScoreProp::KNIApattern(p) => format!("KNIApattern {:?}", p),
            ScoreProp::NNIApattern(p) => format!("NNIApattern {:?}", p),
            ScoreProp::ONpattern(p) => format!("ONpattern {:?}", p),
            ScoreProp::NHpattern(p) => format!("NHpattern {:?}", p),
            ScoreProp::OHpattern(p) => format!("OHpattern {:?}", p),
            ScoreProp::WorkingDayStreak4(p) => format!("WorkingDayStreak4 {:?}", p),
            ScoreProp::WorkingDayStreak5(p) => format!("WorkingDayStreak5 {:?}", p),
            ScoreProp::WorkingDayStreak6(p) => format!("WorkingDayStreak6 {:?}", p),
            ScoreProp::HolidayReward(p) => format!("HolidayReward {:?}", p),
            ScoreProp::Need2Holidays(p) => format!("Need2Holidays {:?}", p),
            ScoreProp::Need2HolidaysNoBf(p) => format!("Need2HolidaysNoBf {:?}", p),
            ScoreProp::OHBalance(p) => format!("OHBalance {:?}", p),
            ScoreProp::ShiftHalfBalance(p) => format!("ShiftHalfBalance {:?}", p),
            ScoreProp::ShiftDirPriority(p) => format!("ShiftDirPriority {:?}", p),
            ScoreProp::KDayCount(p) => format!("KDayCount {:?}", p),
            ScoreProp::IDayCount(p) => format!("IDayCount {:?}", p),
            ScoreProp::ODayCount(p) => format!("ODayCount {:?}", p),
            ScoreProp::HDayCount(p) => format!("HDayCount {:?}", p),
            ScoreProp::IStaffCount(p) => format!("IStaffCount {:?}", p),
            ScoreProp::NStaffCount(p) => format!("NStaffCount {:?}", p),
            ScoreProp::OStaffCount(p) => format!("OStaffCount {:?}", p),
            ScoreProp::HStaffCount(p) => format!("HStaffCount {:?}", p),
            ScoreProp::NGPair(p) => format!("NGPair {:?}", p),
            ScoreProp::LeaderAbility(p) => format!("LeaderAbility {:?}", p),
            ScoreProp::IAloneAbility(p) => format!("IAloneAbility {:?}", p),
            ScoreProp::IAloneBeforeBath(p) => format!("IAloneBeforeBath {:?}", p),
            ScoreProp::NStaffCountWithAbility(p) => format!("NStaffCountWithAbility {:?}", p),
            ScoreProp::NoSamePair3(p) => format!("NoSamePair3 {:?}", p),
            ScoreProp::NoSamePair2(p) => format!("NoSamePair2 {:?}", p),
            ScoreProp::NoUndef(p) => format!("NoUndef {:?}", p),
        };
        write!(f, "{}", s)
    }
}


/// 勤務表ごとの設定
pub struct ScheduleProp {
    pub staff_list: Vec<Staff>,
    pub ng_list: NGList,
    pub staff_count: usize,
    pub day_count: usize,
    pub days: Days,
    pub buffer: usize,
    pub request: Schedule,
    pub schedule_st: ScheduleState,
    pub i_staff_count: Vec<isize>,
    pub score_props: Vec<ScoreProp>, // 結果表示のためのスコア
}

pub struct FillConfig {
    pub name: String,
    pub rng: Box<dyn RngCore>,
}

/// 焼きなましの段階ごとの設定
pub struct AnnealingConfig {
    pub step: usize, // 焼きなましのステップ数
    pub rng: Box<dyn RngCore>, // 焼きなましのupdate関数の乱数生成器
    pub score_props: Vec<ScoreProp>, // 焼きなましのためのスコア
    pub update_func: String,
    pub max_temp: f32,
    pub min_temp: f32,
}