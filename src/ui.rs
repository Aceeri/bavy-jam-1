use bevy::prelude::*;
use bevy::scene2::{CommandsSceneExt, bsn};

use crate::rat::RatCounter;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui)
        .add_systems(Update, update_rat_counter);
}

#[derive(Component, Default, Clone, Reflect, Debug)]
struct RatCounterText;

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
}

fn update_rat_counter(counter: Res<RatCounter>, mut query: Query<&mut Text, With<RatCounterText>>) {
    if !counter.is_changed() {
        return;
    }

    for mut text in &mut query {
        **text = format!("Rats: {}", counter.0);
    }
}

// upgrades?
