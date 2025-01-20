use std::fmt;
use std::sync::Arc;

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::CustomNoise};

pub struct TranslationNoise {
    seed: u32,
    noises: [CustomNoise; 8],
}

impl TranslationNoise {
    pub fn new(seed: u32) -> Self {
        TranslationNoise {
            seed,
            noises: [
                CustomNoise::new(seed, 6.0, 5.0),
                CustomNoise::new(seed + 1, 6.0, 5.0),
                CustomNoise::new(seed, 12.0, 3.0),
                CustomNoise::new(seed + 1, 12.0, 3.0),
                CustomNoise::new(seed, 24.0, 2.0),
                CustomNoise::new(seed + 1, 24.0, 2.0),
                CustomNoise::new(seed, 48.0, 1.0),
                CustomNoise::new(seed + 1, 48.0, 1.0),
            ],
        }
    }
}

impl fmt::Debug for TranslationNoise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TranslationNoise").finish()
    }
}

impl<S: MapShape> PipelineStep<S> for TranslationNoise {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let [latitude, longitude] = input_map.height.convert_coords(x, y);

        let noise_lat = self.noises[0].get_spheric_f32::<S, i32>(latitude, longitude)
            + self.noises[2].get_spheric_f32::<S, i32>(latitude, longitude)
            + self.noises[4].get_spheric_f32::<S, i32>(latitude, longitude)
            + self.noises[6].get_spheric_f32::<S, i32>(latitude, longitude);
        let noise_lon = self.noises[1].get_spheric_f32::<S, i32>(latitude, longitude)
            + self.noises[3].get_spheric_f32::<S, i32>(latitude, longitude)
            + self.noises[5].get_spheric_f32::<S, i32>(latitude, longitude)
            + self.noises[7].get_spheric_f32::<S, i32>(latitude, longitude);

        return input_map
            .height
            .get(latitude + noise_lat, longitude + noise_lon);
    }
}
