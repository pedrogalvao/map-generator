use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{
    color_scheme::ColorScheme, layer::MapViewLayer, projection::projection::Projection,
    util::color_over,
};

pub struct CustomPartialMapLayer {
    // color_scheme: CategoryColorScheme,
    layer_name: String,
}

impl CustomPartialMapLayer {
    pub fn new(layer_name: String) -> Self {
        Self {
            // color_scheme: CategoryColorScheme::new_random(1000),
            layer_name,
        }
    }
}

impl<S, P> MapViewLayer<P, S> for CustomPartialMapLayer
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
        let height = base_img.height();
        let width = base_img.width();
        let Some(custom_pmap) = complete_map.custom_pmaps.get(&self.layer_name) else {
            dbg!("Layer not found");
            dbg!(&self.layer_name);
            for layer in complete_map.custom_pmaps.keys() {
                dbg!(layer);
            }
            return;
        };
        dbg!("Layer found");
        let color_scheme = complete_map
            .custom_color_schemes
            .get(&self.layer_name)
            .unwrap();
        for i in 0..height {
            for j in 0..width {
                if let Some([lat, long]) = projection.img_to_map_coords(
                    i as u32,
                    j as u32,
                    base_img.width() as u32,
                    base_img.height() as u32,
                    center_longitude,
                ) {
                    let value = custom_pmap.get(lat, long).clone();
                    let layer_color: Rgba<u8> = color_scheme.get(value).clone();
                    let original_color = base_img.get_pixel(j as u32, i as u32);
                    let result_color = color_over(original_color, &layer_color);
                    base_img.put_pixel(j as u32, i as u32, result_color);
                }
            }
        }
    }
}
