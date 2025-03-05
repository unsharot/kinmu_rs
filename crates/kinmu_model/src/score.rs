use super::{Schedule, ScheduleConfig};

/// スコアのエイリアス
pub type Score = f32;

/// ScorePropの共通のふるまいを定める
pub trait ScorePropTrait<S, SS, DS>: Sized {
    /// mutで評価する
    fn eval_mut(
        &mut self,
        schedule_config: &ScheduleConfig<Self, S, SS, DS>,
        schedule: &Schedule<S>,
    ) -> Score;

    /// immutで評価する
    fn eval_immut(
        &self,
        schedule_config: &ScheduleConfig<Self, S, SS, DS>,
        schedule: &Schedule<S>,
    ) -> Score;
}

/// mutでScorePropのVecを全て評価して和をとる
#[allow(clippy::ptr_arg)]
pub fn eval_scores_mut<SP: ScorePropTrait<S, SS, DS>, S, SS, DS>(
    sps: &mut Vec<SP>,
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
    schedule: &Schedule<S>,
) -> Score {
    sps.iter_mut()
        .map(|sp: &mut SP| sp.eval_mut(schedule_config, schedule))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

/// immutでScorePropのVecを全て評価して和をとる
#[allow(clippy::ptr_arg)]
pub fn eval_scores_immut<SP: ScorePropTrait<S, SS, DS>, S, SS, DS>(
    sps: &Vec<SP>,
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
    schedule: &Schedule<S>,
) -> Score {
    sps.iter()
        .map(|sp: &SP| sp.eval_immut(schedule_config, schedule))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}
