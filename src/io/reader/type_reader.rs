use crate::kinmu_lib::types::{
    Cond, CondWrapper, DayAttributeName, DayState, Days, NGList, Schedule, Score, ScoreProp, Shift,
    Staff, StaffAttributeName, StaffAttributeNameIndexMap, NG,
};

use super::common::check_len;

use std::collections::HashMap;

pub trait FromConfig: Sized {
    fn from_config(s: &str) -> Result<Self, String>;
}

impl FromConfig for String {
    fn from_config(s: &str) -> Result<Self, String> {
        Ok(s.to_string())
    }
}

/// タプルを読み込む
/// Vecやタプルの複数の入れ子構造になったタプルにも対応
/// 括弧がない場合も対応
fn format_str_tuple_to_words(s: &str) -> Result<Vec<&str>, String> {
    let trimmed_s = s.trim();
    let bare_s;
    if trimmed_s.starts_with("(") {
        if !trimmed_s.ends_with(")") {
            return Err("found '(', but ')' not found".to_string());
        }
        bare_s = &trimmed_s[1..(trimmed_s.len() - 1)];
    } else {
        bare_s = trimmed_s;
    }
    let mut words = Vec::new();
    let mut bracket_count = 0;
    let mut start_idx = 0;
    let mut end_idx = 0;
    for c in bare_s.chars() {
        if bracket_count == 0 && c == ',' {
            words.push(bare_s[start_idx..end_idx].trim());
            start_idx = end_idx + 1;
        }
        if c == '(' || c == '[' {
            bracket_count += 1;
        }
        if c == ')' || c == ']' {
            bracket_count -= 1;
        }
        end_idx += 1;
    }
    if !bare_s[start_idx..end_idx].trim().is_empty() {
        words.push(bare_s[start_idx..end_idx].trim());
    }

    Ok(words)
}

