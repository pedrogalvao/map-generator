use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{
    color_scheme::TextureColorScheme, layer::MapViewLayer, projection::projection::Projection,
    util::color_over,
};

pub struct TextureLayer<S, F>
where
    S: MapShape,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, i32>,
{
    selector: F,
    color_scheme: Box<TextureColorScheme>,
    _marker: std::marker::PhantomData<S>, // PhantomData to indicate usage of S
    transparency: u8,
}

impl<S, F> TextureLayer<S, F>
where
    S: MapShape,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, i32>,
{
    pub fn new(selector: F, color_scheme: Box<TextureColorScheme>) -> Self {
        Self {
            selector: selector,
            color_scheme: color_scheme,
            transparency: 100,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S, P, F> MapViewLayer<P, S> for TextureLayer<S, F>
where
    S: MapShape,
    P: Projection,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, i32>,
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
        for i in 0..height {
            for j in 0..width {
                if let Some([lat, long]) = projection.img_to_map_coords(
                    i as u32,
                    j as u32,
                    base_img.width() as u32,
                    base_img.height() as u32,
                    center_longitude,
                ) {
                    let value = (self.selector)(complete_map).get(lat, long).clone();
                    let mut layer_color = self.color_scheme.get(value, j, i);
                    layer_color[3] = self.transparency;
                    let original_color = base_img.get_pixel(j as u32, i as u32);
                    let result_color = color_over(original_color, &layer_color);
                    base_img.put_pixel(j as u32, i as u32, result_color);
                }
            }
        }
    }
}
