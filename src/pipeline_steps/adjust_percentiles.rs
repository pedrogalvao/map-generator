use std::sync::Arc;

use crate::{
    complete_map::CompleteMap, partial_map::PartialMap, pipeline_steps::util::percentile_2d_vector,
    shapes::map_shape::MapShape,
};

use super::pipeline_step::PipelineStep;

pub fn adjust_percentiles<S: MapShape>(
    percentiles: Vec<(f32, i32)>,
    pmap: &mut PartialMap<S, i32>,
) {
    let minimum = percentile_2d_vector(&pmap.values, 0.0).unwrap();

    let mut v = vec![];
    for (key, value) in &percentiles {
        let curr_value = percentile_2d_vector(&pmap.values, *key).unwrap();
        v.push((*value, curr_value))
    }
    let min_value = minimum.min(percentiles[0].1);

    for i in 0..pmap.values.len() {
        for j in 0..pmap.values[i].len() {
            let mut prev_value = minimum;
            let mut prev_curr_value = min_value;
            for (value, curr_value) in &v {
                if prev_curr_value < pmap.values[i][j] && pmap.values[i][j] <= *curr_value {
                    pmap.values[i][j] = prev_value
                        + ((*value as f32 - prev_value as f32)
                            * (pmap.values[i][j] as f32 - prev_curr_value as f32)
                            / (*curr_value as f32 - prev_curr_value as f32))
                            as i32;
                    break;
                }
                (prev_value, prev_curr_value) = (*value, *curr_value);
            }
        }
    }
}

#[derive(Debug)]
pub struct AdjustHeightPercentiles {
    percentiles: Vec<(f32, i32)>,
}

impl AdjustHeightPercentiles {
    pub fn new(percentiles: Vec<(f32, i32)>) -> Self {
        Self { percentiles }
    }
}

impl<S: MapShape> PipelineStep<S> for AdjustHeightPercentiles {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        adjust_percentiles(self.percentiles.clone(), &mut output_map.height);
        return output_map;
    }
}

#[derive(Debug)]
pub struct AdjustLandHeightPercentiles {
    percentiles: Vec<(f32, i32)>,
    water_percentage: f32,
}

impl AdjustLandHeightPercentiles {
    pub fn new(percentiles: &Vec<(f32, i32)>, water_percentage: f32) -> Self {
        Self {
            percentiles: percentiles.clone(),
            water_percentage,
        }
    }
}

impl<S: MapShape> PipelineStep<S> for AdjustLandHeightPercentiles {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut percentiles = vec![(self.water_percentage, 0)];
        for (k, v) in &self.percentiles {
            let k2: f32 = self.water_percentage + k * (100.0 - self.water_percentage) / 100.0;
            percentiles.push((k2, *v));
        }

        return AdjustHeightPercentiles::new(percentiles).apply(input_map);
    }
}

#[derive(Debug)]
pub struct AdjustOceanDepthPercentiles {
    percentiles: Vec<(f32, i32)>,
    water_percentage: f32,
}

impl AdjustOceanDepthPercentiles {
    pub fn new(percentiles: &Vec<(f32, i32)>, water_percentage: f32) -> Self {
        Self {
            percentiles: percentiles.clone(),
            water_percentage,
        }
    }
}

impl<S: MapShape> PipelineStep<S> for AdjustOceanDepthPercentiles {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut percentiles = vec![];
        for (k, v) in &self.percentiles {
            let k2: f32 = k * self.water_percentage / 100.0;
            percentiles.push((k2, *v));
        }

        return AdjustHeightPercentiles::new(percentiles).apply(input_map);
    }
}

#[derive(Debug)]
pub struct AdjustPrecipitationPercentiles {
    percentiles: Vec<(f32, i32)>,
}

impl AdjustPrecipitationPercentiles {
    pub fn new(percentiles: &Vec<(f32, i32)>) -> Self {
        Self {
            percentiles: percentiles.clone(),
        }
    }
}

impl<S: MapShape> PipelineStep<S> for AdjustPrecipitationPercentiles {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        for pmap in output_map.precipitation.iter_mut() {
            adjust_percentiles(self.percentiles.clone(), pmap);
        }
        return output_map;
    }
}
