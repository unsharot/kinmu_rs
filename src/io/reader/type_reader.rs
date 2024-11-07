use crate::kinmu_lib::types::{
    Cond, DayAttributeName, DayState, Days, NGList, Schedule, Score, ScoreProp, Shift, Staff,
    StaffAttributeName, NG,
};

use super::common::check_len;

pub trait MyFromStr: Sized {
    fn my_from_str(s: &str) -> Result<Self, String>;
}

impl<T, U> MyFromStr for (T, U)
where
    T: MyFromStr,
    U: MyFromStr,
{
    fn my_from_str(s: &str) -> Result<Self, String> {
        let words: Vec<_> = s
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();
        check_len(
            2,
            &words,
            "Needs 2 fields, but not enough.",
            "Needs 2 fields, but too much given.",
        )?;
        let t = T::my_from_str(words[0]).map_err(|e| e.to_string())?;
        let u = U::my_from_str(words[1]).map_err(|e| e.to_string())?;
        Ok((t, u))
    }
}

impl<T, U, V> MyFromStr for (T, U, V)
where
    T: MyFromStr,
    U: MyFromStr,
    V: MyFromStr,
{
    fn my_from_str(s: &str) -> Result<Self, String> {
        let words: Vec<_> = s
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();
        check_len(
            3,
            &words,
            "Needs 3 fields, but not enough.",
            "Needs 3 fields, but too much given.",
        )?;
        let t = T::my_from_str(words[0]).map_err(|e| e.to_string())?;
        let u = U::my_from_str(words[1]).map_err(|e| e.to_string())?;
        let v = V::my_from_str(words[2]).map_err(|e| e.to_string())?;
        Ok((t, u, v))
    }
}

impl<T, U, V, W> MyFromStr for (T, U, V, W)
where
    T: MyFromStr,
    U: MyFromStr,
    V: MyFromStr,
    W: MyFromStr,
{
    fn my_from_str(s: &str) -> Result<Self, String> {
        let words: Vec<_> = s
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();
        check_len(
            4,
            &words,
            "Needs 4 fields, but not enough.",
            "Needs 4 fields, but too much given.",
        )?;
        let t = T::my_from_str(words[0]).map_err(|e| e.to_string())?;
        let u = U::my_from_str(words[1]).map_err(|e| e.to_string())?;
        let v = V::my_from_str(words[2]).map_err(|e| e.to_string())?;
        let w = W::my_from_str(words[3]).map_err(|e| e.to_string())?;
        Ok((t, u, v, w))
    }
}

impl MyFromStr for usize {
    fn my_from_str(s: &str) -> Result<Self, String> {
        Ok(s.parse::<usize>().map_err(|e| e.to_string())?)
    }
}

impl MyFromStr for isize {
    fn my_from_str(s: &str) -> Result<Self, String> {
        Ok(s.parse::<isize>().map_err(|e| e.to_string())?)
    }
}

pub fn read_isizes(text: &str) -> Result<Vec<isize>, String> {
    text.split_whitespace()
        .map(|x| x.parse::<isize>().map_err(|e| e.to_string()))
        .collect()
}

impl MyFromStr for f32 {
    fn my_from_str(s: &str) -> Result<Self, String> {
        Ok(s.parse::<f32>().map_err(|e| e.to_string())?)
    }
}

impl MyFromStr for Shift {
    fn my_from_str(s: &str) -> Result<Self, String> {
        Ok(s.parse::<Shift>().map_err(|e| e.to_string())?)
    }
}

pub fn read_daystate_isize_float(text: &str) -> Result<(DayState, isize, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(
        3,
        &words,
        "Needs 3 fields, but not enough.",
        "Needs 3 fields, but too much given.",
    )?;
    let d = words[0].parse::<DayState>().map_err(|e| e.to_string())?;
    let i = words[1].parse::<isize>().map_err(|e| e.to_string())?;
    let f = words[2].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((d, i, f))
}

pub fn read_staff_list(text: &str) -> Result<Vec<Staff>, String> {
    let mut staff: Vec<Staff> = Vec::new();
    for line in text.lines() {
        let a_staff = read_a_staff(&line)?;
        staff.push(a_staff);
    }
    Ok(staff)
}

