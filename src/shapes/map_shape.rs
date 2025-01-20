use serde::Serialize;

use crate::partial_map::PartialMap;

use super::util::pseudo_random_usize;

pub trait MapShape: Sync + Send + Clone + Serialize {
    fn new() -> Self;
    fn get_distance(&self, p1: &[f32; 2], p2: &[f32; 2]) -> f32;
    fn get_pixel_neighbours<S: MapShape, T: Clone>(
        &self,
        p1: [usize; 2],
        pm: &PartialMap<S, T>,
        pixel_distance: usize,
    ) -> Vec<Vec<T>>;
    fn get_pixel_neighbours_coords<S: MapShape, T: Clone>(
        &self,
        p1: [usize; 2],
        pm: &PartialMap<S, T>,
        pixel_distance: usize,
    ) -> Vec<Vec<[usize; 2]>>;
    fn get_random_point<S: MapShape, T: Clone>(pm: PartialMap<S, T>) -> [usize; 2];
    fn get_random_points<S: MapShape, T: Clone>(
        &self,
        pm: &PartialMap<S, T>,
        n_points: usize,
    ) -> Vec<[usize; 2]>;
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
        return vec![vec![T::default(); circunference]; height];
    }
    fn convert_to_spatial_coords(&self, latitude: f32, longitude: f32) -> [f32; 3];

    fn convert_to_vec_coords<S: MapShape, T: Clone>(
        &self,
        latitude: f32,
        longitude: f32,
        pmap: &PartialMap<S, T>,
    ) -> [usize; 2] {
        let x = ((latitude.rem_euclid(180.0) * pmap.height as f32 / 180.0) as usize)
            .max(0)
            .min(pmap.height - 1);
        let y = ((longitude.rem_euclid(360.0) * pmap.circunference as f32 / 360.0) as usize)
            .max(0)
            .min(pmap.circunference - 1);
        return [x, y];
    }
}
