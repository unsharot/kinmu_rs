//! ScoreProp, Shift, Daysを文字列から変換するためのモジュール

use ::kinmu_lib::types::{
    Cond, CondWrapper, DayAttributeName, DayState, Score, ScoreProp, Shift, StaffAttributeName,
};

use anyhow::Context as _;

pub trait FromConfig: Sized {
    fn from_config(s: &str) -> anyhow::Result<Self>;
}

impl FromConfig for String {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.to_string())
    }
}

/// タプルを読み込む
/// Vecやタプルの複数の入れ子構造になったタプルにも対応
/// 括弧がない場合も対応
fn format_str_tuple_to_words(s: &str) -> anyhow::Result<Vec<&str>> {
    let trimmed_s = s.trim();
    let bare_s = if trimmed_s.starts_with('(') {
        if !trimmed_s.ends_with(')') {
            return Err(anyhow::anyhow!("found '(', but ')' not found"));
        }
        &trimmed_s[1..(trimmed_s.len() - 1)]
    } else {
        trimmed_s
    };
    let mut words = Vec::new();
    let mut bracket_count = 0;
    let mut start_idx = 0;
    let mut end_idx = 0;
    for c in bare_s.chars() {
        if bracket_count == 0 && c == ',' {
            words.push(bare_s[start_idx..end_idx].trim());
            start_idx = end_idx + c.len_utf8();
        }
        if c == '(' || c == '[' {
            bracket_count += 1;
        }
        if c == ')' || c == ']' {
            bracket_count -= 1;
        }
        end_idx += c.len_utf8();
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
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_tuple_to_words(s)?;
        anyhow::ensure!(words.len() >= 2, "Needs 2 fields, but not enough.");
        anyhow::ensure!(2 >= words.len(), "Needs 2 fields, but too much given.");
        let t = T::from_config(words[0])
            .with_context(|| format!("Failed to parse 1st field of {}", s))?;
        let u = U::from_config(words[1])
            .with_context(|| format!("Failed to parse 2nd field of {}", s))?;
        Ok((t, u))
    }
}

impl<T, U, V> FromConfig for (T, U, V)
where
    T: FromConfig,
    U: FromConfig,
    V: FromConfig,
{
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_tuple_to_words(s)?;
        anyhow::ensure!(words.len() >= 3, "Needs 3 fields, but not enough.");
        anyhow::ensure!(3 >= words.len(), "Needs 3 fields, but too much given.");
        let t = T::from_config(words[0])
            .with_context(|| format!("Failed to parse 1st field of {}", s))?;
        let u = U::from_config(words[1])
            .with_context(|| format!("Failed to parse 2nd field of {}", s))?;
        let v = V::from_config(words[2])
            .with_context(|| format!("Failed to parse 3rd field of {}", s))?;
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
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_tuple_to_words(s)?;
        anyhow::ensure!(words.len() >= 4, "Needs 4 fields, but not enough.");
        anyhow::ensure!(4 >= words.len(), "Needs 4 fields, but too much given.");
        let t = T::from_config(words[0])
            .with_context(|| format!("Failed to parse 1st field of {}", s))?;
        let u = U::from_config(words[1])
            .with_context(|| format!("Failed to parse 2nd field of {}", s))?;
        let v = V::from_config(words[2])
            .with_context(|| format!("Failed to parse 3rd field of {}", s))?;
        let w = W::from_config(words[3])
            .with_context(|| format!("Failed to parse 4th field of {}", s))?;
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
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_tuple_to_words(s)?;
        anyhow::ensure!(words.len() >= 7, "Needs 7 fields, but not enough.");
        anyhow::ensure!(7 >= words.len(), "Needs 7 fields, but too much given.");
        let t = T::from_config(words[0])
            .with_context(|| format!("Failed to parse 1st field of {}", s))?;
        let u = U::from_config(words[1])
            .with_context(|| format!("Failed to parse 2nd field of {}", s))?;
        let v = V::from_config(words[2])
            .with_context(|| format!("Failed to parse 3rd field of {}", s))?;
        let w = W::from_config(words[3])
            .with_context(|| format!("Failed to parse 4th field of {}", s))?;
        let x = X::from_config(words[4])
            .with_context(|| format!("Failed to parse 5th field of {}", s))?;
        let y = Y::from_config(words[5])
            .with_context(|| format!("Failed to parse 6th field of {}", s))?;
        let z = Z::from_config(words[6])
            .with_context(|| format!("Failed to parse 7th field of {}", s))?;
        Ok((t, u, v, w, x, y, z))
    }
}

