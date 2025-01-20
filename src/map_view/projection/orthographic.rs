use std::f32::consts::PI;

use num::integer::sqrt;

use super::projection::Projection;

pub struct Orthographic {}

fn img_to_map_coords_north(
    img_x: u32,
    img_y: u32,
    img_width: u32,
    img_height: u32,
    center_longitude: f32,
) -> Option<[f32; 2]> {
    let center_x = img_height / 2;
    let center_y = img_width / 2;

    let dist_to_center =
        sqrt((img_x as i32 - center_x as i32).pow(2) + (img_y as i32 - center_y as i32).pow(2));

    if dist_to_center as u32 > img_height / 2 {
        return None;
    }
    let degrees_from_pole =
        (2.0 * dist_to_center as f32 / img_height.min(img_width) as f32).asin() * 180.0 / PI;
    let latitude = degrees_from_pole - 90.0;

    let longitude;
    if img_x >= img_height / 2 {
        longitude = ((img_y as f32 - center_y as f32) / (img_x as f32 - center_x as f32)).atan()
            * 180.0
            / PI;
    } else {
        longitude = ((img_y as f32 - center_y as f32) / (img_x as f32 - center_x as f32)).atan()
            * 180.0
            / PI
            - 180.0;
    }
    if latitude >= 0.0 {
        return None;
    }
    let longitude = longitude - center_longitude;
    return Some([latitude, longitude]);
}

impl Projection for Orthographic {
    fn new() -> Self {
        Self {}
    }

    fn img_to_map_coords(
        &self,
        img_x: u32,
        img_y: u32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[f32; 2]> {
        // take image pixel coordinates and return latitude and longitude
        img_to_map_coords_north(img_x, img_y, img_width, img_height, center_longitude)
    }

    fn map_to_img_coords(
        &self,
        latitude: f32,
        longitude: f32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[u32; 2]> {
        let center_x = img_height as f32 / 2.0;
        let center_y = img_width as f32 / 2.0;
        let longitude = longitude + center_longitude;
        if latitude > 0.0 {
            return None;
        }
        let longitude_rad = (90.0 - longitude) * PI / 180.0;
        let latitude_rad = latitude * PI / 180.0;
        let img_x = center_x + img_height as f32 / 2.0 * longitude_rad.sin() * latitude_rad.cos();
        let img_y = center_y + img_height as f32 / 2.0 * longitude_rad.cos() * latitude_rad.cos();
        if img_x as u32 > img_height {
            return None;
        } else if img_y as u32 > img_width {
            return None;
        }

        return Some([img_x as u32, img_y as u32]);
    }

    fn get_latitude(&self, img_x: u32, img_y: u32, img_width: u32, img_height: u32) -> f32 {
        let center_x = img_height / 2;
        let center_y = img_width / 2;

        let dist_to_center =
            sqrt((img_x as i32 - center_x as i32).pow(2) + (img_y as i32 - center_y as i32).pow(2));

        let latitude = 360.0 * dist_to_center as f32 / img_height.min(img_width) as f32 - 90.0;

        return latitude;
    }

    fn get_img_y(
        &self,
        latitude: f32,
        longitude: f32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> u32 {
        return self
            .map_to_img_coords(latitude, longitude, img_width, img_height, center_longitude)
            .unwrap()[1];
    }
}
