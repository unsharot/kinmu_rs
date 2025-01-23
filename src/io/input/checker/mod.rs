//! 読み込んだconfigが正常か判定するモジュール

use anyhow::Context as _;
use std::cmp::Ordering;

use crate::kinmu_lib::types::{
    AnnealingConfig, Cond, CondWrapper, DayAttributeName, MainConfig, ScheduleConfig,
    ScoreFunction, ScoreProp, StaffAttributeName,
};

/// チェックの関数
pub fn run(config: &MainConfig) -> anyhow::Result<()> {
    for schedule_config in &config.schedule_configs {
        check_schedule_config(schedule_config)
            .context("schedule_configの変換チェックに失敗しました")?;
        for annealing_config in &schedule_config.annealing_configs {
            check_annealing_config(annealing_config, schedule_config)
                .context("annealing_configの変換チェックに失敗しました")?;
        }
    }
    Ok(())
}

/// 勤務表configのチェック
fn check_schedule_config(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    check_staff_attributes(schedule_config)
        .context("staff.attributesの変換チェックに失敗しました")?;

    check_staff_list(schedule_config).context("staff.listの変換チェックに失敗しました")?;

    check_ng_list(schedule_config).context("staff.ng_listの変換チェックに失敗しました")?;

    check_day_states(schedule_config).context("day.statesの変換チェックに失敗しました")?;

    check_buffer(schedule_config).context("day.buffer_countの変換チェックに失敗しました")?;

    check_schedule_staff(schedule_config)
        .context("day.requested_scheduleの変換チェックに失敗しました")?;

    check_schedule_day(schedule_config)
        .context("day.requested_scheduleの変換チェックに失敗しました")?;

    check_day_attributes(schedule_config).context("day.attributesの変換チェックに失敗しました")?;

    check_score_functions(&schedule_config.result.score_functions, schedule_config)
        .context("result.score_functionsの変換チェックに失敗しました")?;

    Ok(())
}

/// 焼きなましconfigのチェック
fn check_annealing_config(
    annealing_config: &AnnealingConfig,
    schedule_config: &ScheduleConfig,
) -> anyhow::Result<()> {
    check_score_props(&annealing_config.score_props, schedule_config)
        .context("score_functionsの変換チェックに失敗しました")?;

    Ok(())
}

/// staffのattributesが十分か
fn check_staff_attributes(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    let l = schedule_config.staff.attribute_map.names.len();
    for staff in &schedule_config.staff.list {
        match staff.attributes.len().cmp(&l) {
            Ordering::Less => Err(anyhow::anyhow!(
                "staff {} のattributesの長さ({})がstaff.attributesの長さ({})より小さいです",
                staff.name,
                staff.attributes.len(),
                &l
            )),
            Ordering::Greater => Err(anyhow::anyhow!(
                "staff {} のattributesの長さ({})がstaff.attributesの長さ({})より大きいです",
                staff.name,
                staff.attributes.len(),
                &l
            )),
            Ordering::Equal => Ok(()),
        }?;
    }
    Ok(())
}

/// 職員リストが人数分あるか
fn check_staff_list(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    match schedule_config
        .staff
        .list
        .len()
        .cmp(&schedule_config.staff.count)
    {
        Ordering::Less => Err(anyhow::anyhow!(
            "staff.listの長さ({})がstaff.count({})より小さいです",
            schedule_config.staff.list.len(),
            &schedule_config.staff.count
        )),
        Ordering::Greater => Err(anyhow::anyhow!(
            "staff.listの長さ({})がstaff.count({})より大きいです",
            schedule_config.staff.list.len(),
            &schedule_config.staff.count
        )),
        Ordering::Equal => Ok(()),
    }?;
    Ok(())
}

/// NGリストが正常か
fn check_ng_list(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    for (i, (from, to)) in schedule_config.staff.ng_list.iter().enumerate() {
        if schedule_config.staff.count <= *from {
            Err(anyhow::anyhow!(
                "staff.ng_listの{}番目のfrom({})がstaffの番号の最大値({})より大きいです",
                i + 1,
                *from,
                schedule_config.staff.count - 1
            ))?;
        }
        if schedule_config.staff.count <= *to {
            Err(anyhow::anyhow!(
                "staff.ng_listの{}番目のto({})がstaffの番号の最大値({})より大きいです",
                i + 1,
                *to,
                schedule_config.staff.count - 1
            ))?;
        }
    }
    Ok(())
}

/// DayStateが日数だけあるか
fn check_day_states(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    match schedule_config
        .day
        .days
        .len()
        .cmp(&schedule_config.day.count)
    {
        Ordering::Less => Err(anyhow::anyhow!(
            "day.statesの長さ({})がday.count({})より小さいです",
            schedule_config.day.days.len(),
            &schedule_config.day.count
        )),
        Ordering::Greater => Err(anyhow::anyhow!(
            "day.statesの長さ({})がday.count({})より大きいです",
            schedule_config.day.days.len(),
            &schedule_config.day.count
        )),
        Ordering::Equal => Ok(()),
    }?;
    Ok(())
}