impl FromConfig for usize {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.parse::<usize>()?)
    }
}

impl FromConfig for isize {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.parse::<isize>()?)
    }
}

impl FromConfig for i32 {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.parse::<i32>()?)
    }
}

impl FromConfig for f32 {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.parse::<f32>()?)
    }
}

impl FromConfig for Shift {
    fn from_config(s: &str) -> anyhow::Result<Self> {
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
            _ => Err(anyhow::anyhow!("Failed to parse Shift: {}", s)),
        }
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

impl FromConfig for Vec<Shift> {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_vec_to_words(s)?;
        let mut ans = Vec::new();
        for w in words {
            ans.push(<Shift>::from_config(w)?);
        }
        Ok(ans)
    }
}

impl FromConfig for Vec<Vec<Shift>> {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_vec_to_words(s)?;
        let mut ans = Vec::new();
        for w in words {
            ans.push(<Vec<Shift>>::from_config(w)?);
        }
        Ok(ans)
    }
}

impl FromConfig for Vec<DayState> {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let mut ans: Vec<DayState> = Vec::new();
        for c in s.chars() {
            ans.push(<DayState>::from_config(&c.to_string())?);
        }
        Ok(ans)
    }
}

impl FromConfig for DayState {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        match s {
            "W" => Ok(DayState::Weekday),
            "H" => Ok(DayState::Holiday),
            "B" => Ok(DayState::Bath),
            "2" => Ok(DayState::Bath2),
            "M" => Ok(DayState::Measure),
            _ => Err(anyhow::anyhow!("Failed to parse DayState: {}", s)),
        }
    }
}

impl FromConfig for Cond {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words: Vec<&str> = s.splitn(2, ' ').collect();
        anyhow::ensure!(words.len() >= 2, "Needs 2 fields, but not enough.");
        anyhow::ensure!(2 >= words.len(), "Needs 2 fields, but too much given.");
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
            ("StaffWithAttribute", p) => Ok(Cond::StaffWithAttribute(
                <(StaffAttributeName, i32)>::from_config(p)?,
            )),
            ("ParticularStaff", p) => Ok(Cond::ParticularStaff(<usize>::from_config(p)?)),
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
        )>::from_config(p)?)),
        ("PatternFixedAny", p) => Ok(ScoreProp::PatternFixedAny(<(
            CondWrapper,
            Vec<Shift>,
            Score,
        )>::from_config(p)?)),
        ("Streak", p) => Ok(ScoreProp::Streak(
            <(CondWrapper, Vec<Shift>, i32, Score)>::from_config(p)?,
        )),
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

pub struct ScheduleRowWrapper(pub Vec<Shift>);

impl FromConfig for ScheduleRowWrapper {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let mut ans = Vec::new();
        for c in s.chars() {
            ans.push(<Shift>::from_config(&c.to_string())?);
        }
        Ok(ScheduleRowWrapper(ans))
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
        assert_eq!(
            <(isize, isize)>::from_config("(1,2")
                .unwrap_err()
                .to_string(),
            String::from("found '(', but ')' not found")
        );
        assert_eq!(
            <(isize, isize)>::from_config("(1)")
                .unwrap_err()
                .to_string(),
            String::from("Needs 2 fields, but not enough.")
        );
        assert_eq!(
            <(isize, isize)>::from_config("(1, 2, 3)")
                .unwrap_err()
                .to_string(),
            String::from("Needs 2 fields, but too much given.")
        );
        assert_eq!(<(isize, isize)>::from_config("(1,2)").unwrap(), (1, 2));
        assert_eq!(<(isize, isize)>::from_config("1,2").unwrap(), (1, 2));
        assert_eq!(<(isize, isize)>::from_config(" 1, 2 ").unwrap(), (1, 2));
    }

    #[test]
    fn parse_japanese() {
        let s = "StaffCountRegardDayAttribute (DayExceptBuffer (), I, 夜勤, 1000)";
        assert_eq!(
            ScoreProp::from_config(s).unwrap(),
            ScoreProp::StaffCountRegardDayAttribute((
                CondWrapper::from_config("DayExceptBuffer ()").unwrap(),
                Shift::I,
                String::from("夜勤"),
                1000.0
            ))
        );
    }
}
