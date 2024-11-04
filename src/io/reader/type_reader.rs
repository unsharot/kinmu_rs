use crate::kinmu_lib::types::{
    Shift,
    Staff,
    NGList,
    NG,
    Days,
    DayState,
    Schedule,
    ScoreProp,
};

use super::common::check_len;




pub fn read_usize(text: &str) -> Result<usize, String> {
    let ans: usize = text.parse::<usize>().map_err(|e| e.to_string())?;
    Ok(ans)
}

// pub fn read_usizes(text: &str) -> Result<Vec<usize>, String> {
//     text.split_whitespace().map(|x| x.parse::<usize>().map_err(|e| e.to_string())).collect()
// }

pub fn read_isize(text: &str) -> Result<isize, String> {
    let ans: isize = text.parse::<isize>().map_err(|e| e.to_string())?;
    Ok(ans)
}

pub fn read_isizes(text: &str) -> Result<Vec<isize>, String> {
    text.split_whitespace().map(|x| x.parse::<isize>().map_err(|e| e.to_string())).collect()
}

pub fn read_float(text: &str) -> Result<f32, String> {
    let ans: f32 = text.parse::<f32>().map_err(|e| e.to_string())?;
    Ok(ans)
}

pub fn read_float_pair(text: &str) -> Result<(f32, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(2, &words, "Needs 2 fields, but not enough.", "Needs 2 fields, but too much given.")?;
    let f1 = words[0].parse::<f32>().map_err(|e| e.to_string())?;
    let f2 = words[1].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((f1, f2))
}

pub fn read_isize_float(text: &str) -> Result<(isize, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(2, &words, "Needs 2 fields, but not enough.", "Needs 2 fields, but too much given.")?;
    let i = words[0].parse::<isize>().map_err(|e| e.to_string())?;
    let f = words[1].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((i, f))
}

pub fn read_isize_isize_float(text: &str) -> Result<(isize, isize, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(3, &words, "Needs 3 fields, but not enough.", "Needs 3 fields, but too much given.")?;
    let i1 = words[0].parse::<isize>().map_err(|e| e.to_string())?;
    let i2 = words[1].parse::<isize>().map_err(|e| e.to_string())?;
    let f = words[2].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((i1, i2, f))
}

pub fn read_shift_float(text: &str) -> Result<(Shift, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(2, &words, "Needs 2 fields, but not enough.", "Needs 2 fields, but too much given.")?;
    let s = words[0].parse::<Shift>().map_err(|e| e.to_string())?;
    let f = words[1].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((s, f))
}