impl<T, U> FromConfig for (T, U)
where
    T: FromConfig,
    U: FromConfig,
{
    fn from_config(s: &str) -> Result<Self, String> {
        let words = format_str_tuple_to_words(s)?;
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
        let words = format_str_tuple_to_words(s)?;
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
        let words = format_str_tuple_to_words(s)?;
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

impl<T, U, V, W, X, Y, Z> FromConfig for (T, U, V, W, X, Y, Z)
where
    T: FromConfig,
    U: FromConfig,
    V: FromConfig,
    W: FromConfig,
    X: FromConfig,
    Y: FromConfig,
    Z: FromConfig,
{
    fn from_config(s: &str) -> Result<Self, String> {
        let words = format_str_tuple_to_words(s)?;
        check_len(
            7,
            &words,
            "Needs 7 fields, but not enough.",
            "Needs 7 fields, but too much given.",
        )?;
        let t = T::from_config(words[0]).map_err(|e| e.to_string())?;
        let u = U::from_config(words[1]).map_err(|e| e.to_string())?;
        let v = V::from_config(words[2]).map_err(|e| e.to_string())?;
        let w = W::from_config(words[3]).map_err(|e| e.to_string())?;
        let x = X::from_config(words[4]).map_err(|e| e.to_string())?;
        let y = Y::from_config(words[5]).map_err(|e| e.to_string())?;
        let z = Z::from_config(words[6]).map_err(|e| e.to_string())?;
        Ok((t, u, v, w, x, y, z))
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

pub struct DayAttributeWrapper(pub HashMap<DayAttributeName, Vec<isize>>);

impl FromConfig for DayAttributeWrapper {
    fn from_config(s: &str) -> Result<Self, String> {
        let mut ans = HashMap::new();
        let mut name_flag = true;
        let mut name = "".to_string();
        for line in s.lines() {
            if name_flag {
                name = line.to_string();
                name_flag = false;
            } else {
                let att: Vec<isize> = line
                    .split_whitespace()
                    .map(|x| x.parse::<isize>().map_err(|e| e.to_string()))
                    .collect::<Result<Vec<_>, String>>()?;
                ans.insert(name.clone(), att);
                name_flag = true;
            }
        }
        Ok(DayAttributeWrapper(ans))
    }
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

/// Vecを読み込む
/// 2重入れ子構造になったVecにも対応
fn format_str_vec_to_words(s: &str) -> Result<Vec<&str>, String> {
    let trimmed_s = s.trim();
    if !trimmed_s.starts_with("[") {
        return Err("\'[\' not found".to_string());
    }
    if !trimmed_s.ends_with("]") {
        return Err("\']\' not found".to_string());
    }
    let bare_s = &trimmed_s[1..(trimmed_s.len() - 1)];
    let mut words = Vec::new();
    let mut bracket_flag = false;
    let mut start_idx = 0;
    let mut end_idx = 0;
    for c in bare_s.chars() {
        if !bracket_flag && c == ',' {
            words.push(bare_s[start_idx..end_idx].trim());
            start_idx = end_idx + 1;
        }
        if c == '[' {
            bracket_flag = true;
        }
        if c == ']' {
            bracket_flag = false;
        }
        end_idx += 1;
    }
    if !bare_s[start_idx..end_idx].trim().is_empty() {
        words.push(bare_s[start_idx..end_idx].trim());
    }

    Ok(words)
}

impl FromConfig for Vec<Shift> {
    fn from_config(s: &str) -> Result<Self, String> {
        let words = format_str_vec_to_words(s)?;
        let mut ans = Vec::new();
        for w in words {
            ans.push(<Shift>::from_config(w)?);
        }
        Ok(ans)
    }
}

impl FromConfig for Vec<Vec<Shift>> {
    fn from_config(s: &str) -> Result<Self, String> {
        let words = format_str_vec_to_words(s)?;
        let mut ans = Vec::new();
        for w in words {
            ans.push(<Vec<Shift>>::from_config(w)?);
        }
        Ok(ans)
    }
}

impl FromConfig for Vec<Staff> {
    fn from_config(s: &str) -> Result<Self, String> {
        let mut staff: Vec<Staff> = Vec::new();
        for line in s.lines() {
            let a_staff = <Staff>::from_config(&line)?;
            staff.push(a_staff);
        }
        Ok(staff)
    }
}

impl FromConfig for Staff {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        let l = words.len();
        check_len(
            6,
            &words,
            "Needs 6 fields, but not enough.",
            "Needs 6 fields, but too much given.",
        )?;
        let mut attributes = Vec::new();
        for i in 1..(l - 1) {
            attributes.push(<isize>::from_config(&words[i])?);
        }
        let worker: Staff = Staff {
            name: words[l - 1].clone(),
            ability: <isize>::from_config(&words[0])?,
            attributes: attributes,
        };
        Ok(worker)
    }
}

pub struct NGListWrapper(pub NGList);

impl FromConfig for NGListWrapper {
    fn from_config(s: &str) -> Result<Self, String> {
        let mut ans: NGList = Vec::new();
        for line in s.lines() {
            let NGWrapper(ng) = <NGWrapper>::from_config(&line)?;
            ans.push(ng);
        }
        Ok(NGListWrapper(ans))
    }
}

struct NGWrapper(pub NG);

impl FromConfig for NGWrapper {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        check_len(
            2,
            &words,
            "Needs 2 fields, but not enough.",
            "Needs 2 fields, but too much given.",
        )?;
        let id1 = <usize>::from_config(&words[0])?;
        let id2 = <usize>::from_config(&words[1])?;
        Ok(NGWrapper((id1, id2)))
    }
}

pub struct TempWrapper(pub f32, pub f32);

impl FromConfig for TempWrapper {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        check_len(
            2,
            &words,
            "Needs 2 fields, but not enough.",
            "Needs 2 fields, but too much given.",
        )?;
        let id1 = <f32>::from_config(&words[0])?;
        let id2 = <f32>::from_config(&words[1])?;
        Ok(TempWrapper(id1, id2))
    }
}

impl FromConfig for Days {
    fn from_config(s: &str) -> Result<Self, String> {
        let mut ans: Days = Vec::new();
        for c in s.chars() {
            ans.push(c.to_string().parse::<DayState>()?);
        }
        Ok(ans)
    }
}

pub struct ScheduleWrapper(pub Schedule);

impl FromConfig for ScheduleWrapper {
    fn from_config(s: &str) -> Result<Self, String> {
        let mut ans: Schedule = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_string().parse::<Shift>()?);
            }
            ans.push(row);
        }
        Ok(ScheduleWrapper(ans))
    }
}

impl FromConfig for DayState {
    fn from_config(s: &str) -> Result<Self, String> {
        Ok(s.parse::<DayState>().map_err(|e| e.to_string())?)
    }
}

