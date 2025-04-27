use crate::AppState;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameOverlayState {
    #[default]
    None,
    GameMenu,
    SettingsMenu,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum OverlayBackgroundState {
    #[default]
    None,
    Darkened,
}

#[derive(Component)]
pub struct OverlayBackground;

pub struct OverlayStatePlugin;

impl Plugin for OverlayStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameOverlayState>()
            .init_state::<OverlayBackgroundState>()
            .add_systems(
                OnEnter(OverlayBackgroundState::Darkened),
                spawn_overlay_background,
            )
            .add_systems(
                OnExit(OverlayBackgroundState::Darkened),
                cleanup_overlay_background,
            )
            .add_systems(
                Update,
                update_overlay_background_state.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnExit(AppState::InGame),
                |mut next_background_state: ResMut<NextState<OverlayBackgroundState>>| {
                    next_background_state.set(OverlayBackgroundState::None);
                },
            );
    }
}

fn spawn_overlay_background(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.95)),
        OverlayBackground,
        ZIndex(-1),
    ));
}

fn cleanup_overlay_background(
    mut commands: Commands,
    background_query: Query<Entity, With<OverlayBackground>>,
) {
    for entity in background_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_overlay_background_state(
    overlay_state: Res<State<GameOverlayState>>,
    mut next_background_state: ResMut<NextState<OverlayBackgroundState>>,
) {
    match overlay_state.get() {
        GameOverlayState::None => {
            next_background_state.set(OverlayBackgroundState::None);
        }
        _ => {
            next_background_state.set(OverlayBackgroundState::Darkened);
        }
    }
}
