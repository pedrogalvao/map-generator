use image::{ImageBuffer, Rgba};

use crate::{
    complete_map::CompleteMap, pipeline_steps::climate::Climate, shapes::map_shape::MapShape,
};

use super::{layer::MapViewLayer, projection::projection::Projection, util::color_over};

pub struct SatelliteLayer {}

impl<P: Projection, S: MapShape> MapViewLayer<P, S> for SatelliteLayer {
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {
        let height = base_img.height();
        let width = base_img.width();
        for i in 0..height {
            for j in 0..width {
                if let Some([latitude, longitude]) = projection.img_to_map_coords(
                    i as u32,
                    j as u32,
                    base_img.width() as u32,
                    base_img.height() as u32,
                    center_longitude,
                ) {
                    let climate = complete_map.climate.get(latitude, longitude);

                    let layer_color = match climate {
                        Climate::Ocean => Rgba([10, 36, 80, 255]),
                        Climate::HotDesert | Climate::ColdDesert => Rgba([200, 190, 170, 255]),
                        Climate::HotSemiarid | Climate::ColdSemiarid => Rgba([160, 155, 120, 255]),
                        Climate::Savanah => Rgba([92, 110, 80, 255]),
                        Climate::Tropical
                        | Climate::Subarctic
                        | Climate::SubarcticOceanic
                        | Climate::HumidSubtropical => Rgba([52, 85, 52, 255]),
                        Climate::Tundra => Rgba([80, 100, 80, 255]),
                        Climate::Glaciar => Rgba([255, 255, 255, 255]),
                        Climate::Monsoon
                        | Climate::SubtropicalMonsoon
                        | Climate::HotMediterranean
                        | Climate::ColdMediterranean => Rgba([70, 100, 65, 255]),
                        _ => Rgba([70, 100, 65, 255]),
                    };

                    let original_color = base_img.get_pixel(j as u32, i as u32);
                    let result_color = color_over(original_color, &layer_color);
                    base_img.put_pixel(j as u32, i as u32, result_color);
                }
            }
        }
    }
}
