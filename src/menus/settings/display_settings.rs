use crate::camera::GameRenderResolution;
use crate::menus::settings::ui_constants::{
    BOTTOM_BUTTONS_MARGIN, SETTINGS_CONTAINER_HEIGHT, SETTINGS_CONTAINER_WIDTH, SETTING_ROW_HEIGHT,
    SETTING_ROW_MARGIN,
};
use crate::menus::ui_components::{
    ButtonColors, BOTTOM_BUTTON_HEIGHT, BUTTON_HEIGHT, BUTTON_WIDTH,
};
use crate::overlay_state::GameOverlayState;
use crate::AppState;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, PresentMode, WindowMode, WindowPosition, WindowResolution};

#[derive(Event)]
pub struct DisplaySettingsChanged;

#[derive(Resource)]
pub struct NewDisplaySettings {
    pub resolution: (u32, u32),
    pub window_mode: WindowModeType,
    pub vsync: bool,
}

impl FromWorld for NewDisplaySettings {
    fn from_world(world: &mut World) -> Self {
        let window = world.query::<&Window>().single(world);
        Self::new(window)
    }
}

impl NewDisplaySettings {
    fn new(window: &Window) -> Self {
        let current_res = (
            window.resolution.width() as u32,
            window.resolution.height() as u32,
        );

        let current_mode = match window.mode {
            WindowMode::Windowed => WindowModeType::Windowed,
            WindowMode::Fullscreen { .. } => WindowModeType::Fullscreen,
            WindowMode::BorderlessFullscreen { .. } => WindowModeType::BorderlessFullscreen,
            _ => WindowModeType::Windowed,
        };

        let current_vsync = window.present_mode == PresentMode::AutoVsync;

        Self {
            resolution: match current_res {
                (800, 600)
                | (1280, 720)
                | (1366, 768)
                | (1600, 900)
                | (1920, 1080)
                | (2560, 1440)
                | (3840, 2160) => current_res,
                _ => (1920, 1080),
            },
            window_mode: current_mode,
            vsync: current_vsync,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum WindowModeType {
    #[default]
    Fullscreen,
    BorderlessFullscreen,
    Windowed,
}

pub struct DisplaySettingsPlugin;

impl Plugin for DisplaySettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NewDisplaySettings>()
            .add_event::<DisplaySettingsChanged>()
            .add_systems(Startup, initialize_display_settings)
            .add_systems(
                Update,
                (
                    handle_resolution_button,
                    handle_fullscreen_button,
                    handle_vsync_button,
                    handle_apply_button,
                    handle_display_settings_back_button,
                    update_display_settings_text,
                ),
            )
            .add_systems(OnEnter(AppState::SettingsMenu), load_display_settings)
            .add_systems(
                OnEnter(GameOverlayState::SettingsMenu),
                load_display_settings,
            );
    }
}

fn initialize_display_settings(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    commands.insert_resource(NewDisplaySettings::new(window));
}

fn load_display_settings(
    mut display_settings: ResMut<NewDisplaySettings>,
    window: Query<&Window>,
    mut event_writer: EventWriter<DisplaySettingsChanged>,
) {
    let window = window.single();
    let new_settings = NewDisplaySettings::new(window);

    // Update settings from window
    *display_settings = new_settings;

    // Trigger UI update
    event_writer.send(DisplaySettingsChanged);
}

#[derive(Component)]
struct ResolutionText;

#[derive(Component)]
struct FullscreenText;

#[derive(Component)]
struct VSyncText;

pub fn setup_display_settings(parent: &mut ChildBuilder, display_settings: &NewDisplaySettings) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                overflow: Overflow::clip_y(),
                ..default()
            },
            DisplaySettingsUI,
        ))
        .with_children(|parent| {
            // Settings container
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(SETTINGS_CONTAINER_HEIGHT),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    overflow: Overflow::clip_y(),
                    ..default()
                })
                .with_children(|parent| {
                    // Resolution setting
                    parent
                        .spawn(Node {
                            width: Val::Percent(SETTINGS_CONTAINER_WIDTH),
                            height: Val::Px(SETTING_ROW_HEIGHT),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: UiRect::all(Val::Px(SETTING_ROW_MARGIN)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Label
                            parent.spawn((
                                Text::new("Resolution"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            ));

                            // Button
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(BUTTON_WIDTH),
                                        height: Val::Px(BUTTON_HEIGHT),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(ButtonColors::default().normal),
                                    ButtonColors::default(),
                                    ResolutionButton,
                                ))
                                .with_child((
                                    Text::new(format!(
                                        "{}x{}",
                                        display_settings.resolution.0,
                                        display_settings.resolution.1
                                    )),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                                    ResolutionText,
                                ));
                        });

                    // Fullscreen setting
                    parent
                        .spawn(Node {
                            width: Val::Percent(SETTINGS_CONTAINER_WIDTH),
                            height: Val::Px(SETTING_ROW_HEIGHT),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: UiRect::all(Val::Px(SETTING_ROW_MARGIN)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Label
                            parent.spawn((
                                Text::new("Fullscreen"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            ));

                            // Button
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(BUTTON_WIDTH),
                                        height: Val::Px(BUTTON_HEIGHT),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(ButtonColors::default().normal),
                                    ButtonColors::default(),
                                    FullscreenButton,
                                ))
                                .with_child((
                                    Text::new(match display_settings.window_mode {
                                        WindowModeType::Windowed => "Windowed",
                                        WindowModeType::Fullscreen => "Fullscreen",
                                        WindowModeType::BorderlessFullscreen => "Borderless",
                                    }),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                                    FullscreenText,
                                ));
                        });

                    // VSync setting
                    parent
                        .spawn(Node {
                            width: Val::Percent(SETTINGS_CONTAINER_WIDTH),
                            height: Val::Px(SETTING_ROW_HEIGHT),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: UiRect::all(Val::Px(SETTING_ROW_MARGIN)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Label
                            parent.spawn((
                                Text::new("VSync"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            ));

                            // Button
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(BUTTON_WIDTH),
                                        height: Val::Px(BUTTON_HEIGHT),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(ButtonColors::default().normal),
                                    ButtonColors::default(),
                                    VSyncButton,
                                ))
                                .with_child((
                                    Text::new(if display_settings.vsync { "On" } else { "Off" }),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                                    VSyncText,
                                ));
                        });
                });

            // Bottom buttons
            create_bottom_buttons(parent);
        });
}

