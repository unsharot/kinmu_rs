//! 読み込んだconfigが正常か判定するモジュール

use std::cmp::Ordering;

use crate::kinmu_lib::types::ScheduleProp;

pub fn check_schedule_prop(schedule_prop: &ScheduleProp) -> Result<(), String> {
    check_staff_attributes(schedule_prop)?;

    check_day_attributes(schedule_prop)?;

    check_staff_list(schedule_prop)?;

    check_day_states(schedule_prop)?;

    check_schedule_staff(schedule_prop)?;

    check_schedule_day(schedule_prop)?;

    check_ng_list(schedule_prop)?;

    check_buffer(schedule_prop)?;

    Ok(())
}

/// staffのattributesが十分か
fn check_staff_attributes(schedule_prop: &ScheduleProp) -> Result<(), String> {
    let l = schedule_prop.staff_attribute_map.names.len();
    for staff in &schedule_prop.staff_list {
        match staff.attributes.len().cmp(&l) {
            Ordering::Less => Err(format!(
                "staff {} のattributeが設定より少ないです",
                staff.name
            )),
            Ordering::Greater => Err(format!(
                "staff {} のattributeが設定より多いです",
                staff.name
            )),
            Ordering::Equal => Ok(()),
        }?;
    }
    Ok(())
}

/// DayAttributeが日数分か
fn check_day_attributes(schedule_prop: &ScheduleProp) -> Result<(), String> {
    for (k, v) in schedule_prop.day_attributes.iter() {
        match v.len().cmp(&schedule_prop.day_count) {
            Ordering::Less => Err(format!("day_attribute {} が指定日数より少ないです", k)),
            Ordering::Greater => Err(format!("day_attribute {} が指定日数より多いです", k)),
            Ordering::Equal => Ok(()),
        }?;
    }
    Ok(())
}

/// 職員リストが人数分あるか
fn check_staff_list(schedule_prop: &ScheduleProp) -> Result<(), String> {
    match schedule_prop
        .staff_list
        .len()
        .cmp(&schedule_prop.staff_count)
    {
        Ordering::Less => Err("staff_listのstaffの数が設定人数より少ないです"),
        Ordering::Greater => Err("staff_listのstaffの数が設定人数より多いです"),
        Ordering::Equal => Ok(()),
    }?;
    Ok(())
}

/// DayStateが日数だけあるか
fn check_day_states(schedule_prop: &ScheduleProp) -> Result<(), String> {
    match schedule_prop.days.len().cmp(&schedule_prop.day_count) {
        Ordering::Less => Err("day_statesが設定日数より少ないです"),
        Ordering::Greater => Err("day_statesが設定日数より多いです"),
        Ordering::Equal => Ok(()),
    }?;
    Ok(())
}

/// スケジュールが職員だけあるか
fn check_schedule_staff(schedule_prop: &ScheduleProp) -> Result<(), String> {
    match schedule_prop.request.len().cmp(&schedule_prop.staff_count) {
        Ordering::Less => Err("requested_scheduleが設定人数より少ないです"),
        Ordering::Greater => Err("requested_scheduleが設定人数より多いです"),
        Ordering::Equal => Ok(()),
    }?;
    Ok(())
}

/// スケジュールが日数分あるか
fn check_schedule_day(schedule_prop: &ScheduleProp) -> Result<(), String> {
    for (i, row) in schedule_prop.request.iter().enumerate() {
        match row.len().cmp(&schedule_prop.day_count) {
            Ordering::Less => Err(format!(
                "requested_scheduleの{}行目が設定日数より少ないです",
                i + 1
            )),
            Ordering::Greater => Err(format!(
                "requested_scheduleの{}行目が設定日数より少ないです",
                i + 1
            )),
            Ordering::Equal => Ok(()),
        }?;
    }
    Ok(())
}

/// NGリストが正常か
fn check_ng_list(schedule_prop: &ScheduleProp) -> Result<(), String> {
    for (i, (from, to)) in schedule_prop.ng_list.iter().enumerate() {
        if schedule_prop.staff_count <= *from {
            Err(format!(
                "ng_listの{}番目のfromがstaffの番号の範囲より大きいです",
                i + 1
            ))?;
        }
        if schedule_prop.staff_count <= *to {
            Err(format!(
                "ng_listの{}番目のtoがstaffの番号の範囲より大きいです",
                i + 1
            ))?;
        }
    }
    Ok(())
}

/// bufferがday_countを超えない
fn check_buffer(schedule_prop: &ScheduleProp) -> Result<(), String> {
    if schedule_prop.buffer > schedule_prop.day_count {
        Err("bufferがday_countを超えています")?;
    }
    Ok(())
}
