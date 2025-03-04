//! 読み込んだconfigが正常か判定するモジュール

use anyhow::Context as _;
use std::cmp::Ordering;

use ::kinmu_model::{AnnealingConfig, MainConfig, ScheduleConfig, ScoreFunction};

/// チェックの関数
pub fn run<SP, S, SS, DS>(config: &MainConfig<SP, S, SS, DS>) -> anyhow::Result<()>
where
    SP: Check<SP, S, SS, DS>,
{
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
fn check_schedule_config<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()>
where
    SP: Check<SP, S, SS, DS>,
{
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
fn check_annealing_config<SP, S, SS, DS>(
    annealing_config: &AnnealingConfig<SP>,
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()>
where
    SP: Check<SP, S, SS, DS>,
{
    annealing_config
        .score_props
        .iter()
        .try_for_each(|x| x.check(schedule_config))
        .context("score_functionsの変換チェックに失敗しました")?;

    Ok(())
}

/// staffのattributesが十分か
fn check_staff_attributes<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()> {
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
fn check_staff_list<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()> {
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
fn check_ng_list<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()> {
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
fn check_day_states<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()> {
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
fn check_buffer<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()> {
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
fn check_schedule_staff<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()> {
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
fn check_schedule_day<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()> {
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
fn check_day_attributes<SP, S, SS, DS>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()> {
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
fn check_score_functions<SP, S, SS, DS>(
    score_functions: &Vec<ScoreFunction<SP>>,
    sc: &ScheduleConfig<SP, S, SS, DS>,
) -> anyhow::Result<()>
where
    SP: Check<SP, S, SS, DS>,
{
    for sf in score_functions {
        sf.scores.iter().try_for_each(|x| {
            x.check(sc).with_context(|| {
                format!(
                    "スコア関数 {} の変換チェックに失敗しました",
                    &sf.display_name
                )
            })
        })?;
    }
    Ok(())
}

pub trait Check<SP, S, SS, DS>: Sized {
    fn check(&self, schedule_config: &ScheduleConfig<SP, S, SS, DS>) -> anyhow::Result<()>;
}