fn update_display_settings_text(
    mut text_queries: ParamSet<(
        Query<&mut Text, With<ResolutionText>>,
        Query<&mut Text, With<FullscreenText>>,
        Query<&mut Text, With<VSyncText>>,
    )>,
    display_settings: Res<NewDisplaySettings>,
    mut event_reader: EventReader<DisplaySettingsChanged>,
) {
    if event_reader.read().next().is_some() {
        println!("Display settings changed:");
        println!(
            "  Resolution: {}x{}",
            display_settings.resolution.0, display_settings.resolution.1
        );
        println!("  Window mode: {:?}", display_settings.window_mode);
        println!("  VSync: {}", display_settings.vsync);

        // Update resolution text
        if let Ok(mut text) = text_queries.p0().get_single_mut() {
            text.0 = format!(
                "{}x{}",
                display_settings.resolution.0, display_settings.resolution.1
            );
            println!("  Updated resolution text");
        } else {
            println!("  Failed to find resolution text");
        }

        // Update fullscreen mode text
        if let Ok(mut text) = text_queries.p1().get_single_mut() {
            text.0 = match display_settings.window_mode {
                WindowModeType::Windowed => "Windowed",
                WindowModeType::Fullscreen => "Fullscreen",
                WindowModeType::BorderlessFullscreen => "Borderless",
            }
            .to_string();
            println!("  Updated fullscreen text");
        } else {
            println!("  Failed to find fullscreen text");
        }

        // Update VSync text
        if let Ok(mut text) = text_queries.p2().get_single_mut() {
            text.0 = if display_settings.vsync { "On" } else { "Off" }.to_string();
            println!("  Updated VSync text");
        } else {
            println!("  Failed to find VSync text");
        }
    }
}

