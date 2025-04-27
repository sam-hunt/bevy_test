use crate::menus::ui_components::ButtonColors;
use crate::overlay_state::GameOverlayState;
use crate::AppState;
use bevy::prelude::*;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameOverlayState::GameMenu), setup_game_menu)
            .add_systems(
                Update,
                (
                    handle_button_hover,
                    handle_resume_button,
                    handle_settings_button,
                    handle_quit_button,
                    handle_quit_to_os_button,
                )
                    .run_if(in_state(GameOverlayState::GameMenu)),
            )
            .add_systems(OnExit(GameOverlayState::GameMenu), cleanup_game_menu);
    }
}

#[derive(Component)]
struct GameMenu;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct QuitToOSButton;

fn setup_game_menu(mut commands: Commands) {
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
            GameMenu,
        ))
        .with_children(|children| {
            // Title
            children.spawn((
                Text::new("Game Menu"),
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

            // Resume Button
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    ResumeButton,
                ))
                .with_child((
                    Text::new("Resume"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));

            // Settings Button
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    SettingsButton,
                ))
                .with_child((
                    Text::new("Settings"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));

            // Quit to Main Menu Button
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    QuitButton,
                ))
                .with_child((
                    Text::new("Quit to Menu"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));

            // Quit to OS Button
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    QuitToOSButton,
                ))
                .with_child((
                    Text::new("Quit to OS"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });
}

fn handle_button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        Changed<Interaction>,
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

fn handle_resume_button(
    mut next_overlay_state: ResMut<NextState<GameOverlayState>>,
    interaction_query: Query<&Interaction, (With<ResumeButton>, Changed<Interaction>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_overlay_state.set(GameOverlayState::None);
        }
    }
}

fn handle_settings_button(
    mut next_overlay_state: ResMut<NextState<GameOverlayState>>,
    interaction_query: Query<&Interaction, (With<SettingsButton>, Changed<Interaction>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_overlay_state.set(GameOverlayState::SettingsMenu);
        }
    }
}

fn handle_quit_button(
    mut next_state: ResMut<NextState<AppState>>,
    mut next_overlay_state: ResMut<NextState<GameOverlayState>>,
    interaction_query: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_overlay_state.set(GameOverlayState::None);
            next_state.set(AppState::MainMenu);
        }
    }
}

fn handle_quit_to_os_button(
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    interaction_query: Query<&Interaction, (With<QuitToOSButton>, Changed<Interaction>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            app_exit_events.send_default();
        }
    }
}

fn cleanup_game_menu(mut commands: Commands, menu: Query<Entity, With<GameMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
