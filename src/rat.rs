use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::SmallRng};

use crate::pit::RatPit;
use crate::ui::Upgrades;

#[derive(Resource, Default, Reflect)]
pub struct RatCounter(pub u32);

pub fn plugin(app: &mut App) {
    // app.insert_resource(RatCounter(100_000_000));
    app.insert_resource(RatCounter(0));
    app.add_systems(Startup, setup_rat_resources)
        .add_systems(
            Update,
            (strip_skinning_data, spawn_rats_over_time, cursor_push),
        )
        .add_systems(
            FixedUpdate,
            (pit_fall, fall, apply_velocity, apply_damping).chain(),
        );
}

#[derive(Component, Default, Debug, Reflect, Clone)]
pub struct Rat;

#[derive(Component, Default, Debug, Reflect, Clone)]
pub struct Velocity(pub Vec3);

#[derive(Resource)]
struct RatMesh(Handle<Mesh>);

#[derive(Resource)]
struct RatMaterial(Handle<StandardMaterial>);

#[derive(Resource)]
struct RatSpawner {
    timer: Timer,
    rng: SmallRng,
}

pub const BOUNDING_RANGE: f32 = 7.0;
pub const SPAWN_RANGE: f32 = 6.0;
pub const BROOM_STRENGTH: f32 = 20.0;
pub const DAMPING: f32 = 12.0;

#[derive(Resource, Default)]
struct BroomState {
    last_world_pos: Option<Vec3>,
}

fn setup_rat_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    upgrades: Res<Upgrades>,
) {
    let mesh: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("rat.glb"),
    );
    let material = materials.add(StandardMaterial::default());

    commands.insert_resource(RatMesh(mesh));
    commands.insert_resource(RatMaterial(material));
    commands.insert_resource(RatSpawner {
        timer: Timer::from_seconds(upgrades.spawn_interval, TimerMode::Repeating),
        rng: SmallRng::from_rng(&mut rand::rng()),
    });
    commands.insert_resource(BroomState::default());
}

fn spawn_rats_over_time(
    mut commands: Commands,
    mut spawner: ResMut<RatSpawner>,
    rat_mesh: Option<Res<RatMesh>>,
    rat_material: Option<Res<RatMaterial>>,
    pit: Res<RatPit>,
    upgrades: Res<Upgrades>,
    time: Res<Time>,
) {
    let (Some(mesh), Some(material)) = (rat_mesh, rat_material) else {
        return;
    };

    spawner
        .timer
        .set_duration(std::time::Duration::from_secs_f32(upgrades.spawn_interval));
    spawner.timer.tick(time.delta());

    for _ in 0..spawner.timer.times_finished_this_tick() {
        // let (x, z) = rand_pos_outside_pit(&mut spawner.rng, pit.half_size);
        let (x, z) = rand_pos(&mut spawner.rng, pit.half_size);

        commands.spawn((
            Rat,
            Velocity::default(),
            Mesh3d(mesh.0.clone()),
            MeshMaterial3d(material.0.clone()),
            Transform::from_translation(Vec3::new(x, 0.0, z)),
        ));
    }
}

// 0..1 on X and Z, if its above 0.5 then its on the positive side of the pit, and vice versa
// we also need to pick an axis to avoid the pit from, though this kind of leads to accumulation
// in the corners... whatever, maybe some sort of trapezoidal uniform picking would be better but 24 hour jam
//
// otherwise we end up spawning in the pit most of the time and the pit upgrade becomes trash
// fn rand_pos_outside_pit(rng: &mut SmallRng, pit_half: f32) -> (f32, f32) {
//     let rand_axis = |rng: &mut SmallRng| -> f32 {
//         let t: f32 = rng.random_range(0.0..1.0);
//         let pos = pit_half + (BOUNDING_RANGE - pit_half) * ((t * 2.0).fract());
//         if t < 0.5 { -pos } else { pos }
//     };
//     let rand_free =
//         |rng: &mut SmallRng| -> f32 { rng.random_range(-BOUNDING_RANGE..BOUNDING_RANGE) };

//     if rng.random_range(0.0..1.0) < 0.5 {
//         (rand_axis(rng), rand_free(rng))
//     } else {
//         (rand_free(rng), rand_axis(rng))
//     }
// }

