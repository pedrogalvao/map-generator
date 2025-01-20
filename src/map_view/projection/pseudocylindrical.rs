use super::projection::Projection;

pub struct PseudoCylindrical {
    factor: f32,
}

impl Projection for PseudoCylindrical {
    fn new() -> Self {
        Self { factor: 0.5 }
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
        let longitude;

        let half_height = (img_height / 2) as f32;

        let rlen = img_width as f32
            * (half_height.powf(2.0) as f32 - (x as f32 - half_height).powf(2.0)).powf(0.5)
            / half_height;

        let left_empty_pixels = (self.factor * (img_width - rlen as u32) as f32 / 2.0) as u32;
        if y <= left_empty_pixels {
            return None;
        } else if y >= img_width - left_empty_pixels {
            return None;
        } else {
            longitude = (y as f32 - left_empty_pixels as f32) * 360.0
                / (img_width - 2 * left_empty_pixels) as f32
                - 180.0
                + center_longitude;
        }
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

        let half_height = (img_height / 2) as f32;

        let mollweide_row_len = img_width as f32
            * (half_height.powf(2.0) as f32 - (img_x as f32 - half_height).powf(2.0)).powf(0.5)
            / half_height;
        let row_len = self.factor * mollweide_row_len + (1.0 - self.factor) * img_width as f32;

        let img_y = ((img_width as f64 - row_len as f64) / 2.0
            + (row_len as f64 * ((-center_longitude + longitude - 180.0).rem_euclid(360.0)) as f64
                / 360.0)) as u32;

        if img_x == 0 {
            return None;
        }

        return Some([img_x, img_y]);
    }

    fn get_latitude(&self, img_x: u32, _img_y: u32, _img_width: u32, img_height: u32) -> f32 {
        let latitude = (180.0 * img_x as f32 / img_height as f32).rem_euclid(180.0) - 90.0;
        return latitude;
    }

    fn get_img_y(
        &self,
        _latitude: f32,
        _longitude: f32,
        _img_width: u32,
        _img_height: u32,
        _center_longitude: f32,
    ) -> u32 {
        todo!()
    }
}
