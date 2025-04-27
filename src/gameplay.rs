use crate::AppState;
use bevy::prelude::*;

pub struct GameplayPlugin;

#[derive(Component)]
pub struct GameplayEntity;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::InGame), cleanup_gameplay_entities);
    }
}

fn cleanup_gameplay_entities(
    mut commands: Commands,
    gameplay_query: Query<Entity, With<GameplayEntity>>,
) {
    for entity in gameplay_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
