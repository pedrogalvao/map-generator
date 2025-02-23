use std::sync::Arc;

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct Smooth {
    pub pixel_distance: usize,
}

impl Smooth {
    pub fn new() -> Self {
        Self { pixel_distance: 2 }
    }
}

impl<S: MapShape> PipelineStep<S> for Smooth {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let neighbors = input_map
            .height
            .get_pixel_neighbours([x, y], self.pixel_distance);
        let mut sum = 0;
        let mut len = 0;
        for i in 0..neighbors.len() {
            for j in 0..neighbors[i].len() {
                sum += neighbors[i][j];
                len += 1;
            }
        }
        let mean = sum / len;
        return mean;
    }
}

#[derive(Debug)]
pub struct SmoothOcean {
    pub pixel_distance: usize,
}

impl SmoothOcean {
    pub fn new() -> Self {
        Self { pixel_distance: 4 }
    }
}

impl<S: MapShape> PipelineStep<S> for SmoothOcean {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        if input_map.height.values[x][y] > 0 {
            return input_map.height.values[x][y];
        }
        let neighbors = input_map
            .height
            .get_pixel_neighbours([x, y], self.pixel_distance);
        let mut sum = 0;
        let mut len = 0;
        for i in 0..neighbors.len() {
            for j in 0..neighbors[i].len() {
                sum += neighbors[i][j];
                len += 1;
            }
        }
        let mean = sum / len;
        return mean.min(0);
    }
}

pub fn smooth_pmap<S: MapShape>(
    input_map: &PartialMap<S, i32>,
    pixel_distance: usize,
) -> PartialMap<S, i32> {
    let mut output_map = PartialMap::new(input_map.circunference, input_map.height);
    // let mut output_map = input_map.clone();
    for x in 0..input_map.values.len() {
        for y in 0..input_map.values[x].len() {
            let neighbors = input_map.get_pixel_neighbours([x, y], pixel_distance);
            let mut sum = 0;
            let mut len = 0;
            for i in 0..neighbors.len() {
                for j in 0..neighbors[i].len() {
                    sum += neighbors[i][j];
                    len += 1;
                }
            }
            let mean = sum / len;
            output_map.values[x][y] = mean;
        }
    }
    return output_map;
}
