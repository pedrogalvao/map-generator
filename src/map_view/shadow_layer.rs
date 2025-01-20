use std::{collections::HashSet, time::Instant};

use image::{ImageBuffer, Rgba};
use num::integer::sqrt;

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{layer::MapViewLayer, projection::projection::Projection, util::color_over};

pub struct ShadowLayer<S, T, F>
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

impl<S, T, F> ShadowLayer<S, T, F>
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

    pub fn default(selector: F) -> Self {
        Self {
            thickness: 20,
            selector,
            color: Rgba([50, 50, 180, 255]),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S, F> ShadowLayer<S, i32, F>
where
    S: MapShape,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, i32>,
{
    fn pixel_dist_from_shore<P: Projection>(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
        img_x: u32,
        img_y: u32,
    ) -> u32 {
        let height_map = (self.selector)(complete_map);

        let Some([latitude, longitude]) = projection.img_to_map_coords(
            img_y,
            img_x,
            base_img.width() as u32,
            base_img.height() as u32,
            center_longitude,
        ) else {
            return 9999;
        };
        if height_map.get(latitude, longitude) > 0 {
            return 9999;
        }
        for dist in 1..self.thickness {
            for k in -(dist as i32)..=dist as i32 {
                if img_x as i32 + k < 0 || img_x as i32 + k >= base_img.width() as i32 {
                    continue;
                }
                for l in -(dist as i32)..=dist as i32 {
                    if img_y as i32 + l < 0 || img_y as i32 + l >= base_img.height() as i32 {
                        continue;
                    }
                    if let Some([lat2, long2]) = projection.img_to_map_coords(
                        (img_y as i32 + l) as u32,
                        (img_x as i32 + k) as u32,
                        base_img.width() as u32,
                        base_img.height() as u32,
                        center_longitude,
                    ) {
                        let map_x2 = (height_map.height as f32 * (lat2 + 90.0).rem_euclid(180.0)
                            / 180.0) as usize;
                        let row_len = height_map.values[map_x2].len();
                        let map_y2: usize =
                            (row_len as f32 * (long2 + 180.0).rem_euclid(360.0) / 360.0) as usize;

                        if height_map.values[map_x2][map_y2] > 0 {
                            return sqrt((k.pow(2) + l.pow(2)) as u32);
                        }
                    }
                }
            }
        }
        return 100;
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
                for k in -(self.thickness as i32)..=self.thickness as i32 {
                    if img_x as i32 + k < 0 || img_x as i32 + k >= base_img.height() as i32 {
                        continue;
                    }
                    for l in -(self.thickness as i32)..=self.thickness as i32 {
                        if img_y as i32 + l < 0 || img_y as i32 + l >= base_img.width() as i32 {
                            continue;
                        }
                        let img_x2 = (img_x as i32 + k) as u32;
                        let img_y2 = (img_y as i32 + l) as u32;
                        if checked_pixels.contains(&[img_x2, img_y2]) {
                            continue;
                        }
                        let dist_to_shore = self.pixel_dist_from_shore(
                            base_img,
                            projection,
                            center_longitude,
                            complete_map,
                            img_y2,
                            img_x2,
                        );
                        if dist_to_shore <= self.thickness {
                            // base_img.put_pixel(img_y2, img_x2, self.color);
                            let original_color = base_img.get_pixel(img_y2 as u32, img_x2 as u32);
                            let mut layer_color = self.color.clone();
                            layer_color[3] = (220.0
                                * (1.0 - dist_to_shore as f32 / self.thickness as f32))
                                as u8;
                            base_img.put_pixel(
                                img_y2 as u32,
                                img_x2 as u32,
                                color_over(original_color, &layer_color),
                            );
                        }
                        checked_pixels.insert([img_x2, img_y2]);
                    }
                }
            }
        }
    }
}

impl<S, P, F> MapViewLayer<P, S> for ShadowLayer<S, i32, F>
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
            self.draw_quick(base_img, projection, center_longitude, complete_map);
            return;
        }
        let height_map = (self.selector)(complete_map);
        for img_x in 0..base_img.width() {
            for img_y in 0..base_img.height() {
                if let Some([lat, long]) = projection.img_to_map_coords(
                    img_y as u32,
                    img_x as u32,
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
                        let dist = self.pixel_dist_from_shore(
                            base_img,
                            projection,
                            center_longitude,
                            complete_map,
                            img_x,
                            img_y,
                        );
                        if dist > self.thickness {
                            continue;
                        }
                        let original_color = base_img.get_pixel(img_x as u32, img_y as u32);
                        let mut layer_color = self.color.clone();
                        layer_color[3] =
                            (180.0 * (1.0 - dist as f32 / self.thickness as f32)) as u8;
                        base_img.put_pixel(
                            img_x as u32,
                            img_y as u32,
                            color_over(original_color, &layer_color),
                        );
                    }
                }
            }
        }
        let i2 = Instant::now();
    }
}

impl<S, P, F> MapViewLayer<P, S> for ShadowLayer<S, usize, F>
where
    S: MapShape,
    P: Projection,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, usize>,
{
    fn draw_layer(
        &self,
        _base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        _projection: &P,
        _center_longitude: f32,
        _complete_map: &CompleteMap<S>,
    ) {
        todo!()
    }
}
