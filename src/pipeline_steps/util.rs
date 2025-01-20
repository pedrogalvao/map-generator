use std::f32::consts::PI;

use noise::{NoiseFn, Perlin};

use crate::{partial_map::PartialMap, shapes::map_shape::MapShape};

pub fn percentile_2d_vector(vector: &Vec<Vec<i32>>, percentile: f32) -> Option<i32> {
    // Flatten the 2D vector into a 1D vector
    let mut flattened_vector: Vec<i32> = vector.iter().flatten().cloned().collect();

    // Sort the flattened vector
    flattened_vector.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Check if the percentile is within the valid range (0.0 to 100.0)
    if percentile < 0.0 || percentile > 100.0 {
        return None;
    }

    // Calculate the index for the desired percentile
    let mut index = (percentile / 100.0 * (flattened_vector.len() - 1) as f32).round() as usize;

    if index >= flattened_vector.len() {
        index = flattened_vector.len() - 1;
    }

    // Return the element at the calculated index
    Some(flattened_vector[index])
}

#[derive(Debug)]
pub struct CustomNoise {
    perlin: Perlin,
    frequency: f32,
    intensity: f32,
}

impl CustomNoise {
    pub fn new(seed: u32, frequency: f32, intensity: f32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            frequency,
            intensity,
        }
    }
}

impl CustomNoise {
    pub fn get_spheric<S: MapShape, T: Clone>(&self, latitude: f32, longitude: f32) -> i32 {
        return self.get_spheric_f32::<S, T>(latitude, longitude) as i32;
    }
    pub fn get<S: MapShape, T: Clone>(&self, x: usize, y: usize, pmap: &PartialMap<S, T>) -> i32 {
        return self.get_f32(x, y, pmap) as i32;
    }

    pub fn get_spheric_f32<S: MapShape, T: Clone>(&self, latitude: f32, longitude: f32) -> f32 {
        let radius = 0.5;
        let shape = S::new();
        let [x, y, z] = shape.convert_to_spatial_coords(latitude, longitude);
        let perlin_x = self.frequency as f32 * x * radius;
        let perlin_y = self.frequency as f32 * y * radius;
        let perlin_z = self.frequency as f32 * z * radius;
        let perlin_value =
            self.perlin
                .get([perlin_x as f64, perlin_y as f64, perlin_z as f64]) as f32
                * self.intensity;

        return perlin_value;
    }

    pub fn get_f32<S: MapShape, T: Clone>(
        &self,
        x: usize,
        y: usize,
        pmap: &PartialMap<S, T>,
    ) -> f32 {
        let radius = 1.0;
        let [x, y, z] = pmap.convert_to_spatial_coords(x, y);
        let perlin_x = self.frequency as f32 * x * radius;
        let perlin_y = self.frequency as f32 * y * radius;
        let perlin_z = self.frequency as f32 * z * radius;
        let perlin_value =
            self.perlin
                .get([perlin_x as f64, perlin_y as f64, perlin_z as f64]) as f32
                * self.intensity;

        return perlin_value;
    }
}

pub fn rotate_vector(v: [f32; 2], angle: f32) -> [f32; 2] {
    let angle_rad = angle * PI / 180.0;
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();
    let x_new = v[0] * cos_angle - v[1] * sin_angle;
    let y_new = v[0] * sin_angle + v[1] * cos_angle;
    [x_new, y_new]
}
