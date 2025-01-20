use std::{collections::HashSet, time::Instant};

use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{layer::MapViewLayer, projection::projection::Projection};

pub struct ContourLayer<S, T, F>
where
    S: MapShape,
    T: Clone,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, T>,
{
    thickness: u32,
    selector: F,
    color: Rgba<u8>,
    _marker: std::marker::PhantomData<S>, // PhantomData to indicate usage of S
}

impl<S, T, F> ContourLayer<S, T, F>
where
    S: MapShape,
    T: Clone,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, T>,
{
    pub fn new(selector: F, color: Rgba<u8>, thickness: u32) -> Self {
        Self {
            thickness,
            selector,
            color,
            _marker: std::marker::PhantomData,
        }
    }

    fn is_contour_pixel<P: Projection>(
        &self,
        img_x: u32,
        img_y: u32,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
    ) -> bool {
        let Some([lat, long]) = projection.img_to_map_coords(
            img_x,
            img_y,
            base_img.width() as u32,
            base_img.height() as u32,
            center_longitude,
        ) else {
            return false;
        };
        if complete_map.height.get(lat, long) > 0 {
            return false;
        }
        for k in -(self.thickness as i32)..=self.thickness as i32 {
            if img_x as i32 + k < 0 || img_x as i32 + k >= base_img.height() as i32 {
                continue;
            }
            for l in -(self.thickness as i32)..=self.thickness as i32 {
                if img_y as i32 + l < 0 || img_y as i32 + l >= base_img.width() as i32 {
                    continue;
                }
                if let Some([lat2, long2]) = projection.img_to_map_coords(
                    (img_x as i32 + k) as u32,
                    (img_y as i32 + l) as u32,
                    base_img.width() as u32,
                    base_img.height() as u32,
                    center_longitude,
                ) {
                    if complete_map.height.get(lat2, long2) > 0 {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn draw_quick<P: Projection>(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {
        let mut checked_pixels = HashSet::new();
        for [x, y] in complete_map.coastline.iter().flatten() {
            let [latitude, longitude] = complete_map.height.convert_coords(*x, *y);
            if let Some([img_x, img_y]) = projection.map_to_img_coords(
                latitude,
                longitude,
                base_img.width() as u32,
                base_img.height() as u32,
                center_longitude,
            ) {
                for k in -(2 as i32)..=2 as i32 {
                    if img_x as i32 + k < 0 || img_x as i32 + k >= base_img.height() as i32 {
                        continue;
                    }
                    for l in -(2 as i32)..=2 as i32 {
                        if img_y as i32 + l < 0 || img_y as i32 + l >= base_img.width() as i32 {
                            continue;
                        }
                        let img_x2 = (img_x as i32 + k) as u32;
                        let img_y2 = (img_y as i32 + l) as u32;
                        if checked_pixels.contains(&[img_x2, img_y2]) {
                            continue;
                        }
                        if self.is_contour_pixel(
                            img_x2,
                            img_y2,
                            center_longitude,
                            complete_map,
                            base_img,
                            projection,
                        ) {
                            base_img.put_pixel(img_y2, img_x2, self.color);
                        }
                        checked_pixels.insert([img_x2, img_y2]);
                    }
                }
            }
        }
    }
}

impl<S, P, F> MapViewLayer<P, S> for ContourLayer<S, i32, F>
where
    S: MapShape,
    P: Projection,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, i32>,
{
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {
        if self.thickness == 0 {
            return;
        };
        if complete_map.coastline != None {
            let i1 = Instant::now();
            self.draw_quick(base_img, projection, center_longitude, complete_map);
            let i2 = Instant::now();
            return;
        }
        let height_map = (self.selector)(complete_map);
        for i in 0..base_img.width() {
            for j in 0..base_img.height() {
                if let Some([lat, long]) = projection.img_to_map_coords(
                    j as u32,
                    i as u32,
                    base_img.width() as u32,
                    base_img.height() as u32,
                    center_longitude,
                ) {
                    let map_x = (height_map.height as f32 * (lat + 90.0).rem_euclid(180.0) / 180.0)
                        as usize;
                    let row_len = height_map.values[map_x].len();
                    let map_y =
                        (row_len as f32 * (long + 180.0).rem_euclid(360.0) / 360.0) as usize;
                    if height_map.values[map_x][map_y] <= 0 {
                        'outer_loop: for k in -(self.thickness as i32)..=self.thickness as i32 {
                            if i as i32 + k < 0 || i as i32 + k >= base_img.width() as i32 {
                                continue;
                            }
                            for l in -(self.thickness as i32)..=self.thickness as i32 {
                                if j as i32 + l < 0 || j as i32 + l >= base_img.height() as i32 {
                                    continue;
                                }
                                if let Some([lat2, long2]) = projection.img_to_map_coords(
                                    (j as i32 + l) as u32,
                                    (i as i32 + k) as u32,
                                    base_img.width() as u32,
                                    base_img.height() as u32,
                                    center_longitude,
                                ) {
                                    let map_x2 = (height_map.height as f32
                                        * (lat2 + 90.0).rem_euclid(180.0)
                                        / 180.0)
                                        as usize;
                                    let row_len = height_map.values[map_x2].len();
                                    let map_y2: usize = (row_len as f32
                                        * (long2 + 180.0).rem_euclid(360.0)
                                        / 360.0)
                                        as usize;

                                    if height_map.values[map_x2][map_y2] > 0 {
                                        base_img.put_pixel(i as u32, j as u32, self.color);
                                        break 'outer_loop;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<S, P, F> MapViewLayer<P, S> for ContourLayer<S, usize, F>
where
    S: MapShape,
    P: Projection,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, usize>,
{
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {
        if self.thickness == 0 {
            return;
        };
        let i1 = Instant::now();
        let height_map = (self.selector)(complete_map);
        for i in 0..base_img.width() {
            for j in 0..base_img.height() {
                if let Some([lat, long]) = projection.img_to_map_coords(
                    j as u32,
                    i as u32,
                    base_img.width() as u32,
                    base_img.height() as u32,
                    center_longitude,
                ) {
                    let map_x = (height_map.height as f32 * (lat + 90.0).rem_euclid(180.0) / 180.0)
                        as usize;
                    let row_len = height_map.values[map_x].len();
                    let map_y =
                        (row_len as f32 * (long + 180.0).rem_euclid(360.0) / 360.0) as usize;
                    for k in -(self.thickness as i32)..=self.thickness as i32 {
                        if i as i32 + k < 0 || i as i32 + k >= base_img.width() as i32 {
                            continue;
                        }
                        for l in -(self.thickness as i32)..=self.thickness as i32 {
                            if j as i32 + l < 0 || j as i32 + l >= base_img.height() as i32 {
                                continue;
                            }
                            if let Some([lat2, long2]) = projection.img_to_map_coords(
                                (j as i32 + l) as u32,
                                (i as i32 + k) as u32,
                                base_img.width() as u32,
                                base_img.height() as u32,
                                center_longitude,
                            ) {
                                let map_x2 = (height_map.height as f32
                                    * (lat2 + 90.0).rem_euclid(180.0)
                                    / 180.0) as usize;
                                let row_len = height_map.values[map_x2].len();
                                let map_y2: usize =
                                    (row_len as f32 * (long2 + 180.0).rem_euclid(360.0) / 360.0)
                                        as usize;

                                if height_map.values[map_x][map_y]
                                    < height_map.values[map_x2][map_y2]
                                {
                                    base_img.put_pixel(i as u32, j as u32, self.color);
                                }
                            }
                        }
                    }
                }
            }
        }
        let i2 = Instant::now();
    }
}
