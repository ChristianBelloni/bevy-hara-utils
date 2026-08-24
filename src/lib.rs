pub mod asset_loader;
pub mod button;
pub mod cleanup;
pub mod component;

pub mod prelude {
    pub use crate::asset_loader::game_state;
    pub use crate::button::*;
    pub use crate::cleanup::*;
    pub use crate::component::*;
}
