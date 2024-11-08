use crate::kinmu_lib::types::{
    Cond, DayAttributeName, DayState, Days, NGList, Schedule, Score, ScoreProp, Shift, Staff,
    StaffAttributeName, NG,
};

use super::common::check_len;

pub trait FromConfig: Sized {
    fn from_config(s: &str) -> Result<Self, String>;
}

impl FromConfig for String {
    fn from_config(s: &str) -> Result<Self, String> {
        Ok(s.to_string())
    }
}

impl<T, U> FromConfig for (T, U)
where
    T: FromConfig,
    U: FromConfig,
{
    fn from_config(s: &str) -> Result<Self, String> {
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
        let t = T::from_config(words[0]).map_err(|e| e.to_string())?;
        let u = U::from_config(words[1]).map_err(|e| e.to_string())?;
        Ok((t, u))
    }
}

impl<T, U, V> FromConfig for (T, U, V)
where
    T: FromConfig,
    U: FromConfig,
    V: FromConfig,
{
    fn from_config(s: &str) -> Result<Self, String> {
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
        let t = T::from_config(words[0]).map_err(|e| e.to_string())?;
        let u = U::from_config(words[1]).map_err(|e| e.to_string())?;
        let v = V::from_config(words[2]).map_err(|e| e.to_string())?;
        Ok((t, u, v))
    }
}

impl<T, U, V, W> FromConfig for (T, U, V, W)
where
    T: FromConfig,
    U: FromConfig,
    V: FromConfig,
    W: FromConfig,
{
    fn from_config(s: &str) -> Result<Self, String> {
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
        let t = T::from_config(words[0]).map_err(|e| e.to_string())?;
        let u = U::from_config(words[1]).map_err(|e| e.to_string())?;
        let v = V::from_config(words[2]).map_err(|e| e.to_string())?;
        let w = W::from_config(words[3]).map_err(|e| e.to_string())?;
        Ok((t, u, v, w))
    }
}

impl FromConfig for usize {
    fn from_config(s: &str) -> Result<Self, String> {
        Ok(s.parse::<usize>().map_err(|e| e.to_string())?)
    }
}

impl FromConfig for isize {
    fn from_config(s: &str) -> Result<Self, String> {
        Ok(s.parse::<isize>().map_err(|e| e.to_string())?)
    }
}

pub fn read_isizes(text: &str) -> Result<Vec<isize>, String> {
    text.split_whitespace()
        .map(|x| x.parse::<isize>().map_err(|e| e.to_string()))
        .collect()
}

impl FromConfig for f32 {
    fn from_config(s: &str) -> Result<Self, String> {
        Ok(s.parse::<f32>().map_err(|e| e.to_string())?)
    }
}

impl FromConfig for Shift {
    fn from_config(s: &str) -> Result<Self, String> {
        Ok(s.parse::<Shift>().map_err(|e| e.to_string())?)
    }
}

impl FromConfig for Vec<Shift> {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<_> = s
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .collect();
        let mut ans = Vec::new();
        for w in words {
            ans.push(<Shift>::from_config(w)?)
        }
        Ok(ans)
    }
}

impl FromConfig for Vec<Vec<Shift>> {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<_> = s
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .collect();
        let mut ans = Vec::new();
        for w in words {
            ans.push(<Vec<Shift>>::from_config(w)?)
        }
        Ok(ans)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let v1: Vec<Shift> = <Vec<Shift>>::from_config("[N, I, K]").unwrap();

        assert_eq!(v1, vec![Shift::N, Shift::I, Shift::K]);
    }

    #[test]
    fn test2() {
        let v2 = <Vec<Vec<Shift>>>::from_config("[[N, I, K], [O, H, A]]").unwrap();

        assert_eq!(
            v2,
            vec![
                vec![Shift::N, Shift::I, Shift::K],
                vec![Shift::O, Shift::H, Shift::A]
            ]
        );
    }
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
        ability: <isize>::from_config(&words[0])?,
        k_day_count: <isize>::from_config(&words[1])?,
        i_day_count: <isize>::from_config(&words[2])?,
        o_day_count: <isize>::from_config(&words[3])?,
        h_day_count: <isize>::from_config(&words[4])?,
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
    let id1 = <usize>::from_config(&words[0])?;
    let id2 = <usize>::from_config(&words[1])?;
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
    let id1 = <f32>::from_config(&words[0])?;
    let id2 = <f32>::from_config(&words[1])?;
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

impl FromConfig for DayState {
    fn from_config(s: &str) -> Result<Self, String> {
        Ok(s.parse::<DayState>().map_err(|e| e.to_string())?)
    }
}

