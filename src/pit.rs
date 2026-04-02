//! RAT PIT RAT PIT RAT PIT
//!

use bevy::{prelude::*, scene2::bsn};

use crate::rat::BOUNDING_RANGE;
use crate::ui::Upgrades;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (init_pit, spawn_ground, spawn_walls).chain())
        .add_systems(Update, rebuild_ground);
}

fn init_pit(mut commands: Commands, upgrades: Res<Upgrades>) {
    commands.insert_resource(RatPit {
        half_size: upgrades.pit_size,
    });
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
