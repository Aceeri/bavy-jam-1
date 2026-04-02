use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;

pub mod camera;
pub mod health;
pub mod pit;
pub mod player;
pub mod rat;
pub mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin::default())
        .insert_resource(GlobalAmbientLight::NONE)
        .add_plugins(health::plugin)
        .add_plugins(player::plugin)
        .add_plugins(rat::plugin)
        .add_plugins(pit::plugin)
        .add_plugins(camera::plugin)
        .add_plugins(ui::plugin);

    app.run();
}
