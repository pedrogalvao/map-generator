use std::{f64::consts::PI, sync::Arc};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::CustomNoise};

#[derive(Debug)]
pub struct HeightNoisePoles {
    pub noise: CustomNoise,
}

impl HeightNoisePoles {
    pub fn new(seed: u32, frequency: f32, intensity: f32) -> Self {
        HeightNoisePoles {
            noise: CustomNoise::new(seed, frequency, intensity),
        }
    }
}

impl<S: MapShape> PipelineStep<S> for HeightNoisePoles {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let pole_factor = (PI as f32 * (x as f32) / input_map.height.height as f32)
            .cos()
            .powf(2.0);
        let [lat, lon] = input_map.height.convert_coords(x, y);
        let noise_value = self.noise.get_spheric_f32::<S, i32>(lat, lon);
        return input_map.height.values[x][y] + (noise_value * pole_factor) as i32;
    }
}
