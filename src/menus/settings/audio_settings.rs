use crate::menus::settings::ui_constants::{
    SETTINGS_CONTAINER_WIDTH, SETTING_ROW_HEIGHT, SETTING_ROW_MARGIN,
};
use crate::menus::ui_components::ButtonColors;
use bevy::prelude::*;

pub struct AudioSettingsPlugin;

impl Plugin for AudioSettingsPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: We'll add systems here later when we implement the functionality
    }
}

#[derive(Component)]
pub struct AudioSettings;

pub fn setup_audio_settings(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                overflow: Overflow::clip_y(),
                ..default()
            },
            AudioSettings,
        ))
        .with_children(|parent| {
            // Scroll container
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    overflow: Overflow::clip_y(),
                    ..default()
                })
                .with_children(|parent| {
                    // Master Volume setting
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
                            parent.spawn((
                                Text::new("Master Volume"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            ));
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(ButtonColors::default().normal),
                                    ButtonColors::default(),
                                ))
                                .with_child((
                                    Text::new("100%"),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                                ));
                        });

                    // Music Volume setting
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
                            parent.spawn((
                                Text::new("Music Volume"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            ));
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(ButtonColors::default().normal),
                                    ButtonColors::default(),
                                ))
                                .with_child((
                                    Text::new("80%"),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                                ));
                        });

                    // Sound Effects Volume setting
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
                            parent.spawn((
                                Text::new("Sound Effects"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                            ));
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(ButtonColors::default().normal),
                                    ButtonColors::default(),
                                ))
                                .with_child((
                                    Text::new("90%"),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                                ));
                        });
                });
        });
}
