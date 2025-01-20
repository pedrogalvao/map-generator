use super::projection::Projection;

pub struct Equirectangular {}

impl Projection for Equirectangular {
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
        let latitude = (180.0 * x as f32 / img_height as f32) - 90.0;
        let longitude =
            (360.0 * y as f32 / img_width as f32).rem_euclid(360.0) - 180.0 + center_longitude;
        return Some([latitude, longitude]);
    }

    fn map_to_img_coords(
        &self,
        latitude: f32,
        longitude: f32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[u32; 2]> {
        let img_x = (img_height as f32 * (latitude + 90.0) / 180.0) as u32;
        let img_y = (img_width as f64
            * (-center_longitude + longitude + 180.0).rem_euclid(360.0) as f64
            / 360.0) as u32;
        return Some([img_x, img_y]);
    }

    fn get_latitude(&self, img_x: u32, _img_y: u32, _img_width: u32, img_height: u32) -> f32 {
        let latitude = (180.0 * img_x as f32 / img_height as f32).rem_euclid(180.0) - 90.0;
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
