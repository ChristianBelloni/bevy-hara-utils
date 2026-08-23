use bevy::prelude::*;

pub trait Cleanable<State> {
    const STATE: State;
    type CleanupSystemType;
    const CLEANUP_SYSTEM: Self::CleanupSystemType;
}

pub trait MarkerCleanable<State> {
    const STATE: State;
    fn cleanup(mut commands: Commands, entities: Query<Entity, With<Self>>)
    where
        Self: Component + Sized,
    {
        for entity in entities {
            commands.entity(entity).despawn();
        }
    }
}

impl<S: States, T: MarkerCleanable<S> + bevy::prelude::Component> Cleanable<S> for T {
    const STATE: S = <T as MarkerCleanable<S>>::STATE;

    type CleanupSystemType = fn(Commands, Query<Entity, With<T>>);

    const CLEANUP_SYSTEM: Self::CleanupSystemType = T::cleanup;
}

pub trait AppCleanupExt {
    fn add_cleanup_system<S: States, C: Cleanable<S>, SMarker>(&mut self, _: C)
    where
        C::CleanupSystemType: IntoSystem<(), (), SMarker>;
}

impl AppCleanupExt for App {
    fn add_cleanup_system<S, C, SMarker>(&mut self, _: C)
    where
        C::CleanupSystemType: IntoSystem<(), (), SMarker>,
        S: States,
        C: Cleanable<S>,
    {
        self.add_systems(OnExit(C::STATE), C::CLEANUP_SYSTEM);
    }
}

#[macro_export]
macro_rules! marker_cleanup {
    ($marker:ident, $state_ident:ident = $state_variant:expr) => {
        impl MarkerCleanable<$state_ident> for $marker {
            const STATE: $state_ident = $state_variant;
        }
    };
}

pub use crate::marker_cleanup;
