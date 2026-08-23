use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_hara_utils::prelude::*;

struct AppPlugin;

#[derive(Debug, Resource, AssetCollection)]
pub struct AppAssetCollection {}

game_state! {
    pub enum AppState {
        #[default]
        MainMenu
    },
    [AppAssetCollection]
}

game_state! {
    source: AppState = AppState::MainMenu,
    pub enum MainMenuState {
        #[default]
        Idle
    }
}

// Generated code
//
// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, States)]
// pub enum LoadingAppState {
//     #[default]
//     Loading,
//     Loaded,
// }
// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
// #[source(LoadingAppState = LoadingAppState::Loaded)]
// pub enum AppState {
//     #[default]
//     MainMenu,
// }
// pub trait AppStateExt {
//     fn init_app_state(&mut self) -> &mut Self;
// }
// impl AppStateExt for bevy::prelude::App {
//     fn init_app_state(&mut self) -> &mut Self {
//         self.add_sub_state::<AppState>()
//             .init_state::<LoadingAppState>()
//             .add_loading_state(
//                 LoadingState::new(LoadingAppState::Loading)
//                     .load_collection::<AppAssetCollection>()
//                     .continue_to_state(LoadingAppState::Loaded),
//             )
//     }
// }
//
// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
// #[source(AppState = AppState::MainMenu)]
// pub enum LoadingMainMenuState {
//     #[default]
//     Loading,
//     Loaded,
// }
// #[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, SubStates)]
// #[source(LoadingMainMenuState = LoadingMainMenuState::Loaded)]
// pub enum MainMenuState {
//     #[default]
//     Idle,
// }
// pub trait MainMenuStateExt {
//     fn init_main_menu_state(&mut self) -> &mut Self;
// }
// impl MainMenuStateExt for bevy::prelude::App {
//     fn init_main_menu_state(&mut self) -> &mut Self {
//         self.add_sub_state::<MainMenuState>()
//             .add_sub_state::<LoadingMainMenuState>()
//             .add_loading_state(bevy_hara_utils::game_state!(LoadingMainMenuState))
//     }
// }

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_app_state().init_main_menu_state();
    }
}

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppPlugin)
        .run();
}
