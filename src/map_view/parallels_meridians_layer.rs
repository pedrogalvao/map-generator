use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{layer::MapViewLayer, projection::projection::Projection, util::color_over};

pub struct ParallelsMeridiansLayer {
    pub color: Rgba<u8>,
    parallels: Vec<f32>,
    meridians: Vec<f32>,
}

impl ParallelsMeridiansLayer {
    pub fn new(interval: f32) -> Self {
        let mut meridians = vec![];
        let mut longitude = 0.0;
        meridians.push(0.0);
        while longitude < 180.0 {
            longitude += interval;
            meridians.push(longitude);
            meridians.push(-longitude);
        }
        let mut latitude = 0.0;
        let mut parallels = vec![];
        parallels.push(0.0);
        while latitude < 90.0 {
            latitude += interval;
            parallels.push(latitude);
            parallels.push(-latitude);
        }
        Self {
            parallels,
            meridians,
            color: Rgba([255, 255, 255, 100]),
        }
    }

    pub fn default() -> Self {
        Self {
            parallels: vec![0.0, 30.0, 60.0, -30.0, -60.0],
            meridians: vec![
                0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, -30.0, -60.0, -90.0, -120.0, -150.0,
            ],
            color: Rgba([255, 255, 255, 100]),
        }
    }

    pub fn tropics() -> Self {
        Self {
            parallels: vec![23.44, -23.44],
            meridians: vec![],
            color: Rgba([255, 100, 100, 200]),
        }
    }

    pub fn polar_circle() -> Self {
        Self {
            parallels: vec![66.5, -66.5],
            meridians: vec![],
            color: Rgba([0, 200, 255, 200]),
        }
    }

    pub fn set_parallels_interval(&mut self, interval: f32) {
        let mut v = vec![];
        let mut latitude = 0.0;
        while latitude < 90.0 {
            v.push(latitude);
            latitude += interval;
        }
        latitude = 0.0;
        while latitude > -90.0 {
            latitude -= interval;
            v.push(latitude);
        }
        self.parallels = v;
    }

    pub fn set_meridians_interval(&mut self, interval: f32) {
        let mut v = vec![];
        let mut longitude = 0.0;
        while longitude < 180.0 {
            v.push(longitude);
            longitude += interval;
        }
        longitude = 0.0;
        while longitude > -180.0 {
            longitude -= interval;
            v.push(longitude);
        }
        self.meridians = v;
    }

    fn add_meridians<P: Projection>(
        &self,
        imgbuffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
    ) {
        for longitude in &self.meridians {
            let mut prev_img_x = 0;
            let mut prev_img_y = 0;
            for i in 0..2 * imgbuffer.width() {
                let latitude = i as f32 * 90.0 / imgbuffer.width() as f32 - 90.0;
                if let Some([img_x, img_y]) = projection.map_to_img_coords(
                    latitude,
                    *longitude,
                    imgbuffer.width(),
                    imgbuffer.height(),
                    center_longitude,
                ) {
                    if img_x == prev_img_x && img_y == prev_img_y {
                        continue;
                    }
                    if img_x < imgbuffer.height() && img_y < imgbuffer.width() {
                        let color = imgbuffer.get_pixel_mut(img_y as u32, img_x as u32);
                        let new_color = color_over(color, &self.color);
                        imgbuffer.put_pixel(img_y as u32, img_x as u32, new_color);
                    }
                    prev_img_x = img_x;
                    prev_img_y = img_y;
                }
            }
        }
    }

    pub fn add_parallels<P: Projection>(
        &self,
        imgbuffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
    ) {
        for latitude in &self.parallels {
            let mut prev_img_x = 0;
            let mut prev_img_y = 0;
            for i in 0..2 * imgbuffer.width() {
                let longitude = i as f32 * 180.0 / imgbuffer.width() as f32 - 180.0;
                if let Some([img_x, img_y]) = projection.map_to_img_coords(
                    *latitude,
                    longitude,
                    imgbuffer.width(),
                    imgbuffer.height(),
                    center_longitude,
                ) {
                    if img_x == prev_img_x && img_y == prev_img_y {
                        continue;
                    }
                    if img_x < imgbuffer.height() && img_y < imgbuffer.width() {
                        let color = imgbuffer.get_pixel_mut(img_y as u32, img_x as u32);
                        let new_color = color_over(color, &self.color);
                        imgbuffer.put_pixel(img_y as u32, img_x as u32, new_color);
                    }
                    prev_img_x = img_x;
                    prev_img_y = img_y;
                }
            }
        }
    }
}
impl<P, S> MapViewLayer<P, S> for ParallelsMeridiansLayer
where
    S: MapShape,
    P: Projection,
{
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        _complete_map: &CompleteMap<S>,
    ) {
        self.add_meridians(base_img, projection, center_longitude);
        self.add_parallels(base_img, projection, center_longitude);
    }
}
