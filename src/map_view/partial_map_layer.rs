use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{
    color_scheme::ColorScheme, layer::MapViewLayer, projection::projection::Projection,
    util::color_over,
};

pub struct PartialMapLayer<S, T, F>
where
    S: MapShape,
    T: Clone,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, T>,
{
    selector: F,
    color_scheme: Box<dyn ColorScheme<T>>,
    _marker: std::marker::PhantomData<S>, // PhantomData to indicate usage of S
}

impl<S, T, F> PartialMapLayer<S, T, F>
where
    S: MapShape,
    T: Clone,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, T>,
{
    pub fn new(selector: F, color_scheme: Box<dyn ColorScheme<T>>) -> Self {
        Self {
            selector: selector,
            color_scheme: color_scheme,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S, T, P, F> MapViewLayer<P, S> for PartialMapLayer<S, T, F>
where
    S: MapShape,
    T: Clone + Default,
    P: Projection,
    F: Fn(&CompleteMap<S>) -> &PartialMap<S, T>,
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
                    let layer_color: Rgba<u8> = self.color_scheme.get(value).clone();
                    let original_color = base_img.get_pixel(j as u32, i as u32);
                    let result_color = color_over(original_color, &layer_color);
                    base_img.put_pixel(j as u32, i as u32, result_color);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! pmap_layer {
    ($layer_name:ident, $color_scheme:expr) => {
        Box::new(PartialMapLayer::new(
            move |m| &m.$layer_name,
            Box::new($color_scheme.clone()),
        ))
    };
    ($layer_name:ident, $t:expr, $color_scheme:expr) => {
        Box::new(PartialMapLayer::new(
            move |m| &m.$layer_name[$t],
            Box::new($color_scheme.clone()),
        ))
    };
}
