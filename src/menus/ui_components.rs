use bevy::prelude::*;

#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub active: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
            active: Color::linear_rgb(0.35, 0.35, 0.35),
        }
    }
}

// Common UI dimensions
pub const BUTTON_WIDTH: f32 = 150.0;
pub const BUTTON_HEIGHT: f32 = 40.0;
pub const BOTTOM_BUTTON_HEIGHT: f32 = 50.0;
