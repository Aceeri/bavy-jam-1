use bevy::prelude::*;

pub mod camera;
pub mod health;
pub mod player;
pub mod rat;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(health::plugin)
        .add_plugins(player::plugin)
        .add_plugins(rat::plugin)
        .add_plugins(camera::plugin);

    app.run();
}
