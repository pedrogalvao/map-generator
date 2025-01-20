use std::time::Instant;

use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{layer::MapViewLayer, projection::projection::Projection};

pub struct MapView<P: Projection, S: MapShape> {
    pub projection: P,
    pub center: [f32; 2],
    pub resolution: [usize; 2],
    pub time_of_year: usize,
    pub layers: Vec<Box<dyn MapViewLayer<P, S>>>,
}

impl<P: Projection, S: MapShape> MapView<P, S> {
    pub fn new() -> Self {
        Self {
            projection: P::new(),
            center: [0.0, 0.0],
            resolution: [1000, 500],
            time_of_year: 0,
            layers: vec![],
        }
    }

    pub fn draw(&self, complete_map: &CompleteMap<S>, filename: &str) {
        let _ = self.return_image_buffer(complete_map).save(filename);
    }

    pub fn return_image_buffer(
        &self,
        complete_map: &CompleteMap<S>,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let height = self.resolution[0];
        let width = self.resolution[1];

        let mut imgbuffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(height as u32, width as u32);

        for layer in &self.layers {
            layer.draw_layer(
                &mut imgbuffer,
                &self.projection,
                self.center[1],
                complete_map,
            );
        }

        return imgbuffer;
    }
}
