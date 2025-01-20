use std::{fmt, sync::Arc};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::CustomNoise};

pub struct SupercontinentHeightNoise {
    pub noise: CustomNoise,
    intensity: f32,
}

impl SupercontinentHeightNoise {
    pub fn new(seed: u32, frequency: f32, intensity: f32) -> Self {
        SupercontinentHeightNoise {
            noise: CustomNoise::new(seed, frequency, intensity),
            intensity,
        }
    }
}

impl fmt::Debug for SupercontinentHeightNoise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SupercontinentHeightNoise")
            .field("intensity", &self.intensity)
            .finish()
    }
}

impl<S: MapShape> PipelineStep<S> for SupercontinentHeightNoise {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let [latitude, longitude] = input_map.height.convert_coords(x, y);
        let plate_number = input_map.tectonic_plates.get(latitude, longitude);
        let plate_center = input_map.tectonic_plates_centers[plate_number];
        let mut noise_value = self.noise.get_spheric_f32::<S, i32>(latitude, longitude);
        if plate_center[0].powf(2.0) + plate_center[1].powf(2.0)
            > (45.0 as f32).powf(2.0) + (90.0 as f32).powf(2.0)
        {
            noise_value = (2.0 * noise_value - noise_value.abs()) / 3.0;
        } else {
            noise_value = (2.0 * noise_value + noise_value.abs()) / 3.0;
        }
        return input_map.height.values[x][y] + noise_value as i32;
    }
}
