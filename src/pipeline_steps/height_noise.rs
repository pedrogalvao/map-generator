use std::sync::Arc;

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::CustomNoise};

#[derive(Debug)]
pub struct HeightNoise {
    pub noise: CustomNoise,
}

impl HeightNoise {
    pub fn new(seed: u32, frequency: f32, intensity: f32) -> Self {
        HeightNoise {
            noise: CustomNoise::new(seed, frequency, intensity),
        }
    }
}

impl<S: MapShape> PipelineStep<S> for HeightNoise {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let [lat, lon] = input_map.height.convert_coords(x, y);
        let noise_value = self.noise.get_spheric::<S, i32>(lat, lon);

        return input_map.height.values[x][y] + noise_value;
    }
}
