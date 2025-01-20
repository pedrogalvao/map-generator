use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};
use std::sync::Arc;

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct SmoothPrecipitation {
    pub pixel_distance: usize,
}

impl SmoothPrecipitation {
    pub fn new() -> Self {
        Self { pixel_distance: 2 }
    }
}

impl<S: MapShape> PipelineStep<S> for SmoothPrecipitation {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let neighbors = input_map
            .annual_precipitation
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

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        // process elements in parallel with rayon
        output_map
            .annual_precipitation
            .values
            .par_iter_mut()
            .enumerate()
            .for_each(|(x, inner_vec)| {
                inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
                    *num = self.process_element(x, y, Arc::new(input_map));
                });
            });
        return output_map;
    }
}

#[derive(Debug)]
pub struct CalculateAnnualPrecipitation {}

impl<S: MapShape> PipelineStep<S> for CalculateAnnualPrecipitation {
    fn process_element(&self, x: usize, y: usize, complete_map: Arc<&CompleteMap<S>>) -> i32 {
        let mut total = 0;
        for precipitation_map in &complete_map.precipitation {
            total += precipitation_map.values[x][y];
        }
        total * 12 / complete_map.precipitation.len() as i32
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.annual_precipitation = PartialMap::new(
            input_map.precipitation[0].circunference,
            input_map.precipitation[0].height,
        );
        output_map
            .annual_precipitation
            .values
            .par_iter_mut()
            .enumerate()
            .for_each(|(x, inner_vec)| {
                inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
                    *num = self.process_element(x, y, Arc::new(input_map));
                });
            });
        let output_map = SmoothPrecipitation::new().apply(&output_map);
        let output_map = SmoothPrecipitation::new().apply(&output_map);
        return output_map;
    }
}
