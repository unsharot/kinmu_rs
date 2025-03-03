use super::{ShiftState, ScoreProp, Shift, DayState};

pub type ScheduleConfig = kinmu_model::ScheduleConfig<ScoreProp, Shift, ShiftState, DayState>;