impl FromConfig for Cond {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<&str> = s.split_whitespace().collect();
        check_len(
            2,
            &words,
            "Needs 2 fields, but not enough.",
            "Needs 2 fields, but too much given.",
        )?;
        match (words[0], words[1]) {
            ("Every", _) => Ok(Cond::Every),
            ("Or", p) => Ok(Cond::Or(<(Box<Cond>, Box<Cond>)>::from_config(p)?)),
            ("And", p) => Ok(Cond::And(<(Box<Cond>, Box<Cond>)>::from_config(p)?)),
            ("Not", p) => Ok(Cond::Not(Box::new(<Cond>::from_config(p)?))),
            ("DayExceptBuffer", _) => Ok(Cond::DayExceptBuffer),
            ("DayInRange", p) => Ok(Cond::DayInRange(<(usize, usize)>::from_config(p)?)),
            ("ParticularDayState", p) => Ok(Cond::ParticularDayState(<DayState>::from_config(p)?)),
            ("BeforeDayState", p) => Ok(Cond::ParticularDayState(<DayState>::from_config(p)?)),
            ("ParticularDay", p) => Ok(Cond::ParticularDay(<usize>::from_config(p)?)),
            ("StaffInRange", p) => Ok(Cond::StaffInRange(<(usize, usize)>::from_config(p)?)),
            ("StaffWithAbility", p) => Ok(Cond::StaffWithAbility(<isize>::from_config(p)?)),
            ("ParticularStaff", p) => Ok(Cond::ParticularStaff(<usize>::from_config(p)?)),
            (s, p) => Err(format!("Failed to parse Cond: {} {}", s, p)),
        }
    }
}

impl FromConfig for Box<Cond> {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<&str> = s.split_whitespace().collect();
        check_len(
            2,
            &words,
            "Needs 2 fields, but not enough.",
            "Needs 2 fields, but too much given.",
        )?;
        match (words[0], words[1]) {
            ("Every", _) => Ok(Box::new(Cond::Every)),
            ("Or", p) => Ok(Box::new(Cond::Or(<(Box<Cond>, Box<Cond>)>::from_config(
                p,
            )?))),
            ("And", p) => Ok(Box::new(Cond::And(<(Box<Cond>, Box<Cond>)>::from_config(
                p,
            )?))),
            ("Not", p) => Ok(Box::new(Cond::Not(Box::new(<Cond>::from_config(p)?)))),
            ("DayExceptBuffer", _) => Ok(Box::new(Cond::DayExceptBuffer)),
            ("DayInRange", p) => Ok(Box::new(Cond::DayInRange(<(usize, usize)>::from_config(
                p,
            )?))),
            ("ParticularDayState", p) => Ok(Box::new(Cond::ParticularDayState(
                <DayState>::from_config(p)?,
            ))),
            ("BeforeDayState", p) => Ok(Box::new(Cond::ParticularDayState(
                <DayState>::from_config(p)?,
            ))),
            ("ParticularDay", p) => Ok(Box::new(Cond::ParticularDay(<usize>::from_config(p)?))),
            ("StaffInRange", p) => Ok(Box::new(Cond::StaffInRange(<(usize, usize)>::from_config(
                p,
            )?))),
            ("StaffWithAbility", p) => {
                Ok(Box::new(Cond::StaffWithAbility(<isize>::from_config(p)?)))
            }
            ("ParticularStaff", p) => Ok(Box::new(Cond::ParticularStaff(<usize>::from_config(p)?))),
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
        ("PatternGeneral", p) => Ok(ScoreProp::PatternGeneral(
            <(Cond, Vec<Vec<Shift>>, Score)>::from_config(p)?,
        )),
        ("PatternFixed", p) => Ok(ScoreProp::PatternFixed(
            <(Cond, Vec<Shift>, Score)>::from_config(p)?,
        )),
        ("Streak", p) => Ok(ScoreProp::Streak(
            <(Cond, Vec<Shift>, isize, Score)>::from_config(p)?,
        )),
        ("Need2Holidays", p) => Ok(ScoreProp::Need2Holidays(
            <(Cond, Vec<Shift>, Score)>::from_config(p)?,
        )),
        ("ShiftsBalance", p) => Ok(ScoreProp::ShiftsBalance(
            <(Cond, Shift, Shift, Score)>::from_config(p)?,
        )),
        ("ShiftHalfBalance", p) => Ok(ScoreProp::ShiftHalfBalance(
            <(Cond, Shift, Score)>::from_config(p)?,
        )),
        ("ShiftDirPriority", p) => Ok(ScoreProp::ShiftDirPriority(
            <(Cond, Shift, Score)>::from_config(p)?,
        )),
        ("DayCountRegardStaffAttribute", p) => Ok(ScoreProp::DayCountRegardStaffAttribute(
            <(Cond, Shift, StaffAttributeName, Score)>::from_config(p)?,
        )),
        ("StaffCountRegardDayAttribute", p) => Ok(ScoreProp::StaffCountRegardDayAttribute(
            <(Cond, Shift, DayAttributeName, Score)>::from_config(p)?,
        )),
        ("StaffCount", p) => Ok(ScoreProp::StaffCount(
            <(Cond, Shift, isize, Score)>::from_config(p)?,
        )),
        ("NGPair", p) => Ok(ScoreProp::NGPair(<(Cond, Shift, Score)>::from_config(p)?)),
        ("NoSamePair", p) => Ok(ScoreProp::NoSamePair(
            <(Cond, isize, Shift, Score)>::from_config(p)?,
        )),
        (s, p) => Err(format!("Failed to parse ScoreProp: {} {}", s, p)),
    }
}
