use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{layer::MapViewLayer, projection::projection::Projection, util::color_over};

pub struct WindMapLayer {
    month: usize,
    color: Rgba<u8>,
}

impl WindMapLayer {
    pub fn new(month: usize, color: Rgba<u8>) -> Self {
        Self { month, color }
    }
}

impl<S: MapShape, P: Projection> MapViewLayer<P, S> for WindMapLayer {
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {
        let img_height = base_img.height();
        let img_width = base_img.width();
        for i in 0..40 {
            for j in 0..30 {
                let [latitude, longitude] = [
                    180.0 * i as f32 / 40.0 - 90.0,
                    360.0 * j as f32 / 30.0 - 180.0,
                ];
                let [mut latitude, mut longitude] = [latitude, longitude];
                for _ in 0..1000 {
                    if let Some([img_x, img_y]) = projection.map_to_img_coords(
                        latitude,
                        longitude,
                        img_width,
                        img_height,
                        center_longitude,
                    ) {
                        if img_x >= img_height || img_y >= img_width {
                            break;
                        }
                        let original_color = base_img.get_pixel(img_y as u32, img_x as u32);
                        let new_color = color_over(original_color, &self.color);
                        base_img.put_pixel(img_y as u32, img_x as u32, new_color);
                    } else {
                        break;
                    }
                    let direction = complete_map.winds[self.month]
                        .get(latitude, longitude)
                        .clone();
                    [latitude, longitude] = [
                        latitude + 0.2 * direction[0],
                        longitude + 0.2 * direction[1],
                    ];
                    let pressure = complete_map.atm_pressure[self.month].get(latitude, longitude);
                    if pressure < -300 {
                        break;
                    }
                }
            }
        }
    }
}