pub fn read_a_staff(text: &str) -> Result<Staff, String> {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    check_len(
        6,
        &words,
        "Needs 6 fields, but not enough.",
        "Needs 6 fields, but too much given.",
    )?;
    let worker: Staff = Staff {
        name: words[5].clone(),
        ability: <isize>::my_from_str(&words[0])?,
        k_day_count: <isize>::my_from_str(&words[1])?,
        i_day_count: <isize>::my_from_str(&words[2])?,
        o_day_count: <isize>::my_from_str(&words[3])?,
        h_day_count: <isize>::my_from_str(&words[4])?,
    };
    Ok(worker)
}

pub fn read_ng_list(text: &str) -> Result<NGList, String> {
    let mut ans: NGList = Vec::new();
    for line in text.lines() {
        let ng = read_ng(&line)?;
        ans.push(ng);
    }
    Ok(ans)
}

pub fn read_ng(text: &str) -> Result<NG, String> {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    check_len(
        2,
        &words,
        "Needs 2 fields, but not enough.",
        "Needs 2 fields, but too much given.",
    )?;
    let id1 = <usize>::my_from_str(&words[0])?;
    let id2 = <usize>::my_from_str(&words[1])?;
    Ok((id1, id2))
}

pub fn read_temp(text: &str) -> Result<(f32, f32), String> {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    check_len(
        2,
        &words,
        "Needs 2 fields, but not enough.",
        "Needs 2 fields, but too much given.",
    )?;
    let id1 = <f32>::my_from_str(&words[0])?;
    let id2 = <f32>::my_from_str(&words[1])?;
    Ok((id1, id2))
}

pub fn read_days(text: &str) -> Result<Days, String> {
    let mut ans: Days = Vec::new();
    for c in text.chars() {
        ans.push(c.to_string().parse::<DayState>()?);
    }
    Ok(ans)
}

pub fn read_schedule(text: &str) -> Result<Schedule, String> {
    let mut ans: Schedule = Vec::new();
    for line in text.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_string().parse::<Shift>()?);
        }
        ans.push(row);
    }
    Ok(ans)
}

impl MyFromStr for DayState {
    fn my_from_str(s: &str) -> Result<Self, String> {
        Ok(s.parse::<DayState>().map_err(|e| e.to_string())?)
    }
}

impl MyFromStr for Cond {
    fn my_from_str(s: &str) -> Result<Self, String> {
        let words: Vec<&str> = s.split_whitespace().collect();
        check_len(
            2,
            &words,
            "Needs 2 fields, but not enough.",
            "Needs 2 fields, but too much given.",
        )?;
        match (words[0], words[1]) {
            ("Every", _) => Ok(Cond::Every),
            ("Or", p) => Ok(Cond::Or(<(Box<Cond>, Box<Cond>)>::my_from_str(p)?)),
            ("And", p) => Ok(Cond::And(<(Box<Cond>, Box<Cond>)>::my_from_str(p)?)),
            ("Not", p) => Ok(Cond::Not(Box::new(<Cond>::my_from_str(p)?))),
            ("DayExceptBuffer", _) => Ok(Cond::DayExceptBuffer),
            ("DayInRange", p) => Ok(Cond::DayInRange(<(usize, usize)>::my_from_str(p)?)),
            ("ParticularDayState", p) => Ok(Cond::ParticularDayState(<DayState>::my_from_str(p)?)),
            ("BeforeDayState", p) => Ok(Cond::ParticularDayState(<DayState>::my_from_str(p)?)),
            ("ParticularDay", p) => Ok(Cond::ParticularDay(<usize>::my_from_str(p)?)),
            ("StaffInRange", p) => Ok(Cond::StaffInRange(<(usize, usize)>::my_from_str(p)?)),
            ("StaffWithAbility", p) => Ok(Cond::StaffWithAbility(<isize>::my_from_str(p)?)),
            ("ParticularStaff", p) => Ok(Cond::ParticularStaff(<usize>::my_from_str(p)?)),
            (s, p) => Err(format!("Failed to parse Cond: {} {}", s, p)),
        }
    }
}

