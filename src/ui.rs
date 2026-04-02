use bevy::prelude::*;
use bevy::scene2::{CommandsSceneExt, bsn};

use crate::pit::RatPit;
use crate::rat::RatCounter;

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

    pub broom_cost: u32,
    pub spawn_cost: u32,
    pub pit_cost: u32,
}

impl Default for Upgrades {
    fn default() -> Self {
        Self {
            broom_length: 0.1,
            spawn_interval: 1.0,
            pit_size: 0.3,

            broom_cost: 5,
            spawn_cost: 5,
            pit_cost: 10,
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
        **text = format!("Rats: {}", counter.0);
    }
}

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
                if counter.0 >= upgrades.broom_cost {
                    counter.0 -= upgrades.broom_cost;
                    upgrades.broom_length += 0.1;
                    upgrades.broom_cost = (upgrades.broom_cost as f32 * 1.4) as u32;
                }
            }
            UpgradeButton::SpawnRate => {
                if counter.0 >= upgrades.spawn_cost {
                    counter.0 -= upgrades.spawn_cost;
                    upgrades.spawn_interval = (upgrades.spawn_interval * 0.85).max(0.01);
                    upgrades.spawn_cost = (upgrades.spawn_cost as f32 * 1.2) as u32;
                }
            }
            UpgradeButton::PitSize => {
                if counter.0 >= upgrades.pit_cost {
                    counter.0 -= upgrades.pit_cost;
                    upgrades.pit_size += 0.1;
                    pit.half_size = upgrades.pit_size;
                    upgrades.pit_cost = (upgrades.pit_cost as f32 * 1.2) as u32;
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
                    UpgradeButton::Broom => format!("Broom [{}]", upgrades.broom_cost),
                    UpgradeButton::SpawnRate => format!("Spawn [{}]", upgrades.spawn_cost),
                    UpgradeButton::PitSize => format!("Pit [{}]", upgrades.pit_cost),
                };
            }
        }
    }
}
