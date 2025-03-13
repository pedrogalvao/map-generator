use std::sync::Arc;

use crate::{
    complete_map::CompleteMap, partial_map::load_categories_from_img, shapes::map_shape::MapShape,
};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct LoadCustomLayer {
    filename: String,
}

impl LoadCustomLayer {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}

impl<S: MapShape> PipelineStep<S> for LoadCustomLayer {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        dbg!("loading layer");
        dbg!(&self.filename);
        let mut output_map = input_map.clone();
        let (custom_pmap, color_scheme) = load_categories_from_img(&self.filename);

        output_map.custom_pmaps.insert(
            self.filename.split('/').last().unwrap().to_string(),
            custom_pmap,
        );

        output_map.custom_color_schemes.insert(
            self.filename.split('/').last().unwrap().to_string(),
            color_scheme,
        );

        return output_map;
    }
}
