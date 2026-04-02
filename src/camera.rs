use bevy::{
    prelude::*,
    scene2::{Scene, bsn},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_ratmera);
}

#[derive(Component, Reflect, Debug, FromTemplate)]
pub struct Ratmera;

#[derive(Component, Reflect, Debug, FromTemplate)]
pub struct RatmeraTarget(pub Entity);

pub fn ratmera() -> impl Scene {
    bsn! {
        #RAT_CAMERA
        Camera
        Camera3d
        Ratmera
    }
}

pub fn update_ratmera(
    globals: Query<&GlobalTransform>,
    mut ratmeras: Query<(&mut Transform, &RatmeraTarget), With<Ratmera>>,
) {
    for (mut transform, target) in &mut ratmeras {
        let Ok(target_global) = globals.get(target.0) else {
            continue;
        };

        transform.translation = target_global.translation() + Vec3::new(0.0, 3.0, 2.0);
        transform.look_at(target_global.translation(), Vec3::Y)
    }
}
