use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_rats)
        .add_systems(Update, strip_skinning_data);
}

#[derive(Component, Default, Debug, Reflect, Clone)]
pub struct Rat;

#[derive(Resource)]
struct RatMesh(Handle<Mesh>);

fn spawn_rats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const MAP_HALF_SIZE: f32 = 25.0;
    const SPACING: f32 = 0.5;
    const EXCLUSION_RADIUS: f32 = 5.0;

    let mesh: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive { mesh: 0, primitive: 0 }.from_asset("rat.glb"),
    );
    let material = materials.add(StandardMaterial::default());

    commands.insert_resource(RatMesh(mesh.clone()));

    let mut x = -MAP_HALF_SIZE;
    while x <= MAP_HALF_SIZE {
        let mut z = -MAP_HALF_SIZE;
        while z <= MAP_HALF_SIZE {
            let pos = Vec2::new(x, z);
            if pos.length() > EXCLUSION_RADIUS {
                commands.spawn((
                    Rat,
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(Vec3::new(x, 0.0, z)),
                ));
            }
            z += SPACING;
        }
        x += SPACING;
    }
}

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
