use noise::*;

// noise to determine height
pub fn generate_noise_map(map_width: usize, map_height: usize, mut scale: f32) -> Vec<Vec<f32>> {
    let simplex = SuperSimplex::new(1);
    let mut noise_map = vec![vec![0f32; map_width]; map_height];

    if scale <= 0. {
        scale = 0.0001f32;
    }

    for y in 0..map_height {
        for x in 0..map_width {
            let sample_x = x as f32 / scale;
            let sample_y = y as f32 / scale;

            let simplex_value = simplex.get([sample_x as f64, sample_y as f64]) as f32;
            noise_map[x][y] = simplex_value;
        }
    }

    noise_map
}

pub struct MapGenerator {
    map_width: usize,
    map_height: usize,
    noise_scale: f32,
}

impl MapGenerator {
    pub fn new(map_width: usize, map_height: usize, noise_scale: f32) -> Self {
        Self {
            map_width,
            map_height,
            noise_scale,
        }
    }
}
