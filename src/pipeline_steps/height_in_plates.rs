use std::{collections::HashMap, sync::Arc};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::CustomNoise};

#[derive(Debug)]
pub struct HeightInPlates {
    pub noise_intensity: f32,
    pub noise_frequency: f32,
    noises: HashMap<usize, CustomNoise>,
}

impl HeightInPlates {
    pub fn new(seed: u32, noise_frequency: f32, noise_intensity: f32) -> Self {
        let mut noises = HashMap::new();
        for key in 0..100 {
            let noise =
                CustomNoise::new(1000 * (key as u32) + seed, noise_frequency, noise_intensity);
            noises.insert(key, noise);
        }
        HeightInPlates {
            noise_intensity,
            noise_frequency,
            noises,
        }
    }
}

impl HeightInPlates {
    pub fn get_noise(&self, key: usize) -> &CustomNoise {
        return self.noises.get(&(key % 100)).unwrap();
    }
}

impl<S: MapShape> PipelineStep<S> for HeightInPlates {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let noise = self.get_noise(input_map.tectonic_plates.values[x][y]);

        let [lat, lon] = input_map.height.convert_coords(x, y);
        let noise_value = noise.get_spheric::<S, i32>(lat, lon);

        return input_map.height.values[x][y] + noise_value as i32;
    }
}
