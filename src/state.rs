use bevy::{
    ecs::{schedule::ScheduleLabel, system::ScheduleSystem},
    prelude::*,
};

pub trait AppStateExt {
    fn add_state_system<S: States, M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
        state: S,
    ) -> &mut Self;
}

impl AppStateExt for App {
    fn add_state_system<S: States, M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
        state: S,
    ) -> &mut Self {
        self.add_systems(schedule, systems.run_if(in_state(state)))
    }
}
