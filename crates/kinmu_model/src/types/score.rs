use super::super::{Schedule, ScheduleConfig};

pub type Score = f32;

pub trait ScorePropTrait<S, SS, DS>: Sized {
    fn eval_mut(
        &mut self,
        schedule_config: &ScheduleConfig<Self, S, SS, DS>,
        schedule: &Schedule<S>,
    ) -> Score;
    fn eval_immut(
        &self,
        schedule_config: &ScheduleConfig<Self, S, SS, DS>,
        schedule: &Schedule<S>,
    ) -> Score;
}

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
