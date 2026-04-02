use bevy::{
    prelude::*,
    scene2::{CommandsSceneExt, Scene, bsn},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, move_camera);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_scene(top_down_camera());

    commands.spawn((
        Name::new("Ground plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
}

#[derive(Component, Reflect, Debug, FromTemplate)]
pub struct TopDownCamera {
    pub speed: f32,
}

pub fn top_down_camera() -> impl Scene {
    bsn! {
        Camera3d
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
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
    }
}