/// bufferがday_countを超えない
fn check_buffer(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    if schedule_config.day.buffer_count > schedule_config.day.count {
        Err(anyhow::anyhow!(
            "buffer({})がday_count({})より大きいです",
            schedule_config.day.buffer_count,
            schedule_config.day.count
        ))?;
    }
    Ok(())
}

/// スケジュールが職員だけあるか
fn check_schedule_staff(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    match schedule_config
        .day
        .requested_schedule
        .len()
        .cmp(&schedule_config.staff.count)
    {
        Ordering::Less => Err(anyhow::anyhow!(
            "day.requested_scheduleの長さ({})からstaff.count({})より小さいです",
            schedule_config.day.requested_schedule.len(),
            &schedule_config.staff.count
        )),
        Ordering::Greater => Err(anyhow::anyhow!(
            "day.requested_scheduleの長さ({})からstaff.count({})より大きいです",
            schedule_config.day.requested_schedule.len(),
            &schedule_config.staff.count
        )),
        Ordering::Equal => Ok(()),
    }?;
    Ok(())
}

/// スケジュールが日数分あるか
fn check_schedule_day(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    for (i, row) in schedule_config.day.requested_schedule.iter().enumerate() {
        match row.len().cmp(&schedule_config.day.count) {
            Ordering::Less => Err(anyhow::anyhow!(
                "day.requested_scheduleの{}行目の長さ({})がday.count({})より大きいです",
                i + 1,
                row.len(),
                &schedule_config.day.count
            )),
            Ordering::Greater => Err(anyhow::anyhow!(
                "day.requested_scheduleの{}行目の長さ({})がday.count({})より小さいです",
                i + 1,
                row.len(),
                &schedule_config.day.count
            )),
            Ordering::Equal => Ok(()),
        }?;
    }
    Ok(())
}

/// DayAttributeが日数分か
fn check_day_attributes(schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
    for (k, v) in schedule_config.day.attributes.iter() {
        match v.len().cmp(&schedule_config.day.count) {
            Ordering::Less => Err(anyhow::anyhow!(
                "day.attributes {} のvaluesの長さ({})がday.count({})より小さいです",
                k,
                v.len(),
                schedule_config.day.count
            )),
            Ordering::Greater => Err(anyhow::anyhow!(
                "day.attributes {} のvaluesの長さ({})がday.count({})より大きいです",
                k,
                v.len(),
                schedule_config.day.count
            )),
            Ordering::Equal => Ok(()),
        }?;
    }
    Ok(())
}

/// ScoreFunctionsが有効か
fn check_score_functions(
    score_functions: &Vec<ScoreFunction>,
    sc: &ScheduleConfig,
) -> anyhow::Result<()> {
    for sf in score_functions {
        check_score_props(&sf.scores, sc).with_context(|| {
            format!(
                "スコア関数 {} の変換チェックに失敗しました",
                &sf.display_name
            )
        })?;
    }
    Ok(())
}

/// ScorePropの中のStaffAttributeNameやDayAttributeNameが有効か
fn check_score_props(score_props: &Vec<ScoreProp>, sc: &ScheduleConfig) -> anyhow::Result<()> {
    for score_prop in score_props {
        match score_prop {
            ScoreProp::PatternGeneral((c, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::PatternFixed((c, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::PatternGeneralAny((c, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::PatternFixedAny((c, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::Streak((c, _, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::ShiftsBalance((c, _, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::ShiftHalfBalance((c, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::ShiftDirPriority((c, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::DayCountRegardStaffAttribute((c, _, sa, _)) => {
                check_cond_wrapper(c, sc).and(check_staff_attribute_exists(sa, sc))
            }
            ScoreProp::StaffCountRegardDayAttribute((c, _, da, _)) => {
                check_cond_wrapper(c, sc).and(check_day_attribute_exists(da, sc))
            }
            ScoreProp::StaffCount((c, _, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::StaffCountAtLeast((c, _, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::StaffCountWithPremise((c1, _, _, c2, _, _, _)) => {
                check_cond_wrapper(c1, sc).and(check_cond_wrapper(c2, sc))
            }
            ScoreProp::NGPair((c, _, _)) => check_cond_wrapper(c, sc),
            ScoreProp::NoSamePair((c, _, _, _)) => check_cond_wrapper(c, sc),
        }
        .with_context(|| format!("スコア {:?} の変換チェックに失敗しました", score_prop))?;
    }
    Ok(())
}

/// CondWrapperの中のStaffAttributeNameやDayAttributeNameが有効か
fn check_cond_wrapper(c: &CondWrapper, sc: &ScheduleConfig) -> anyhow::Result<()> {
    check_cond(&c.cond, sc)
        .with_context(|| format!("CondWrapper {:?} の変換チェックに失敗しました", &c.cond))?;
    Ok(())
}

/// Condの中のStaffAttributeNameやDayAttributeNameが有効か
fn check_cond(c: &Cond, sc: &ScheduleConfig) -> anyhow::Result<()> {
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
    sc: &ScheduleConfig,
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
fn check_day_attribute_exists(da: &DayAttributeName, sc: &ScheduleConfig) -> anyhow::Result<()> {
    if sc.day.attributes.contains_key(da) {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "{} はday.attributesに登録されていません",
            da
        ))
    }
}
