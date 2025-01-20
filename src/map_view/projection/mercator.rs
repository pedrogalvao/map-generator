use std::f32::consts::PI;

use super::projection::Projection;

pub struct Mercator {}

impl Projection for Mercator {
    fn new() -> Self {
        Self {}
    }

    fn img_to_map_coords(
        &self,
        x: u32,
        y: u32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[f32; 2]> {
        // take image pixel coordinates and return latitude and longitude
        let lon = y as f32 / img_width as f32 * 360.0 - 180.0;

        let merc_y = 1.0 - (x as f32 / img_height as f32 * 2.0);
        let lat_rad = -((merc_y * PI).exp().atan() * 2.0 - PI / 2.0);
        let lat = lat_rad.to_degrees();
        let longitude = lon + center_longitude;
        Some([lat, longitude])
    }

    fn map_to_img_coords(
        &self,
        latitude: f32,
        longitude: f32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[u32; 2]> {
        let longitude = (longitude - center_longitude - 180.0).rem_euclid(360.0) - 180.0;
        let y = (longitude + 180.0) / 360.0 * img_width as f32;

        let lat_rad = latitude.to_radians();
        let merc_n = (PI / 4.0 - lat_rad / 2.0).tan().ln();
        let x = (1.0 - merc_n / PI) / 2.0 * img_height as f32;

        if x as u32 >= img_height || y as u32 >= img_width {
            return None;
        }

        Some([x as u32, y as u32])
    }

    fn get_latitude(&self, img_x: u32, _img_y: u32, _img_width: u32, _img_height: u32) -> f32 {
        let latitude = 90.0 / (1.0 + (2.718 as f32).powf(-(0.01 * img_x as f32)));
        return latitude;
    }

    fn get_img_y(
        &self,
        _latitude: f32,
        longitude: f32,
        img_width: u32,
        _img_height: u32,
        center_longitude: f32,
    ) -> u32 {
        let j = (img_width as f64
            * (-center_longitude + longitude + 180.0).rem_euclid(360.0) as f64
            / 360.0) as u32;
        return j;
    }
}
