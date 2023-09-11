use std::collections::HashMap;

use bevy::prelude::*;
use terrain::{generate_voxel_data, Chunk, generate_mesh};

mod noise;
mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut chunk_map = HashMap::new();

    let chunk_pos = IVec3::ZERO;
    let voxels = generate_voxel_data(chunk_pos);
    let chunk = Chunk { voxels };

    chunk_map.insert(chunk_pos, chunk);

    let (vertices, indices) = generate_mesh(&chunk_map);

    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::RED.into()),
        ..Default::default()
    });
    
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.5, 150.0, 28.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(40., 82.5, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
