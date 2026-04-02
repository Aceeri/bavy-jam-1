use bevy::{
    prelude::*,
    scene2::{CommandsSceneExt, Scene, bsn},
};

#[cfg(not(target_arch = "wasm32"))]
use bevy::light::{VolumetricFog, VolumetricLight};
#[cfg(not(target_arch = "wasm32"))]
use bevy::render::occlusion_culling::OcclusionCulling;

use crate::rat::BOUNDING_RANGE;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, move_camera);
}

fn setup(mut commands: Commands) {
    let camera = commands.spawn_scene(top_down_camera()).id();
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(camera).insert((
        VolumetricFog {
            ambient_intensity: 0.0,
            step_count: 32,
            ..default()
        },
        OcclusionCulling,
    ));

    let light = commands.spawn((
        DirectionalLight {
            illuminance: 100.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    )).id();
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(light).insert(VolumetricLight);
}

#[derive(Component, Reflect, Debug, FromTemplate)]
pub struct TopDownCamera {
    pub speed: f32,
}

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 13.5, 10.5);

pub fn top_down_camera() -> impl Scene {
    bsn! {
        Camera3d
        Transform {
            translation: {CAMERA_OFFSET},
            rotation: Quat::from_rotation_x(-56.0_f32.to_radians()),
        }
        TopDownCamera { speed: 10.0 }
    }
}

pub fn move_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(&mut Transform, &TopDownCamera)>,
) {
    for (mut transform, camera) in &mut cameras {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        transform.translation += direction * camera.speed * time.delta_secs();

        let mut target = transform.translation - CAMERA_OFFSET;
        target.x = target.x.clamp(-BOUNDING_RANGE, BOUNDING_RANGE);
        target.z = target.z.clamp(-BOUNDING_RANGE, BOUNDING_RANGE);
        transform.translation = target + CAMERA_OFFSET;
    }
}
