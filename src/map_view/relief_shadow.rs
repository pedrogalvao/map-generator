use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};
use image::{ImageBuffer, Rgba};

use super::{layer::MapViewLayer, projection::projection::Projection, util::color_over};

pub struct ReliefShadowLayer {}

impl<P: Projection, S: MapShape> MapViewLayer<P, S> for ReliefShadowLayer {
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {
        let height = base_img.height();
        let width = base_img.width();
        for i in 1..height {
            for j in 1..width {
                if let Some([latitude, longitude]) = projection.img_to_map_coords(
                    i as u32,
                    j as u32,
                    base_img.width() as u32,
                    base_img.height() as u32,
                    center_longitude,
                ) {
                    let height = complete_map.height.get(latitude, longitude);
                    if height <= 0 {
                        continue;
                    }
                    if let Some([latitude2, longitude2]) = projection.img_to_map_coords(
                        i - 1 as u32,
                        j as u32,
                        base_img.width() as u32,
                        base_img.height() as u32,
                        center_longitude,
                    ) {
                        if let Some([latitude3, longitude3]) = projection.img_to_map_coords(
                            i as u32,
                            j - 1 as u32,
                            base_img.width() as u32,
                            base_img.height() as u32,
                            center_longitude,
                        ) {
                            let left_neighbor_height =
                                complete_map.height.get(latitude2, longitude2);
                            let up_neighbor_height = complete_map.height.get(latitude3, longitude3);
                            let slope = (left_neighbor_height - height).max(0)
                                + (up_neighbor_height - height).max(0);
                            let layer_color;
                            if slope > 200 {
                                layer_color = Rgba([0, 0, 0, 50])
                            } else if slope > 100 {
                                layer_color = Rgba([0, 0, 0, 35])
                            } else if slope > 50 {
                                layer_color = Rgba([0, 0, 0, 25])
                            } else if slope > 20 {
                                layer_color = Rgba([0, 0, 0, 16])
                            } else if slope > 5 {
                                layer_color = Rgba([0, 0, 0, 8])
                            } else if slope > 0 {
                                layer_color = Rgba([0, 0, 0, 4])
                            } else {
                                layer_color = Rgba([0, 0, 0, 0])
                            }
                            let original_color = base_img.get_pixel(j as u32, i as u32);
                            let result_color = color_over(original_color, &layer_color);
                            base_img.put_pixel(j as u32, i as u32, result_color);
                        }
                    }
                }
            }
        }
    }
}
