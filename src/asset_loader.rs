/// Define an opinionated game state with an optional list of asset collections
///
/// Usage:
///
/// ```
/// use bevy::prelude::*;
/// use bevy_asset_loader::*;
/// use bevy_hara_utils::prelude::*;
///
/// #[derive(Debug, Resource, AssetCollection)]
/// pub struct AppAssetCollection {}
/// game_state! {
///     pub enum AppState {
///         #[default]
///         MainMenu
///     },
///     [AppAssetCollection]
/// }
/// ```
/// Generated code
/// ```
/// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
/// pub enum LoadingAppState {
///     #[default]
///     Loading,
///     Loaded,
/// }
/// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
/// #[source(LoadingAppState = LoadingAppState::Loaded)]
/// pub enum AppState {
///     #[default]
///     MainMenu,
/// }
/// pub trait AppStateExt {
///     fn init_app_state(&mut self) -> &mut Self;
/// }
/// impl AppStateExt for bevy::prelude::App {
///     fn init_app_state(&mut self) -> &mut Self {
///         self.add_sub_state::<AppState>()
///             .init_state::<LoadingAppState>()
///             .add_loading_state(
///                 LoadingState::new(LoadingAppState::Loading)
///                     .load_collection::<AppAssetCollection>()
///                     .continue_to_state(LoadingAppState::Loaded),
///             )
///     }
/// }
/// ```
///
/// Substates usage:
///
/// ```
/// use bevy::prelude::*;
/// use bevy_asset_loader::*;
/// use bevy_hara_utils::prelude::*;
///
/// #[derive(Debug, Resource, AssetCollection)]
/// pub struct AppAssetCollection {}
/// game_state! {
///     pub enum AppState {
///         #[default]
///         MainMenu
///     },
///     [AppAssetCollection]
/// }
///
/// game_state! {
///     source: AppState = AppState::MainMenu,
///     pub enum MainMenuState {
///         #[default]
///         Idle
///     }
/// }
/// ```
///
/// Generated code:
/// ```
/// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
/// pub enum LoadingAppState {
///     #[default]
///     Loading,
///     Loaded,
/// }
/// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
/// #[source(LoadingAppState = LoadingAppState::Loaded)]
/// pub enum AppState {
///     #[default]
///     MainMenu,
/// }
/// pub trait AppStateExt {
///     fn init_app_state(&mut self) -> &mut Self;
/// }
/// impl AppStateExt for bevy::prelude::App {
///     fn init_app_state(&mut self) -> &mut Self {
///         self.add_sub_state::<AppState>()
///             .init_state::<LoadingAppState>()
///             .add_loading_state(
///                 LoadingState::new(LoadingAppState::Loading)
///                     .load_collection::<AppAssetCollection>()
///                     .continue_to_state(LoadingAppState::Loaded),
///             )
///     }
/// }
///
/// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
/// #[source(AppState = AppState::MainMenu)]
/// pub enum LoadingMainMenuState {
///     #[default]
///     Loading,
///     Loaded,
/// }
/// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
/// #[source(LoadingMainMenuState = LoadingMainMenuState::Loaded)]
/// pub enum MainMenuState {
///     #[default]
///     Idle,
/// }
/// pub trait MainMenuStateExt {
///     fn init_main_menu_state(&mut self) -> &mut Self;
/// }
/// impl MainMenuStateExt for bevy::prelude::App {
///     fn init_main_menu_state(&mut self) -> &mut Self {
///         self.add_sub_state::<MainMenuState>()
///             .add_sub_state::<LoadingMainMenuState>()
///             .add_loading_state(bevy_hara_utils::game_state!(LoadingMainMenuState))
///     }
/// }
/// ```
#[macro_export]
macro_rules! game_state {
    ($state:ident$(, [$($collection:ident),*])?) => {
        LoadingState::new($state::Loading)
            $($(.load_collection::<$collection>())*)?
            .continue_to_state($state::Loaded)
    };


    (inner: pub enum $state:ident {$($tt:tt)*} $(, [$($collection:ident),*])?) => {
        ::paste::paste! {
            #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
            #[source([< Loading $state >] = [ < Loading $state > ]::Loaded)]
            pub enum $state {$($tt)*}
        }
    };

    (pub enum $state:ident {$($tt:tt)*} $(, [$($collection:ident),*])?) => {
        ::paste::paste! {
            #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
            pub enum [< Loading $state >] {
                #[default]
                Loading,
                Loaded
            }
        }

        $crate::game_state! { inner: pub enum $state { $($tt)* } }

        ::paste::paste! {
           pub trait [< $state Ext >] {
                fn [< init_ $state:snake >](&mut self) -> &mut Self;
            }

            impl [< $state Ext >] for bevy::prelude::App {
                fn [< init_ $state:snake >](&mut self) -> &mut Self {
                    self
                        .add_sub_state::<$state>()
                        .init_state::< [< Loading $state >] >()
                        .add_loading_state($crate::game_state!([< Loading $state >]$(, [$($collection),*])? ))
                }
            }
        }
    };
    (source: $source_state:ident = $source_variant:expr, pub enum $state:ident {$($tt:tt)*} $(, [$($collection:ident),*])?) => {
        ::paste::paste! {
            #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
            #[source($source_state = $source_variant)]
            pub enum [< Loading $state >] {
                #[default]
                Loading,
                Loaded
            }
        }

        $crate::game_state! { inner: pub enum $state { $($tt)* } }

        ::paste::paste! {
           pub trait [< $state Ext >] {
                fn [< init_ $state:snake >](&mut self) -> &mut Self;

                fn [< run_in_ $state:snake >]<S, M>(
                    &mut self,
                    schedule: impl ::bevy::ecs::schedule::ScheduleLabel,
                    systems: impl ::bevy::prelude::IntoScheduleConfigs<::bevy::ecs::system::ScheduleSystem, M>,
                ) -> &mut Self;
            }

            impl [< $state Ext >] for bevy::prelude::App {
                fn [< init_ $state:snake >](&mut self) -> &mut Self {
                    self
                        .add_sub_state::<$state>()
                        .add_sub_state::< [< Loading $state >] >()
                        .add_loading_state($crate::game_state!([< Loading $state >]$(, [$($collection),*])? ))
                }

                fn [< run_in_ $state:snake >]<S, M>(
                    &mut self,
                    schedule: impl ::bevy::ecs::schedule::ScheduleLabel,
                    systems: impl ::bevy::prelude::IntoScheduleConfigs<::bevy::ecs::system::ScheduleSystem, M>,
                ) -> &mut Self {
                    use $crate::state::AppStateExt;

                    self.add_state_system(schedule, systems, [< Loading $state >]::Loaded)
                }
            }
        }
    };
}

pub use crate::game_state;
