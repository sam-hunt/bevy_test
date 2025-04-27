#![allow(clippy::type_complexity)]

mod actions;
mod audio;
pub mod camera;
mod gameplay;
mod loading;
mod menus;
mod overlay_state;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::gameplay::GameplayPlugin;
use crate::loading::LoadingPlugin;
use crate::menus::{GameMenuPlugin, MainMenuPlugin, SettingsMenuPlugin};
use crate::overlay_state::OverlayStatePlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    InGame,
    // Here the main menu is drawn and waiting for player interaction
    MainMenu,
    // Settings menu state
    SettingsMenu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_plugins((
            LoadingPlugin,
            MainMenuPlugin,
            SettingsMenuPlugin,
            GameMenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            OverlayStatePlugin,
            GameplayPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
