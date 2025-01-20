use std::f32::consts::PI;
use std::vec;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::partial_map::PartialMap;

use crate::shapes::map_shape::MapShape;
use crate::shapes::util::pseudo_random_usize;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Globe;

impl MapShape for Globe {
    fn new() -> Self {
        Self
    }

    fn get_distance(
        &self,
        [latitude1, longitude1]: &[f32; 2],
        [latitude2, longitude2]: &[f32; 2],
    ) -> f32 {
        let delta_latitude = (latitude2 - latitude1).to_radians();
        let delta_longitude = (longitude2 - longitude1).to_radians();

        let a = (delta_latitude / 2.0).sin().powi(2)
            + (latitude1.to_radians()).cos()
                * (latitude2.to_radians()).cos()
                * (delta_longitude / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        let distance = c;
        distance
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
            for dy in -(pixel_distance as i32)..=(pixel_distance as i32) {
                let y0 = p1[1] as i32;
                let mut y = (y0 as f32
                    * (pm.values[x as usize].len() as f32 / pm.values[p1[0]].len() as f32))
                    as i32;
                y += dy;
                y = y.rem_euclid(pm.values[x as usize].len() as i32);
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
            for dy in -(pixel_distance as i32)..=(pixel_distance as i32) {
                let y0 = p1[1] as i32;
                let mut y = (y0 as f32
                    * (pm.values[x as usize].len() as f32 / pm.values[p1[0]].len() as f32))
                    as i32;
                y += dy;
                y = y.rem_euclid(pm.values[x as usize].len() as i32);
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
        //todo!();
        return points;
    }

    fn get_random_points_from_seed<S: MapShape, T: Clone>(
        &self,
        pm: &PartialMap<S, T>,
        n_points: usize,
        seed: u32,
    ) -> Vec<[usize; 2]> {
        let mut points = Vec::with_capacity(n_points);
        for i in 0..n_points {
            let latitude = pseudo_random_usize(seed + i as u32) as f32 % 180.0 - 90.0;
            let longitude = pseudo_random_usize(seed + i as u32 + 1000) as f32 % 360.0 - 180.0;
            let [x, y] = self.convert_to_vec_coords(latitude, longitude, pm);
            points.push([x, y]);
        }
        return points;
    }

    fn new_vec<T: Default + Clone>(circunference: usize, height: usize) -> Vec<Vec<T>> {
        let mut v = vec![];
        for x in 0..height {
            let v_length = 1
                + (1.99
                    * (circunference as f32 / (height as f32))
                    * ((height as f32 / 2.0).powf(2.0)
                        - (x as f32 - height as f32 / 2.0).powf(2.0))
                    .sqrt())
                .ceil() as usize;
            v.push(vec![T::default(); v_length]);
        }
        return v;
    }

    fn convert_to_spatial_coords(&self, latitude: f32, longitude: f32) -> [f32; 3] {
        let x = (longitude * PI / 180.0).cos() * (latitude * PI / 180.0).cos();
        let y = (longitude * PI / 180.0).sin() * (latitude * PI / 180.0).cos();
        let z = (latitude * PI / 180.0).sin();
        [x, y, z]
    }
}
