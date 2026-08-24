use bevy::{
    ecs::{schedule::ScheduleLabel, system::ScheduleSystem},
    prelude::*,
};

pub trait AppStateExt {
    fn add_on_enter_state_systems<S: States, M>(
        &mut self,

        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
        state: S,
    ) -> &mut Self;

    fn add_on_exit_state_systems<S: States, M>(
        &mut self,

        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
        state: S,
    ) -> &mut Self;

    fn add_in_state_systems<S: States, M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
        state: S,
    ) -> &mut Self;
}

impl AppStateExt for App {
    fn add_on_enter_state_systems<S: States, M>(
        &mut self,

        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
        state: S,
    ) -> &mut Self {
        self.add_systems(OnEnter(state), systems)
    }

    fn add_on_exit_state_systems<S: States, M>(
        &mut self,

        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
        state: S,
    ) -> &mut Self {
        self.add_systems(OnExit(state), systems)
    }

    fn add_in_state_systems<S: States, M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
        state: S,
    ) -> &mut Self {
        self.add_systems(schedule, systems.run_if(in_state(state)))
    }
}
