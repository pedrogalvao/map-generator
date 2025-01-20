use std::sync::Arc;

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::CustomNoise};

#[derive(Debug)]
pub struct HeightNoiseMult {
    pub noise: CustomNoise,
}

impl HeightNoiseMult {
    pub fn new(seed: u32, frequency: f32, intensity: f32) -> Self {
        HeightNoiseMult {
            noise: CustomNoise::new(seed, frequency, intensity),
        }
    }
}

impl<S: MapShape> PipelineStep<S> for HeightNoiseMult {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let num = input_map.height.values[x][y];
        if num < 15 {
            return num;
        }
        let [lat, lon] = input_map.height.convert_coords(x, y);
        let noise_value = self.noise.get_spheric_f32::<S, i32>(lat, lon);
        let multiplier = (1.0 + noise_value).max(0.2);
        return (num as f32 * multiplier) as i32;
    }
}
