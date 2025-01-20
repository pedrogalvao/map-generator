use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{layer::MapViewLayer, projection::projection::Projection, util::color_over};

pub struct RiversLayer {
    pub color: Rgba<u8>,
}

impl RiversLayer {
    pub fn new(color: Rgba<u8>) -> Self {
        Self { color }
    }
    pub fn default() -> Self {
        Self {
            color: Rgba([20, 50, 255, 255]),
        }
    }
}

impl<P, S> MapViewLayer<P, S> for RiversLayer
where
    S: MapShape,
    P: Projection,
{
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {
        for river in complete_map.rivers.iter() {
            let mut opt_prev_img_x = None;
            let mut opt_prev_img_y = None;
            for point in river {
                let latitude =
                    point.position[0] as f32 * 180.0 / complete_map.height.height as f32 - 90.0;
                let longitude = point.position[1] as f32 * 360.0
                    / complete_map.height.values[point.position[0]].len() as f32
                    - 180.0;
                if let Some([img_x, img_y]) = projection.map_to_img_coords(
                    latitude,
                    longitude,
                    base_img.width(),
                    base_img.height(),
                    center_longitude,
                ) {
                    if let Some(prev_img_x) = opt_prev_img_x {
                        if let Some(prev_img_y) = opt_prev_img_y {
                            // ensure continuity
                            let dist_x = (prev_img_x as i32 - img_x as i32).abs();
                            let dist_y = (prev_img_y as i32 - img_y as i32).abs();
                            if (dist_x > 1 || dist_y > 1) && dist_y < base_img.width() as i32 / 16 {
                                let n = dist_x.max(dist_y);
                                for i in 0..n {
                                    let x = ((prev_img_x as i32 * i + img_x as i32 * (n - i)) / n)
                                        as usize;
                                    let y = ((prev_img_y as i32 * i + img_y as i32 * (n - i)) / n)
                                        as usize;
                                    if x as u32 >= base_img.height() || y as u32 >= base_img.width()
                                    {
                                        continue;
                                    }
                                    let original_color = base_img.get_pixel(y as u32, x as u32);
                                    let new_color = color_over(original_color, &self.color);
                                    base_img.put_pixel(y as u32, x as u32, new_color);
                                }
                            }
                        }
                    };
                    if img_y as u32 >= base_img.width() || img_x as u32 >= base_img.height() {
                        continue;
                    }
                    base_img.put_pixel(img_y as u32, img_x as u32, self.color);
                    opt_prev_img_x = Some(img_x);
                    opt_prev_img_y = Some(img_y);
                } else {
                    opt_prev_img_x = None;
                    opt_prev_img_y = None;
                }
            }
        }
    }
}
