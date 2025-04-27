use crate::loading::TextureAssets;
use crate::menus::ui_components::ButtonColors;
use crate::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

/// This plugin is responsible for the main menu
/// The menu is only drawn during the State `AppState::MainMenu` and is removed when that state is exited
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (
                    handle_button_hover,
                    handle_main_menu_play_button,
                    handle_main_menu_settings_button,
                    handle_main_menu_quit_button,
                    handle_external_links,
                )
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct MainMenuPlayButton;

#[derive(Component)]
struct MainMenuSettingsButton;

#[derive(Component)]
struct MainMenuQuitButton;

#[derive(Component)]
struct OpenLink(&'static str);

fn setup_main_menu(mut commands: Commands, textures: Res<TextureAssets>) {
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
            MainMenu,
        ))
        .with_children(|children: &mut ChildBuilder<'_>| {
            // TODO: Game Title TBD
            children.spawn((
                Text::new("Untitled Game"),
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

            // Play Button
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    MainMenuPlayButton,
                ))
                .with_child((
                    Text::new("Play"),
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
                        width: Val::Px(250.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    MainMenuSettingsButton,
                ))
                .with_child((
                    Text::new("Settings"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));

            // Quit Button
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    BackgroundColor(ButtonColors::default().normal),
                    ButtonColors::default(),
                    MainMenuQuitButton,
                ))
                .with_child((
                    Text::new("Quit"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });

    // Footer buttons
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(5.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            MainMenu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors::default(),
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: textures.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors::default(),
                    OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Open source"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode::new(textures.github.clone()),
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
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

fn handle_main_menu_play_button(
    mut next_state: ResMut<NextState<AppState>>,
    interaction_query: Query<&Interaction, (With<MainMenuPlayButton>, Changed<Interaction>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::InGame);
        }
    }
}

fn handle_main_menu_settings_button(
    mut next_state: ResMut<NextState<AppState>>,
    interaction_query: Query<&Interaction, (With<MainMenuSettingsButton>, Changed<Interaction>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::SettingsMenu);
        }
    }
}

fn handle_main_menu_quit_button(
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    interaction_query: Query<&Interaction, (With<MainMenuQuitButton>, Changed<Interaction>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            app_exit_events.send_default();
        }
    }
}

fn handle_external_links(
    interaction_query: Query<(&Interaction, &OpenLink), Changed<Interaction>>,
) {
    for (interaction, link) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Err(error) = webbrowser::open(link.0) {
                warn!("Failed to open link {error:?}");
            }
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, menu: Query<Entity, With<MainMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
