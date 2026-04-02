//! Ur a RAT, a devious RAT, RAT.
//!

use bevy::prelude::*;

use crate::rat::Rat;

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, movement);
}

#[derive(Component, Reflect, Default, Debug)]
#[require(Rat, PlayerInput)]
pub struct Player;

#[derive(Component, Reflect, Default, Debug)]
pub struct PlayerInput {
    // movement inputs
    pub movement: Vec2,
    // primary/secondary attack inputs
    pub primary: bool,
    pub secondary: bool,
}

pub fn inputs(
    mouse: Res<ButtonInput<MouseButton>>,
    button: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut PlayerInput,)>,
) {
    let player_input = &mut *player.0;

    if button.pressed(KeyCode::KeyW) {
        player_input.movement.y -= 1.;
    }

    if button.pressed(KeyCode::KeyS) {
        player_input.movement.y += 1.;
    }

    if button.pressed(KeyCode::KeyA) {
        player_input.movement.x -= 1.;
    }

    if button.pressed(KeyCode::KeyD) {
        player_input.movement.x += 1.;
    }

    player_input.primary = mouse.pressed(MouseButton::Left);
    player_input.secondary = mouse.pressed(MouseButton::Right);
}

// TODO: movement, we on a 2d plane for now, 3d is just for visuals
pub fn movement(time: Res<Time>, mut players: Query<(&mut Transform, &PlayerInput)>) {
    for (mut transform, input) in &mut players {
        transform.translation.x += input.movement.x * time.delta_secs();
        transform.translation.z += input.movement.y * time.delta_secs();
    }
}
