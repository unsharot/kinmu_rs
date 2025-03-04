use super::{DayState, ScoreProp, Shift, ShiftState};

use ::kinmu_input::Check;
use ::kinmu_model::{DayAttributeName, StaffAttributeName};

pub type ScheduleConfig = kinmu_model::ScheduleConfig<ScoreProp, Shift, ShiftState, DayState>;

pub(super) struct StaffAttributeNameWrapper<'a>(pub &'a StaffAttributeName);

impl Check<ScoreProp, Shift, ShiftState, DayState> for StaffAttributeNameWrapper<'_> {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        if schedule_config.staff.attribute_map.names.contains(self.0) {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "{} はstaff.attributesに登録されていません",
                self.0
            ))
        }
    }
}

pub(super) struct DayAttributeNameWrapper<'a>(pub &'a DayAttributeName);

impl Check<ScoreProp, Shift, ShiftState, DayState> for DayAttributeNameWrapper<'_> {
    fn check(&self, schedule_config: &ScheduleConfig) -> anyhow::Result<()> {
        if schedule_config.day.attributes.contains_key(self.0) {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "{} はday.attributesに登録されていません",
                self.0
            ))
        }
    }
}
