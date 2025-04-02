use super::{DayConfig, Schedule, StaffConfig};

/// スコアのエイリアス
pub type Score = f32;

/// ScorePropの共通のふるまいを定める
pub trait ScoreProp<S, SS, DS>: Sized {
    /// mutで評価する
    fn eval_mut(
        &mut self,
        staff_config: &StaffConfig,
        day_config: &DayConfig<S, SS, DS>,
        schedule: &Schedule<S>,
    ) -> Score;

    /// immutで評価する
    fn eval_immut(
        &self,
        staff_config: &StaffConfig,
        day_config: &DayConfig<S, SS, DS>,
        schedule: &Schedule<S>,
    ) -> Score;
}

/// mutでScorePropのVecを全て評価して和をとる
#[allow(clippy::ptr_arg)]
pub fn eval_scores_mut<SP: ScoreProp<S, SS, DS>, S, SS, DS>(
    sps: &mut Vec<SP>,
    staff_config: &StaffConfig,
    day_config: &DayConfig<S, SS, DS>,
    schedule: &Schedule<S>,
) -> Score {
    sps.iter_mut()
        .map(|sp: &mut SP| sp.eval_mut(staff_config, day_config, schedule))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

/// immutでScorePropのVecを全て評価して和をとる
#[allow(clippy::ptr_arg)]
pub fn eval_scores_immut<SP: ScoreProp<S, SS, DS>, S, SS, DS>(
    sps: &Vec<SP>,
    staff_config: &StaffConfig,
    day_config: &DayConfig<S, SS, DS>,
    schedule: &Schedule<S>,
) -> Score {
    sps.iter()
        .map(|sp: &SP| sp.eval_immut(staff_config, day_config, schedule))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}
