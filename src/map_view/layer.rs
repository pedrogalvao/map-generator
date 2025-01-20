use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::projection::projection::Projection;

pub trait MapViewLayer<P: Projection, S: MapShape> {
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    );
}

// struct RiversLayer<S: MapShape> {
//     target: Vec<River>,
//     thickness: u32,
//     color: Rgba<u8>,
// }