fn create_bottom_buttons(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(BOTTOM_BUTTON_HEIGHT),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::top(Val::Px(BOTTOM_BUTTONS_MARGIN)),
            ..default()
        })
        .with_children(|parent| {
            // Apply button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(BUTTON_WIDTH),
                        height: Val::Px(BOTTOM_BUTTON_HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    ApplyButton,
                ))
                .with_child((
                    Text::new("Apply"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));

            // Back button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(BUTTON_WIDTH),
                        height: Val::Px(BOTTOM_BUTTON_HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    DisplaySettingsBackButton,
                ))
                .with_child((
                    Text::new("Back"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });
}

#[derive(Component)]
struct DisplaySettingsUI;

#[derive(Component)]
pub struct ResolutionButton;

#[derive(Component)]
pub struct FullscreenButton;

#[derive(Component)]
pub struct VSyncButton;

#[derive(Component)]
struct ApplyButton;

#[derive(Component)]
struct DisplaySettingsBackButton;

fn handle_resolution_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (With<ResolutionButton>, Changed<Interaction>),
    >,
    mut display_settings: ResMut<NewDisplaySettings>,
    mut event_writer: EventWriter<DisplaySettingsChanged>,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Cycle through common resolutions
                let new_res = match display_settings.resolution {
                    (800, 600) => (1280, 720),
                    (1280, 720) => (1366, 768),
                    (1366, 768) => (1600, 900),
                    (1600, 900) => (1920, 1080),
                    (1920, 1080) => (2560, 1440),
                    (2560, 1440) => (3840, 2160),
                    (3840, 2160) => (800, 600),
                    _ => (1920, 1080), // Default if somehow in an unknown state
                };
                display_settings.resolution = new_res;
                event_writer.send(DisplaySettingsChanged);
                *color = button_colors.active.into();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn handle_fullscreen_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (With<FullscreenButton>, Changed<Interaction>),
    >,
    mut display_settings: ResMut<NewDisplaySettings>,
    mut event_writer: EventWriter<DisplaySettingsChanged>,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Cycle through window modes
                display_settings.window_mode = match display_settings.window_mode {
                    WindowModeType::Windowed => WindowModeType::Fullscreen,
                    WindowModeType::Fullscreen => WindowModeType::BorderlessFullscreen,
                    WindowModeType::BorderlessFullscreen => WindowModeType::Windowed,
                };
                event_writer.send(DisplaySettingsChanged);
                *color = button_colors.active.into();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn handle_vsync_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (With<VSyncButton>, Changed<Interaction>),
    >,
    mut display_settings: ResMut<NewDisplaySettings>,
    mut event_writer: EventWriter<DisplaySettingsChanged>,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                display_settings.vsync = !display_settings.vsync;
                event_writer.send(DisplaySettingsChanged);
                *color = button_colors.active.into();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn handle_apply_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (With<ApplyButton>, Changed<Interaction>),
    >,
    mut window: Query<&mut Window>,
    display_settings: Res<NewDisplaySettings>,
    mut render_resolution: ResMut<GameRenderResolution>,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Apply changes to window
                if let Ok(mut window) = window.get_single_mut() {
                    // Update the render resolution
                    render_resolution.width = display_settings.resolution.0 as f32;
                    render_resolution.height = display_settings.resolution.1 as f32;

                    // In fullscreen modes, keep the window at native resolution
                    let (width, height) = match display_settings.window_mode {
                        WindowModeType::Windowed => display_settings.resolution,
                        _ => (
                            window.resolution.physical_width(),
                            window.resolution.physical_height(),
                        ),
                    };

                    // Apply window mode first
                    match display_settings.window_mode {
                        WindowModeType::Windowed => {
                            window.mode = WindowMode::Windowed;
                            window.position = WindowPosition::Automatic;
                        }
                        WindowModeType::Fullscreen => {
                            window.mode = WindowMode::Fullscreen(MonitorSelection::Current);
                        }
                        WindowModeType::BorderlessFullscreen => {
                            window.mode =
                                WindowMode::BorderlessFullscreen(MonitorSelection::Current);
                        }
                    }

                    // Then set the window resolution
                    window.resolution = WindowResolution::new(width as f32, height as f32);

                    // Apply other settings
                    window.present_mode = if display_settings.vsync {
                        PresentMode::AutoVsync
                    } else {
                        PresentMode::AutoNoVsync
                    };
                }
                *color = button_colors.active.into();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn handle_display_settings_back_button(
    mut next_state: ResMut<NextState<AppState>>,
    mut next_overlay_state: ResMut<NextState<GameOverlayState>>,
    interaction_query: Query<&Interaction, (With<DisplaySettingsBackButton>, Changed<Interaction>)>,
    app_state: Res<State<AppState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            match app_state.get() {
                AppState::SettingsMenu => {
                    next_state.set(AppState::MainMenu);
                }
                AppState::InGame => {
                    next_overlay_state.set(GameOverlayState::GameMenu);
                }
                _ => {}
            }
        }
    }
}