// actually just let the user get automated rats too
fn rand_pos(rng: &mut SmallRng, pit_half: f32) -> (f32, f32) {
    (
        rng.random_range(-BOUNDING_RANGE..BOUNDING_RANGE),
        rng.random_range(-BOUNDING_RANGE..BOUNDING_RANGE),
    )
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Rat>>) {
    let dt = time.delta_secs();
    for (mut transform, vel) in &mut query {
        transform.translation += vel.0 * dt;
        transform.translation.x = transform
            .translation
            .x
            .clamp(-BOUNDING_RANGE, BOUNDING_RANGE);
        transform.translation.z = transform
            .translation
            .z
            .clamp(-BOUNDING_RANGE, BOUNDING_RANGE);

        let flat_vel = Vec3::new(vel.0.x, 0.0, vel.0.z);
        if flat_vel.length_squared() > 0.01 {
            let target = Quat::look_to_rh(flat_vel, Vec3::Y);
            transform.rotation = transform.rotation.slerp(target, (dt * 10.0).min(1.0));
        }
    }
}

fn apply_damping(time: Res<Time>, mut query: Query<&mut Velocity, With<Rat>>) {
    let dt = time.delta_secs();
    for mut vel in &mut query {
        vel.0.x *= (1.0 - DAMPING * dt).max(0.0);
        vel.0.z *= (1.0 - DAMPING * dt).max(0.0);
    }
}

const PIT_FALL_ACCEL: f32 = 0.9;
const PIT_DESPAWN_Y: f32 = -15.0;

#[derive(Component, Reflect, Debug)]
pub struct Falling;

fn pit_fall(
    mut commands: Commands,
    pit: Res<RatPit>,
    mut rats: Query<(Entity, &GlobalTransform), (With<Rat>, Without<Falling>)>,
) {
    for (entity, transform) in &mut rats {
        let pos = transform.translation();
        if pos.x.abs() < pit.half_size && pos.z.abs() < pit.half_size {
            commands.entity(entity).insert(Falling);
        }
    }
}

pub fn fall(
    mut commands: Commands,
    mut counter: ResMut<RatCounter>,
    pit: Res<RatPit>,
    mut rats: Query<(Entity, &mut Transform, &mut Velocity), (With<Rat>, With<Falling>)>,
) {
    for (entity, mut transform, mut vel) in &mut rats {
        vel.0.y -= PIT_FALL_ACCEL;

        // hit the pit walls on the way down
        transform.translation.x = transform.translation.x.clamp(-pit.half_size, pit.half_size);
        transform.translation.z = transform.translation.z.clamp(-pit.half_size, pit.half_size);

        if transform.translation.y < PIT_DESPAWN_Y {
            commands.entity(entity).despawn();
            counter.0 += 1;
        }
    }
}

fn cursor_to_ground(
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec3> {
    let window = windows.single().ok()?;
    let cursor_pos = window.cursor_position()?;
    let (camera, camera_transform) = cameras.single().ok()?;
    let ray = camera
        .viewport_to_world(camera_transform, cursor_pos)
        .ok()?;
    if ray.direction.y.abs() < 1e-6 {
        return None;
    }
    let t = -ray.origin.y / ray.direction.y;
    if t < 0.0 {
        return None;
    }
    Some(ray.origin + ray.direction * t)
}

fn cursor_push(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut rats: Query<(&Transform, &mut Velocity), With<Rat>>,
    mut broom: ResMut<BroomState>,
    upgrades: Res<Upgrades>,
) {
    let world_pos = cursor_to_ground(&windows, &cameras);

    if !mouse.pressed(MouseButton::Left) {
        broom.last_world_pos = None;
        return;
    }

    let Some(current) = world_pos else {
        broom.last_world_pos = None;
        return;
    };

    if let Some(last) = broom.last_world_pos {
        let sweep = current - last;
        let sweep_len = sweep.xz().length();

        if sweep_len > 0.01 {
            let sweep_dir = Vec3::new(sweep.x, 0.0, sweep.z).normalize();
            let perp = Vec3::new(-sweep_dir.z, 0.0, sweep_dir.x);

            let bristle_half = upgrades.broom_length;
            let thickness = 0.5;
            let a = current - perp * bristle_half;
            let b = current + perp * bristle_half;

            for (transform, mut vel) in &mut rats {
                let p = Vec3::new(transform.translation.x, 0.0, transform.translation.z);

                // 2d capsule sdf
                let ab = b - a;
                let ap = p - a;
                let t = ap.dot(ab) / ab.dot(ab);
                let t = t.clamp(0.0, 1.0);
                let closest = a + ab * t;
                let dist = (p - closest).length();

                if dist < thickness {
                    vel.0 += sweep_dir * BROOM_STRENGTH * sweep_len.min(3.0);
                }
            }
        }
    }

    broom.last_world_pos = Some(current);
}

// i dont know how to use blender that well
fn strip_skinning_data(
    rat_mesh: Option<Res<RatMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let Some(rat_mesh) = rat_mesh else { return };
    let Some(mut mesh) = meshes.get_mut(rat_mesh.0.id()) else {
        return;
    };

    mesh.remove_attribute(Mesh::ATTRIBUTE_JOINT_INDEX);
    mesh.remove_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT);
    *done = true;
}