impl MyFromStr for Box<Cond> {
    fn my_from_str(s: &str) -> Result<Self, String> {
        let words: Vec<&str> = s.split_whitespace().collect();
        check_len(
            2,
            &words,
            "Needs 2 fields, but not enough.",
            "Needs 2 fields, but too much given.",
        )?;
        match (words[0], words[1]) {
            ("Every", _) => Ok(Box::new(Cond::Every)),
            ("Or", p) => Ok(Box::new(Cond::Or(<(Box<Cond>, Box<Cond>)>::my_from_str(
                p,
            )?))),
            ("And", p) => Ok(Box::new(Cond::And(<(Box<Cond>, Box<Cond>)>::my_from_str(
                p,
            )?))),
            ("Not", p) => Ok(Box::new(Cond::Not(Box::new(<Cond>::my_from_str(p)?)))),
            ("DayExceptBuffer", _) => Ok(Box::new(Cond::DayExceptBuffer)),
            ("DayInRange", p) => Ok(Box::new(Cond::DayInRange(<(usize, usize)>::my_from_str(
                p,
            )?))),
            ("ParticularDayState", p) => Ok(Box::new(Cond::ParticularDayState(
                <DayState>::my_from_str(p)?,
            ))),
            ("BeforeDayState", p) => Ok(Box::new(Cond::ParticularDayState(
                <DayState>::my_from_str(p)?,
            ))),
            ("ParticularDay", p) => Ok(Box::new(Cond::ParticularDay(<usize>::my_from_str(p)?))),
            ("StaffInRange", p) => Ok(Box::new(Cond::StaffInRange(<(usize, usize)>::my_from_str(
                p,
            )?))),
            ("StaffWithAbility", p) => {
                Ok(Box::new(Cond::StaffWithAbility(<isize>::my_from_str(p)?)))
            }
            ("ParticularStaff", p) => Ok(Box::new(Cond::ParticularStaff(<usize>::my_from_str(p)?))),
            (s, p) => Err(format!("Failed to parse Box<Cond>: {} {}", s, p)),
        }
    }
}

pub fn read_score_props(text: &str) -> Result<Vec<ScoreProp>, String> {
    let mut ans: Vec<ScoreProp> = Vec::new();
    for line in text.lines() {
        ans.push(read_score_prop(&line)?);
    }
    Ok(ans)
}

pub fn read_score_prop(text: &str) -> Result<ScoreProp, String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    check_len(
        2,
        &words,
        "Needs 2 fields, but not enough.",
        "Needs 2 fields, but too much given.",
    )?;
    match (words[0], words[1]) {
        ("PatternInList", p) => Ok(ScoreProp::PatternInList(
            <(Cond, Vec<Vec<Shift>>, Score)>::my_from_str(p)?,
        )),
        ("Pattern", p) => Ok(ScoreProp::Pattern(
            <(Cond, Vec<Shift>, Score)>::my_from_str(p)?,
        )),
        ("Streak", p) => Ok(ScoreProp::Streak(
            <(Cond, Vec<Shift>, isize, Score)>::my_from_str(p)?,
        )),
        ("Need2Holidays", p) => Ok(ScoreProp::Need2Holidays(
            <(Cond, Vec<Shift>, Score)>::my_from_str(p)?,
        )),
        ("ShiftsBalance", p) => Ok(ScoreProp::ShiftsBalance(
            <(Cond, Shift, Shift, Score)>::my_from_str(p)?,
        )),
        ("ShiftHalfBalance", p) => Ok(ScoreProp::ShiftHalfBalance(
            <(Cond, Shift, Score)>::my_from_str(p)?,
        )),
        ("ShiftDirPriority", p) => Ok(ScoreProp::ShiftDirPriority(
            <(Cond, Shift, Score)>::my_from_str(p)?,
        )),
        ("DayCountRegardStaffAttribute", p) => Ok(ScoreProp::DayCountRegardStaffAttribute(
            <(Cond, Shift, StaffAttributeName, Score)>::my_from_str(p)?,
        )),
        ("StaffCountRegardDayAttribute", p) => Ok(ScoreProp::StaffCountRegardDayAttribute(
            <(Cond, Shift, DayAttributeName, Score)>::my_from_str(p)?,
        )),
        ("StaffCount", p) => Ok(ScoreProp::StaffCount(
            <(Cond, Shift, isize, Score)>::my_from_str(p)?,
        )),
        ("NGPair", p) => Ok(ScoreProp::NGPair(<(Cond, Shift, Score)>::my_from_str(p)?)),
        ("NoSamePair", p) => Ok(ScoreProp::NoSamePair(
            <(Cond, isize, Shift, Score)>::my_from_str(p)?,
        )),
        (s, p) => Err(format!("Failed to parse ScoreProp: {} {}", s, p)),
    }
}
