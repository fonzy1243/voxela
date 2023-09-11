use bevy::prelude::*;
use std::collections::HashMap;

pub const CHUNK_SIZE: usize = 32 * 32 * 32;

pub struct Voxel {
    is_solid: bool,
}

pub struct Chunk {
    pub voxels: Vec<Voxel>,
}

type ChunkMap = HashMap<IVec3, Chunk>;

pub fn generate_voxel_data(chunk_pos: IVec3) -> Vec<Voxel> {
    let mut voxels = Vec::with_capacity(CHUNK_SIZE);

    for z in 0..32 {
        for y in 0..32 {
            for x in 0..32 {
                let pos = IVec3::new(
                    x + chunk_pos.x * 32,
                    y + chunk_pos.y * 32,
                    z * chunk_pos.z * 32,
                );
                let is_solid = true;
                voxels.push(Voxel { is_solid });
            }
        }
    }

    voxels
}

fn generate_cube_vertices(pos: Vec3) -> Vec<[f32; 3]> {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    vec![
        // 0
        [x + 0., y + 1., z + 1.],
        // 1
        [x + 1., y + 1., z + 1.],
        // 2
        [x + 1., y + 1., z + 0.],
        // 3
        [x + 0., y + 1., z + 0.],
        // 4
        [x + 0., y + 0., z + 0.],
        // 5
        [x + 1., y + 0., z + 0.],
        // 6
        [x + 1., y + 0., z + 1.],
        // 7
        [x + 0., y + 0., z + 1.],
    ]
}

fn generate_cube_indices(start: u32) -> Vec<u32> {
    vec![
        0, 1, 2, 2, 3, 0, // Top
        7, 5, 6, 7, 5, 4, // Bottom
        7, 0, 4, 4, 0, 3, // Left
        6, 5, 1, 1, 5, 2, // Right
        7, 1, 0, 7, 6, 1, // Front
        5, 4, 3, 4, 3, 2, // Back
    ]
    .iter()
    .map(|index| index + start)
    .collect()
}

pub fn generate_mesh(chunk_map: &ChunkMap) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for (chunk_pos, chunk) in chunk_map.iter() {
        let chunk_offset = Vec3::new(
            chunk_pos.x as f32 * 32.,
            chunk_pos.y as f32 * 32.,
            chunk_pos.z as f32 * 32.,
        );

        for z in 0..32 {
            for y in 0..32 {
                for x in 0..32 {
                    // 32 * 32, 32, 1
                    let index = z * 1024 + y * 32 + x;

                    if !chunk.voxels[index].is_solid {
                        continue;
                    }

                    let pos = Vec3::new(
                        x as f32 + chunk_offset.x,
                        y as f32 + chunk_offset.y,
                        z as f32 + chunk_offset.z,
                    );

                    let cube_vertices = generate_cube_vertices(pos);
                    let cube_indices = generate_cube_indices(vertices.len() as u32);

                    vertices.extend(cube_vertices);
                    indices.extend(cube_indices);
                }
            }
        }
    }

    (vertices, indices)
}
