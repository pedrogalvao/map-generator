use num::integer::sqrt;

use crate::map_view::util::{reverse_spin_coords, spin_coords};

use super::projection::Projection;

pub struct Oblique<P: Projection> {
    pub latitude: f32,
    pub longitude: f32,
    projection: P,
}

impl<P: Projection> Projection for Oblique<P> {
    fn new() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            projection: P::new(),
        }
    }

    fn img_to_map_coords(
        &self,
        x: u32,
        y: u32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[f32; 2]> {
        if let Some([latitude, longitude]) =
            self.projection
                .img_to_map_coords(x, y, img_width, img_height, center_longitude)
        {
            return Some(spin_coords(
                latitude,
                longitude,
                self.longitude,
                self.latitude,
            ));
        }
        None
    }

    fn map_to_img_coords(
        &self,
        latitude: f32,
        longitude: f32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[u32; 2]> {
        let [latitude, longitude] =
            reverse_spin_coords(latitude, longitude.clone(), self.longitude, self.latitude);
        self.projection.map_to_img_coords(
            latitude,
            longitude,
            img_width,
            img_height,
            center_longitude,
        )
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
