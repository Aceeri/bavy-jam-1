//! RAT PIT RAT PIT RAT PIT
//!

use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::light::{FogVolume, VolumetricLight};

use crate::rat::BOUNDING_RANGE;
use crate::ui::Upgrades;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Startup,
        (init_pit, spawn_ground, spawn_walls, spawn_corner_lights).chain(),
    )
    .add_systems(Update, rebuild_ground);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, resize_pit_fog);
}

#[derive(Component)]
struct PitFog;

fn init_pit(mut commands: Commands, upgrades: Res<Upgrades>) {
    commands.insert_resource(RatPit {
        half_size: upgrades.pit_size,
    });

    #[cfg(not(target_arch = "wasm32"))]
    {
        let h = upgrades.pit_size;
        commands.spawn((
            PitFog,
            FogVolume {
                density_factor: 1.0,
                absorption: 0.8,
                scattering: 0.0,
                scattering_asymmetry: 0.0,
                fog_color: Color::BLACK,
                light_tint: Color::BLACK,
                light_intensity: 0.0,
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, -1.5, 0.0)).with_scale(Vec3::new(
                h * 2.0,
                3.0,
                h * 2.0,
            )),
        ));
    }
}

#[derive(Resource, Reflect)]
pub struct RatPit {
    pub half_size: f32,
}

#[derive(Component, Reflect, Copy, Clone, Debug, Default)]
pub struct GroundPanel;

const MAP_HALF: f32 = 25.0;
const GROUND_DEPTH: f32 = 1500.0;
const WALL_HEIGHT: f32 = 1.0;
const WALL_THICKNESS: f32 = 0.3;

fn spawn_ground(
    mut commands: Commands,
    pit: Res<RatPit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::srgb(0.3, 0.5, 0.3));
    spawn_panels(&mut commands, &pit, &mut meshes, material);
}

pub fn rebuild_ground(
    mut commands: Commands,
    pit: Res<RatPit>,
    panels: Query<Entity, With<GroundPanel>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !pit.is_changed() {
        return;
    }

    for entity in &panels {
        commands.entity(entity).despawn();
    }

    let material = materials.add(Color::srgb(0.3, 0.5, 0.3));
    spawn_panels(&mut commands, &pit, &mut meshes, material);
}

pub fn spawn_panels(
    commands: &mut Commands,
    pit: &RatPit,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
) {
    let h = pit.half_size.clamp(0.0, MAP_HALF);
    let edge = MAP_HALF - h;

    let y = -GROUND_DEPTH / 2.0;

    // not entirely sure bsn! makes this easier to do?
    // kind of more of a pain trying to figure out how to get mesh/material in

    // north
    let mesh = meshes.add(Cuboid::new(MAP_HALF * 2.0, GROUND_DEPTH, edge));
    commands.spawn((
        GroundPanel,
        Mesh3d(mesh),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::new(0.0, y, -(h + edge / 2.0))),
    ));

    // south
    let mesh = meshes.add(Cuboid::new(MAP_HALF * 2.0, GROUND_DEPTH, edge));
    commands.spawn((
        GroundPanel,
        Mesh3d(mesh),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::new(0.0, y, h + edge / 2.0)),
    ));

    // west
    let mesh = meshes.add(Cuboid::new(edge, GROUND_DEPTH, h * 2.0));
    commands.spawn((
        GroundPanel,
        Mesh3d(mesh),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::new(-(h + edge / 2.0), y, 0.0)),
    ));

    // east
    let mesh = meshes.add(Cuboid::new(edge, GROUND_DEPTH, h * 2.0));
    commands.spawn((
        GroundPanel,
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(h + edge / 2.0, y, 0.0)),
    ));
}

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_material = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let arena_len = BOUNDING_RANGE * 2.0 + WALL_THICKNESS * 3.0;

    // north
    let mesh = meshes.add(Cuboid::new(arena_len, WALL_HEIGHT, WALL_THICKNESS));
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(Vec3::new(
            0.0,
            WALL_HEIGHT / 2.0,
            -BOUNDING_RANGE - WALL_THICKNESS,
        )),
    ));

    // south
    let mesh = meshes.add(Cuboid::new(arena_len, WALL_HEIGHT, WALL_THICKNESS));
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(Vec3::new(
            0.0,
            WALL_HEIGHT / 2.0,
            BOUNDING_RANGE + WALL_THICKNESS,
        )),
    ));

    // west
    let mesh = meshes.add(Cuboid::new(WALL_THICKNESS, WALL_HEIGHT, arena_len));
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(Vec3::new(
            -BOUNDING_RANGE - WALL_THICKNESS,
            WALL_HEIGHT / 2.0,
            0.0,
        )),
    ));

    // east
    let mesh = meshes.add(Cuboid::new(WALL_THICKNESS, WALL_HEIGHT, arena_len));
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(wall_material),
        Transform::from_translation(Vec3::new(
            BOUNDING_RANGE + WALL_THICKNESS,
            WALL_HEIGHT / 2.0,
            0.0,
        )),
    ));
}

fn spawn_corner_lights(mut commands: Commands) {
    let b = BOUNDING_RANGE + 0.3;
    let height = 5.0;

    for corner in [
        Vec3::new(-b, height, -b),
        Vec3::new(b, height, -b),
        Vec3::new(-b, height, b),
        Vec3::new(b, height, b),
    ] {
        let dir = (Vec3::ZERO - corner).normalize();
        let light = commands.spawn((
            SpotLight {
                intensity: 500_000.0,
                range: b * 5.0,
                radius: 0.5,
                outer_angle: 0.9,
                inner_angle: 0.4,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_translation(corner).looking_to(dir, Vec3::Y),
        )).id();
        #[cfg(not(target_arch = "wasm32"))]
        commands.entity(light).insert(VolumetricLight);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn resize_pit_fog(pit: Res<RatPit>, mut fog: Query<&mut Transform, With<PitFog>>) {
    if !pit.is_changed() { return; }
    for mut transform in &mut fog {
        transform.scale = Vec3::new(pit.half_size * 2.0, 3.0, pit.half_size * 2.0);
    }
}
