use std::sync::Arc;

use crate::{complete_map::CompleteMap, partial_map::load_from_img, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct LoadHeight {
    filename: String,
}

impl LoadHeight {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}

impl<S: MapShape> PipelineStep<S> for LoadHeight {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        output_map.height = load_from_img(&self.filename, -5000, 6400);

        return output_map;
    }
}
