use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::partial_map::PartialMap;

use crate::shapes::map_shape::MapShape;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Flat;

impl MapShape for Flat {
    fn new() -> Self {
        Self
    }

    fn get_distance(
        &self,
        [latitude1, longitude1]: &[f32; 2],
        [latitude2, longitude2]: &[f32; 2],
    ) -> f32 {
        let dx = (latitude1 - latitude2).abs();
        let dy = (longitude1 - longitude2).abs();

        let distance_squared = dx * dx + dy * dy;

        (distance_squared as f32).sqrt()
    }

    fn get_pixel_neighbours<S: MapShape, T: Clone>(
        &self,
        p1: [usize; 2],
        pm: &PartialMap<S, T>,
        pixel_distance: usize,
    ) -> Vec<Vec<T>> {
        let mut neighbors = vec![];
        for x in (p1[0] as i32 - pixel_distance as i32)..=(p1[0] as i32 + pixel_distance as i32) {
            neighbors.push(vec![]);
            if x >= pm.values.len() as i32 || x < 0 {
                continue;
            }
            for y in (p1[1] as i32 - pixel_distance as i32)..=(p1[1] as i32 + pixel_distance as i32)
            {
                if y >= pm.values[x as usize].len() as i32 || y < 0 {
                    continue;
                }
                let n_len: usize = neighbors.len();
                neighbors[n_len - 1].push(pm.values[x as usize][y as usize].clone());
            }
        }
        return neighbors;
    }

    fn get_pixel_neighbours_coords<S: MapShape, T: Clone>(
        &self,
        p1: [usize; 2],
        pm: &PartialMap<S, T>,
        pixel_distance: usize,
    ) -> Vec<Vec<[usize; 2]>> {
        let mut neighbors = vec![];
        for x in (p1[0] as i32 - pixel_distance as i32)..=(p1[0] as i32 + pixel_distance as i32) {
            neighbors.push(vec![]);
            if x >= pm.values.len() as i32 || x < 0 {
                continue;
            }
            for y in (p1[1] as i32 - pixel_distance as i32)..=(p1[1] as i32 + pixel_distance as i32)
            {
                if y >= pm.values[x as usize].len() as i32 || y < 0 {
                    continue;
                }
                let n_len: usize = neighbors.len();
                neighbors[n_len - 1].push([x as usize, y as usize]);
            }
        }
        return neighbors;
    }

    fn get_random_point<S: MapShape, T: Clone>(pm: PartialMap<S, T>) -> [usize; 2] {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..pm.values.len());
        let y = rng.gen_range(0..pm.values[x].len());
        return [x, y];
    }

    fn get_random_points<S: MapShape, T: Clone>(
        &self,
        pm: &PartialMap<S, T>,
        n_points: usize,
    ) -> Vec<[usize; 2]> {
        let mut rng = rand::thread_rng();
        let mut points = Vec::with_capacity(n_points);
        for _ in 0..n_points {
            let x = rng.gen_range(0..pm.values.len());
            let y = rng.gen_range(0..pm.values[x].len());
            points.push([x, y]);
        }
        return points;
    }

    fn convert_to_spatial_coords(&self, latitude: f32, longitude: f32) -> [f32; 3] {
        let x = longitude / 90.0;
        let y = 0.0;
        let z = latitude / 90.0;
        [x, y, z]
    }
}
