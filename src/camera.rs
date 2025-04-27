use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

#[derive(Resource)]
pub struct GameRenderResolution {
    pub width: f32,
    pub height: f32,
}

impl Default for GameRenderResolution {
    fn default() -> Self {
        Self {
            width: 1920.0,
            height: 1080.0,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameRenderResolution>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera_scaling);
    }
}

fn setup_camera(mut commands: Commands, _render_resolution: Res<GameRenderResolution>) {
    commands.spawn(Camera2d);
}

fn update_camera_scaling(
    render_resolution: Res<GameRenderResolution>,
    mut projection_query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    for mut projection in &mut projection_query {
        projection.scaling_mode = ScalingMode::AutoMin {
            min_width: render_resolution.width,
            min_height: render_resolution.height,
        };
    }
}
