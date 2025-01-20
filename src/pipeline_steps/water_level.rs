use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::percentile_2d_vector};

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterLevel {
    pub percentage: f32,
}

impl WaterLevel {
    pub fn new() -> Self {
        WaterLevel { percentage: 75.0 }
    }
}

impl<S: MapShape> PipelineStep<S> for WaterLevel {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let sub_value = percentile_2d_vector(&input_map.height.values, self.percentage).unwrap();
        let output_map = AddHeight::new(-sub_value).apply(input_map);
        return output_map;
    }
}

#[derive(Debug)]
pub struct AddHeight {
    pub value: i32,
}

impl AddHeight {
    pub fn new(value: i32) -> Self {
        AddHeight { value }
    }
}

impl<S: MapShape> PipelineStep<S> for AddHeight {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        return input_map.height.values[x][y] + self.value;
    }
}
