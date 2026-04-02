use bevy::prelude::*;
use bevy::scene2::{CommandsSceneExt, bsn};

use crate::pit::RatPit;
use crate::rat::{BOUNDING_RANGE, RatCounter};

pub fn plugin(app: &mut App) {
    app.init_resource::<Upgrades>()
        .add_systems(Startup, spawn_ui)
        .add_systems(
            Update,
            (
                update_rat_counter,
                handle_upgrade_buttons,
                update_upgrade_text,
            ),
        );
}

#[derive(Resource)]
pub struct Upgrades {
    pub broom_length: f32,
    pub spawn_interval: f32,
    pub pit_size: f32,

    pub broom_cost: f32,
    pub spawn_cost: f32,
    pub pit_cost: f32,
}

impl Default for Upgrades {
    fn default() -> Self {
        Self {
            broom_length: 0.1,
            spawn_interval: 1.0,
            pit_size: 0.3,

            broom_cost: 5.0,
            spawn_cost: 5.0,
            pit_cost: 10.0,
        }
    }
}

#[derive(Component, Default, Clone, Reflect, Debug)]
struct RatCounterText;

#[derive(Component)]
enum UpgradeButton {
    Broom,
    SpawnRate,
    PitSize,
}

#[derive(Component, Default, Clone, Reflect, Debug)]
struct UpgradeButtonText;

fn spawn_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        RatCounterText
        Text("Rats: 0")
        TextColor(Color::WHITE)
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(120.0),
            left: Val::Px(12.0),
        }
    });

    // TODO: bsn-ify this
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|parent| {
            spawn_button(parent, UpgradeButton::Broom, "Broom [5]");
            spawn_button(parent, UpgradeButton::SpawnRate, "Spawn [5]");
            spawn_button(parent, UpgradeButton::PitSize, "Pit [10]");
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, button: UpgradeButton, label: &str) {
    parent
        .spawn((
            button,
            Button,
            Node {
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|child| {
            child.spawn((
                UpgradeButtonText,
                Text::new(label.to_string()),
                TextColor(Color::WHITE),
            ));
        });
}

fn update_rat_counter(counter: Res<RatCounter>, mut query: Query<&mut Text, With<RatCounterText>>) {
    if !counter.is_changed() {
        return;
    }

    for mut text in &mut query {
        **text = format!("Rats: {} (+{}/s)", counter.total, counter.per_second);
    }
}

const MINIMUM_SPAWN_RATE: f32 = 0.001;
const MAX_BROOM: f32 = BOUNDING_RANGE * std::f32::consts::SQRT_2;

fn handle_upgrade_buttons(
    interactions: Query<(&Interaction, &UpgradeButton), Changed<Interaction>>,
    mut upgrades: ResMut<Upgrades>,
    mut counter: ResMut<RatCounter>,
    mut pit: ResMut<RatPit>,
) {
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            UpgradeButton::Broom => {
                if upgrades.broom_length >= MAX_BROOM {
                    continue;
                }

                let cost = upgrades.broom_cost as u32;
                if counter.total >= cost {
                    counter.total -= cost;
                    upgrades.broom_length = (upgrades.broom_length + 0.15).min(MAX_BROOM);
                    upgrades.broom_cost *= 1.2;
                }
            }
            UpgradeButton::SpawnRate => {
                if upgrades.spawn_interval <= MINIMUM_SPAWN_RATE {
                    continue;
                }

                let cost = upgrades.spawn_cost as u32;
                if counter.total >= cost {
                    counter.total -= cost;
                    upgrades.spawn_interval =
                        (upgrades.spawn_interval * 0.9).max(MINIMUM_SPAWN_RATE);
                    upgrades.spawn_cost *= 1.15;
                }
            }
            UpgradeButton::PitSize => {
                if upgrades.pit_size >= BOUNDING_RANGE {
                    continue;
                }

                let cost = upgrades.pit_cost as u32;
                if counter.total >= cost {
                    counter.total -= cost;
                    upgrades.pit_size = (upgrades.pit_size + 0.1).min(BOUNDING_RANGE);
                    pit.half_size = upgrades.pit_size;
                    upgrades.pit_cost *= 1.1;
                }
            }
        }
    }
}

fn update_upgrade_text(
    upgrades: Res<Upgrades>,
    buttons: Query<(&UpgradeButton, &Children)>,
    mut texts: Query<&mut Text, With<UpgradeButtonText>>,
) {
    if !upgrades.is_changed() {
        return;
    }

    for (button, children) in &buttons {
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                **text = match button {
                    UpgradeButton::Broom => {
                        let cost = if upgrades.broom_length >= MAX_BROOM {
                            "MAXED".to_owned()
                        } else {
                            format!("{}", upgrades.broom_cost as u32)
                        };
                        format!("Broom [{}]", cost)
                    }
                    UpgradeButton::SpawnRate => {
                        let cost = if upgrades.spawn_interval <= MINIMUM_SPAWN_RATE {
                            "MAXED".to_owned()
                        } else {
                            format!("{}", upgrades.spawn_cost as u32)
                        };

                        format!("Spawn [{}]", cost)
                    }
                    UpgradeButton::PitSize => {
                        let cost = if upgrades.pit_size >= BOUNDING_RANGE {
                            "MAXED".to_owned()
                        } else {
                            format!("{}", upgrades.pit_cost as u32)
                        };
                        format!("Pit [{}]", cost)
                    }
                };
            }
        }
    }
}
