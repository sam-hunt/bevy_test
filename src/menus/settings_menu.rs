use crate::menus::settings::*;
use crate::menus::ui_components::ButtonColors;
use crate::overlay_state::GameOverlayState;
use crate::AppState;
use bevy::prelude::*;

pub struct SettingsMenuPlugin;

/// This plugin is responsible for the settings menu
/// The menu is drawn during either:
/// - The State `AppState::SettingsMenu` (from main menu)
/// - The State `GameOverlayState::SettingsMenu` (in-game)
impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DisplaySettingsPlugin,
            AudioSettingsPlugin,
            ControlsSettingsPlugin,
        ))
        .add_systems(
            OnEnter(AppState::SettingsMenu),
            setup_settings_menu.run_if(not(in_state(GameOverlayState::SettingsMenu))),
        )
        .add_systems(
            OnEnter(GameOverlayState::SettingsMenu),
            setup_settings_menu.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                handle_tab_button_hover,
                handle_regular_button_hover,
                handle_tab_buttons,
            )
                .run_if(
                    in_state(AppState::SettingsMenu).or(in_state(GameOverlayState::SettingsMenu)),
                ),
        )
        .add_systems(
            OnExit(AppState::SettingsMenu),
            cleanup_settings_menu.run_if(not(in_state(GameOverlayState::SettingsMenu))),
        )
        .add_systems(
            OnExit(GameOverlayState::SettingsMenu),
            cleanup_settings_menu.run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component)]
struct SettingsMenu;

#[derive(Component)]
struct TabButton {
    tab_index: usize,
}

#[derive(Component)]
struct TabContent {
    tab_index: usize,
}

#[derive(Component)]
struct ActiveButton;

fn setup_settings_menu(mut commands: Commands, display_settings: Res<NewDisplaySettings>) {
    // Menu container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SettingsMenu,
        ))
        .with_children(|children| {
            // Title
            children.spawn((
                Text::new("Settings"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Tab buttons container
            children
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    // Display Tab
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::right(Val::Px(10.0)),
                                ..Default::default()
                            },
                            BackgroundColor(ButtonColors::default().normal),
                            ButtonColors::default(),
                            TabButton { tab_index: 0 },
                            ActiveButton,
                        ))
                        .with_child((
                            Text::new("Display"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                        ));

                    // Audio Tab
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::right(Val::Px(10.0)),
                                ..Default::default()
                            },
                            BackgroundColor(ButtonColors::default().normal),
                            ButtonColors::default(),
                            TabButton { tab_index: 1 },
                        ))
                        .with_child((
                            Text::new("Audio"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                        ));

                    // Controls Tab
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BackgroundColor(ButtonColors::default().normal),
                            ButtonColors::default(),
                            TabButton { tab_index: 2 },
                        ))
                        .with_child((
                            Text::new("Controls"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                        ));
                });

            // Tab content container
            children
                .spawn(Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(40.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    // Display tab content
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            TabContent { tab_index: 0 },
                        ))
                        .with_children(|parent| {
                            setup_display_settings(parent, &display_settings);
                        });

                    // Audio tab content
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                display: Display::None,
                                ..default()
                            },
                            TabContent { tab_index: 1 },
                        ))
                        .with_children(|parent| {
                            setup_audio_settings(parent);
                        });

                    // Controls tab content
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                display: Display::None,
                                ..default()
                            },
                            TabContent { tab_index: 2 },
                        ))
                        .with_children(|parent| {
                            setup_controls_settings(parent);
                        });
                });
        });
}

fn handle_tab_button_hover(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            &TabButton,
            Option<&ActiveButton>,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut color, button_colors, _tab_button, is_active) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                if is_active.is_some() {
                    *color = Color::linear_rgb(0.4, 0.4, 0.4).into();
                } else {
                    *color = button_colors.hovered.into();
                }
            }
            Interaction::None => {
                if is_active.is_some() {
                    *color = button_colors.active.into();
                } else {
                    *color = button_colors.normal.into();
                }
            }
            Interaction::Pressed => {}
        }
    }
}

fn handle_regular_button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (Changed<Interaction>, Without<TabButton>),
    >,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
            Interaction::Pressed => {}
        }
    }
}

fn handle_tab_buttons(
    interaction_query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
    mut tab_content_query: Query<(&mut Node, &TabContent)>,
    mut commands: Commands,
    mut tab_button_query: Query<
        (Entity, &mut BackgroundColor, &ButtonColors, &TabButton),
        With<Button>,
    >,
) {
    for (interaction, pressed_tab) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Remove ActiveButton from all tab buttons
            for (entity, _, _, _) in &tab_button_query {
                commands.entity(entity).remove::<ActiveButton>();
            }

            // Add ActiveButton to the clicked tab
            for (entity, _, _, button) in &tab_button_query {
                if button.tab_index == pressed_tab.tab_index {
                    commands.entity(entity).insert(ActiveButton);
                }
            }

            // Update tab content visibility
            for (mut node, tab_content) in &mut tab_content_query {
                node.display = if tab_content.tab_index == pressed_tab.tab_index {
                    Display::Flex
                } else {
                    Display::None
                };
            }

            // Update colors for ALL tab buttons
            for (_, mut color, button_colors, button) in &mut tab_button_query {
                if button.tab_index == pressed_tab.tab_index {
                    *color = button_colors.active.into();
                } else {
                    *color = button_colors.normal.into();
                }
            }
        }
    }
}

fn cleanup_settings_menu(mut commands: Commands, menu: Query<Entity, With<SettingsMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
