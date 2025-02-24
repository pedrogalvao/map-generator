use image::{ImageBuffer, Rgba};
use rocket::tokio::task::LocalEnterGuard;

use crate::{
    complete_map::CompleteMap,
    pipeline_steps::{climate::Climate, vegetation},
    shapes::map_shape::MapShape,
};

use super::{
    color_scheme::{ColorScheme, VEGETATION_COLORS},
    layer::MapViewLayer,
    projection::projection::Projection,
    util::color_over,
};

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
                        Climate::Glaciar => Rgba([255, 255, 255, 255]),
                        // Climate::HotDesert | Climate::ColdDesert => Rgba([200, 0, 0, 255]),
                        _ => {
                            let vegetation =
                                complete_map.vegetation_density.get(latitude, longitude);
                            // if vegetation > 0 && vegetation < 990 {
                            //     dbg!(vegetation);
                            // }
                            let mut color = VEGETATION_COLORS.get(vegetation);
                            let climate = complete_map.climate.get(latitude, longitude);
                            if climate == Climate::Tundra {
                                let mut max_temperature = -1.0;
                                for temp_pmap in &complete_map.temperature {
                                    let temperature = temp_pmap.get(latitude, longitude);
                                    if temperature > max_temperature {
                                        max_temperature = temperature;
                                    }
                                }
                                if max_temperature < 5.0 {
                                    color = color_over(
                                        &color,
                                        &Rgba([
                                            255,
                                            255,
                                            255,
                                            255 - (63.0 * (max_temperature - 1.0)) as u8,
                                        ]),
                                    );
                                }
                            }
                            color
                        }
                    };

                    let original_color = base_img.get_pixel(j as u32, i as u32);
                    let result_color = color_over(original_color, &layer_color);
                    base_img.put_pixel(j as u32, i as u32, result_color);
                }
            }
        }
    }
}