pub fn read_daystate_isize_float(text: &str) -> Result<(DayState, isize, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(3, &words, "Needs 3 fields, but not enough.", "Needs 3 fields, but too much given.")?;
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
    check_len(6, &words, "Needs 6 fields, but not enough.", "Needs 6 fields, but too much given.")?;
    let worker: Staff = Staff {
        name: words[5].clone(),
        ability: read_isize(&words[0])?,
        k_day_count: read_isize(&words[1])?,
        i_day_count: read_isize(&words[2])?,
        o_day_count: read_isize(&words[3])?,
        h_day_count: read_isize(&words[4])?,
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
    check_len(2, &words, "Needs 2 fields, but not enough.", "Needs 2 fields, but too much given.")?;
    let id1 = read_usize(&words[0])?;
    let id2 = read_usize(&words[1])?;
    Ok((id1, id2))
}

pub fn read_temp(text: &str) -> Result<(f32, f32), String> {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    check_len(2, &words, "Needs 2 fields, but not enough.", "Needs 2 fields, but too much given.")?;
    let id1 = read_float(&words[0])?;
    let id2 = read_float(&words[1])?;
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

pub fn read_score_props(text: &str) -> Result<Vec<ScoreProp>, String> {
    let mut ans: Vec<ScoreProp> = Vec::new();
    for line in text.lines() {
        ans.push(read_score_prop(&line)?);
    }
    Ok(ans)
}

pub fn read_score_prop(text: &str) -> Result<ScoreProp, String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    check_len(2, &words, "Needs 2 fields, but not enough.", "Needs 2 fields, but too much given.")?;
    match (words[0], words[1]) {
        ("IAKpattern", p) => Ok(ScoreProp::IAKpattern(read_float(p)?)),
        ("KIApattern", p) => Ok(ScoreProp::KIApattern(read_float(p)?)),
        ("KNIApattern", p) => Ok(ScoreProp::KNIApattern(read_float(p)?)),
        ("NNIApattern", p) => Ok(ScoreProp::NNIApattern(read_float(p)?)),
        ("ONpattern", p) => Ok(ScoreProp::ONpattern(read_float(p)?)),
        ("NHpattern", p) => Ok(ScoreProp::NHpattern(read_float(p)?)),
        ("OHpattern", p) => Ok(ScoreProp::OHpattern(read_float(p)?)),
        ("WorkingDayStreak4", p) => Ok(ScoreProp::WorkingDayStreak4(read_float_pair(p)?)),
        ("WorkingDayStreak5", p) => Ok(ScoreProp::WorkingDayStreak5(read_float_pair(p)?)),
        ("WorkingDayStreak6", p) => Ok(ScoreProp::WorkingDayStreak6(read_float_pair(p)?)),
        ("HolidayReward", p) => Ok(ScoreProp::HolidayReward(read_float(p)?)),
        ("Need2Holidays", p) => Ok(ScoreProp::Need2Holidays(read_float(p)?)),
        ("Need2HolidaysNoBf", p) => Ok(ScoreProp::Need2HolidaysNoBf(read_float(p)?)),
        ("OHBalance", p) => Ok(ScoreProp::OHBalance(read_float(p)?)),
        ("ShiftHalfBalance", p) => Ok(ScoreProp::ShiftHalfBalance(read_shift_float(p)?)),
        ("ShiftDirPriority", p) => Ok(ScoreProp::ShiftDirPriority(read_shift_float(p)?)),
        ("KDayCount", p) => Ok(ScoreProp::KDayCount(read_float(p)?)),
        ("IDayCount", p) => Ok(ScoreProp::IDayCount(read_float(p)?)),
        ("ODayCount", p) => Ok(ScoreProp::ODayCount(read_float(p)?)),
        ("HDayCount", p) => Ok(ScoreProp::HDayCount(read_float(p)?)),
        ("IStaffCount", p) => Ok(ScoreProp::IStaffCount(read_float(p)?)),
        ("NStaffCount", p) => Ok(ScoreProp::NStaffCount(read_daystate_isize_float(p)?)),
        ("OStaffCount", p) => Ok(ScoreProp::OStaffCount(read_isize_float(p)?)),
        ("HStaffCount", p) => Ok(ScoreProp::HStaffCount(read_isize_float(p)?)),
        ("NGPair", p) => Ok(ScoreProp::NGPair(read_float(p)?)),
        ("LeaderAbility", p) => Ok(ScoreProp::LeaderAbility(read_isize_float(p)?)),
        ("IAloneAbility", p) => Ok(ScoreProp::IAloneAbility(read_isize_float(p)?)),
        ("IAloneBeforeBath", p) => Ok(ScoreProp::IAloneBeforeBath(read_float(p)?)),
        ("NStaffCountWithAbility", p) => Ok(ScoreProp::NStaffCountWithAbility(read_isize_isize_float(p)?)),
        ("NoSamePair3", p) => Ok(ScoreProp::NoSamePair3(read_float(p)?)),
        ("NoSamePair2", p) => Ok(ScoreProp::NoSamePair2(read_float(p)?)),
        ("NoUndef", p) => Ok(ScoreProp::NoUndef(read_float(p)?)),
        (s, p) => Err(format!("Failed to parse ScoreProp: {} {}",s,p))
    }
}