impl FromConfig for Cond {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<&str> = s.splitn(2, ' ').collect();
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
            ("BeforeDayState", p) => Ok(Cond::BeforeDayState(<DayState>::from_config(p)?)),
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
        let words: Vec<&str> = s.splitn(2, ' ').collect();
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

impl FromConfig for CondWrapper {
    fn from_config(s: &str) -> Result<Self, String> {
        let cond = Cond::from_config(s)?;
        Ok(CondWrapper::new(cond))
    }
}

impl FromConfig for ScoreProp {
    fn from_config(s: &str) -> Result<Self, String> {
        let words: Vec<&str> = s.splitn(2, ' ').collect();
        check_len(
            2,
            &words,
            "Needs 2 fields, but not enough.",
            "Needs 2 fields, but too much given.",
        )?;
        match (words[0], words[1]) {
            ("PatternGeneral", p) => Ok(ScoreProp::PatternGeneral(<(
                CondWrapper,
                Vec<Vec<Shift>>,
                Score,
            )>::from_config(p)?)),
            ("PatternFixed", p) => Ok(ScoreProp::PatternFixed(
                <(CondWrapper, Vec<Shift>, Score)>::from_config(p)?,
            )),
            ("PatternGeneralAny", p) => Ok(ScoreProp::PatternGeneralAny(<(
                CondWrapper,
                Vec<Vec<Shift>>,
                Score,
            )>::from_config(
                p
            )?)),
            ("PatternFixedAny", p) => Ok(ScoreProp::PatternFixedAny(<(
                CondWrapper,
                Vec<Shift>,
                Score,
            )>::from_config(
                p
            )?)),
            ("Streak", p) => Ok(ScoreProp::Streak(
                <(CondWrapper, Vec<Shift>, isize, Score)>::from_config(p)?,
            )),
            ("ShiftsBalance", p) => Ok(ScoreProp::ShiftsBalance(<(
                CondWrapper,
                Shift,
                Shift,
                Score,
            )>::from_config(p)?)),
            ("ShiftHalfBalance", p) => {
                Ok(ScoreProp::ShiftHalfBalance(
                    <(CondWrapper, Shift, Score)>::from_config(p)?,
                ))
            }
            ("ShiftDirPriority", p) => {
                Ok(ScoreProp::ShiftDirPriority(
                    <(CondWrapper, Shift, Score)>::from_config(p)?,
                ))
            }
            ("DayCountRegardStaffAttribute", p) => Ok(ScoreProp::DayCountRegardStaffAttribute(
                <(CondWrapper, Shift, StaffAttributeName, Score)>::from_config(p)?,
            )),
            ("StaffCountRegardDayAttribute", p) => Ok(ScoreProp::StaffCountRegardDayAttribute(
                <(CondWrapper, Shift, DayAttributeName, Score)>::from_config(p)?,
            )),
            ("StaffCount", p) => Ok(ScoreProp::StaffCount(
                <(CondWrapper, Shift, isize, Score)>::from_config(p)?,
            )),
            ("StaffCountWithPremise", p) => Ok(ScoreProp::StaffCountWithPremise(<(
                CondWrapper,
                Shift,
                isize,
                CondWrapper,
                Shift,
                isize,
                Score,
            )>::from_config(
                p
            )?)),
            ("NGPair", p) => Ok(ScoreProp::NGPair(
                <(CondWrapper, Shift, Score)>::from_config(p)?,
            )),
            ("NoSamePair", p) => Ok(ScoreProp::NoSamePair(
                <(CondWrapper, isize, Shift, Score)>::from_config(p)?,
            )),
            (s, p) => Err(format!("Failed to parse ScoreProp: {} {}", s, p)),
        }
    }
}

impl FromConfig for Vec<ScoreProp> {
    fn from_config(s: &str) -> Result<Self, String> {
        let mut ans: Vec<ScoreProp> = Vec::new();
        for line in s.lines() {
            ans.push(<ScoreProp>::from_config(&line)?);
        }
        Ok(ans)
    }
}

pub struct StaffAttributeMapWrapper(pub StaffAttributeNameIndexMap);

impl FromConfig for StaffAttributeMapWrapper {
    fn from_config(s: &str) -> Result<Self, String> {
        let names: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        let mut name_to_index = HashMap::new();
        for (i, name) in names.iter().enumerate() {
            name_to_index.insert(name.to_string(), i);
        }
        let sa_map = StaffAttributeNameIndexMap {
            names: names,
            name_to_index: name_to_index,
        };
        Ok(StaffAttributeMapWrapper(sa_map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_shift_test() {
        let v1: Vec<Shift> = <Vec<Shift>>::from_config("[N, I, K]").unwrap();

        assert_eq!(v1, vec![Shift::N, Shift::I, Shift::K]);
    }

    #[test]
    fn vec_vec_shift_test() {
        let v2 = <Vec<Vec<Shift>>>::from_config("[[N, I, K], [O, H, A]]").unwrap();

        assert_eq!(
            v2,
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
            ScoreProp::PatternGeneral((
                CondWrapper::new(Cond::Every),
                vec![
                    vec![Shift::N, Shift::O, Shift::H],
                    vec![Shift::O, Shift::H],
                    vec![Shift::K, Shift::Y]
                ],
                123.0
            )),
            ScoreProp::from_config(s).unwrap()
        );
    }

    #[test]
    fn parse_tuple_test() {
        assert_eq!(<(isize, isize)>::from_config("(1,2"), Err("found '(', but ')' not found".to_string()));
        assert_eq!(<(isize, isize)>::from_config("(1)"), Err("Needs 2 fields, but not enough.".to_string()));
        assert_eq!(<(isize, isize)>::from_config("(1, 2, 3)"), Err("Needs 2 fields, but too much given.".to_string()));
        assert_eq!(<(isize, isize)>::from_config("(1,2)"), Ok((1,2)));
        assert_eq!(<(isize, isize)>::from_config("1,2"), Ok((1,2)));
        assert_eq!(<(isize, isize)>::from_config(" 1, 2 "), Ok((1,2)));
    }
}
