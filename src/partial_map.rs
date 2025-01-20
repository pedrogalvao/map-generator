use std::sync::Arc;

use image::{GenericImageView, ImageBuffer, Rgba};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

#[derive(Clone, Serialize, Deserialize)]
pub struct PartialMap<S: MapShape, T: Clone> {
    pub circunference: usize,
    pub height: usize,
    meters_per_pixel: i32,
    shape: S,
    pub values: Vec<Vec<T>>,
}

impl<S: MapShape, T: Clone + Default> PartialMap<S, T> {
    pub fn new(circunference: usize, height: usize) -> Self {
        let shape = S::new();
        Self {
            circunference,
            height,
            meters_per_pixel: 100,
            shape: shape,
            values: S::new_vec(circunference, height),
        }
    }
}

impl<S: MapShape, T: Clone> PartialMap<S, T> {
    pub fn get(&self, latitude: f32, longitude: f32) -> T {
        let n_rows = self.values.len() as f32;
        let x = ((n_rows * (latitude + 90.0) / 180.0) as usize).min(self.values.len() - 1);
        let n_cols = self.values[x].len() as f32;
        let y = (n_cols * (longitude + 180.0) / 360.0)
            .rem_euclid(n_cols)
            .min(n_cols - 1.0) as usize;
        return self.values[x][y].clone();
    }

    pub fn convert_coords(&self, x: usize, y: usize) -> [f32; 2] {
        let latitude = x as f32 * 180.0 / self.values.len() as f32 - 90.0;
        let longitude = y as f32 * 360.0 / self.values[x].len() as f32 - 180.0;
        return [latitude, longitude];
    }

    pub fn convert_to_vec_coords(&self, latitude: f32, longitude: f32) -> [usize; 2] {
        let n_rows = self.values.len() as f32;
        let x = ((n_rows * (latitude + 90.0) / 180.0) as usize).min(self.values.len() - 1);
        let n_cols = self.values[x].len() as f32;
        let y = (n_cols * (longitude + 180.0) / 360.0)
            .rem_euclid(n_cols)
            .min(n_cols - 1.0) as usize;
        return [x, y];
    }

    pub fn convert_to_spatial_coords(&self, x: usize, y: usize) -> [f32; 3] {
        let [latitude, longitude] = self.convert_coords(x, y);
        self.shape.convert_to_spatial_coords(latitude, longitude)
    }

    pub fn get_distance(&self, p1: &[f32; 2], p2: &[f32; 2]) -> f32 {
        return self.shape.get_distance(p1, p2);
    }

    pub fn get_random_points_from_seed(&self, n_points: usize, seed: u32) -> Vec<[usize; 2]> {
        return self.shape.get_random_points_from_seed(self, n_points, seed);
    }

    pub fn get_pixel_neighbours(&self, p1: [usize; 2], pixel_distance: usize) -> Vec<Vec<T>> {
        return self.shape.get_pixel_neighbours(p1, self, pixel_distance);
    }

    pub fn get_pixel_neighbours_coords(
        &self,
        p1: [usize; 2],
        pixel_distance: usize,
    ) -> Vec<Vec<[usize; 2]>> {
        return self
            .shape
            .get_pixel_neighbours_coords(p1, self, pixel_distance);
    }
}

impl<S: MapShape, T: Clone + Send + Sync> PartialMap<S, T> {
    pub fn iterate_operator<'a, F>(&mut self, input_map: &'a CompleteMap<S>, operator: F)
    where
        F: Fn(usize, usize, Arc<&'a CompleteMap<S>>) -> T + Sync + Send,
        S: 'a,
    {
        self.values
            .par_iter_mut()
            .enumerate()
            .for_each(|(x, inner_vec)| {
                inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
                    *num = operator(x, y, Arc::new(input_map));
                });
            });
    }
}

impl<S: MapShape> PartialMap<S, i32> {
    pub fn _get_interpolation(&self, latitude: f32, longitude: f32) -> i32 {
        let n_rows = self.values.len() as f32;
        let x = n_rows * (latitude + 90.0) / 180.0;
        let x1 = x.floor() as usize;
        let x2 = x.ceil().min(n_rows - 1.0) as usize;
        let n_cols = self.values[x as usize].len();
        let n_cols1 = self.values[x1].len();
        let n_cols2 = self.values[x2].len();
        let y_f: f32 = n_cols as f32 * (longitude + 180.0) / 360.0;
        let y1 = ((n_cols1 as f32 * (longitude + 180.0) / 360.0) as usize).rem_euclid(n_cols1);
        let y2 = ((n_cols2 as f32 * (longitude + 180.0) / 360.0) as usize).rem_euclid(n_cols2);

        let weight1 = x - x1 as f32;
        let weight2 = x2 as f32 - x;
        let weight3 = y_f - y_f.floor();
        let weight4 = y_f.ceil().rem_euclid(n_cols as f32) - y_f;

        let total = weight1 * self.values[x1][y1] as f32
            + weight2 * self.values[x2][y2] as f32
            + weight3 * self.values[x as usize][(y_f.floor() as usize).rem_euclid(n_cols)] as f32
            + weight4
                * self.values[x as usize][y_f.ceil().rem_euclid(n_cols as f32) as usize] as f32;

        let value = total / (weight1 + weight2 + weight3 + weight4);

        return value as i32;
    }

    pub fn save_as_img(&self, filename: &str, min_value: i32, max_value: i32) {
        let mut imgbuffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(self.circunference as u32, self.height as u32);
        for img_x in 0..self.circunference {
            for img_y in 0..self.height {
                let y = (img_x * self.values[img_y].len() / self.circunference)
                    .min(self.values[img_y].len() - 1);
                if self.values[img_y][y] > 0 {
                    let value = ((255 * self.values[img_y][y] / max_value) as u8).max(1);
                    imgbuffer.put_pixel(img_x as u32, img_y as u32, Rgba([0, value, 0, 255]));
                } else {
                    let value = ((-255 * self.values[img_y][y] / min_value) as u8).max(1);
                    imgbuffer.put_pixel(img_x as u32, img_y as u32, Rgba([0, 0, value, 255]));
                }
            }
        }
        imgbuffer
            .save(filename)
            .expect("Could not save partial map image!");
    }
}

pub fn load_from_img<S: MapShape>(
    filename: &str,
    min_value: i32,
    max_value: i32,
) -> PartialMap<S, i32> {
    let imgbuffer = image::open(filename).expect("File not found!");

    let mut pmap =
        PartialMap::<S, i32>::new(imgbuffer.width() as usize, imgbuffer.height() as usize);

    for img_y in 0..imgbuffer.height() {
        let mut line = vec![];
        for img_x in 0..imgbuffer.width() {
            let color = imgbuffer.get_pixel(img_x, img_y);
            let value;
            if color[1] > color[2] {
                // land
                value = max_value * color[1] as i32 / 254;
            } else {
                // water
                value = min_value * (255 - color[2] as i32) / 255;
            }
            line.push(value);
        }
        pmap.values[img_y as usize] = line;
    }
    return pmap;
}
