use super::{DayState, Shift, ShiftState, StdScoreProp};

use kinmu_input_by_file::Check;
use kinmu_model::{DayAttributeName, StaffAttributeName};

/// 具体的なScheduleConfig
pub type ScheduleConfig = kinmu_model::ScheduleConfig<StdScoreProp, Shift, ShiftState, DayState>;

/// 具体的なDayConfig
pub type DayConfig = kinmu_model::DayConfig<Shift, ShiftState, DayState>;

/// StaffAttributeNameにCheckを実装するためのWrapper
pub(super) struct StaffAttributeNameWrapper<'a>(pub &'a StaffAttributeName);

impl Check<StdScoreProp, Shift, ShiftState, DayState> for StaffAttributeNameWrapper<'_> {
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

/// DayAttributeNameにCheckを実装するためのWrapper
pub(super) struct DayAttributeNameWrapper<'a>(pub &'a DayAttributeName);

impl Check<StdScoreProp, Shift, ShiftState, DayState> for DayAttributeNameWrapper<'_> {
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
