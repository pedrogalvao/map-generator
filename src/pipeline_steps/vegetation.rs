use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};
use std::{fmt, sync::Arc};

use super::{
    climate::Climate, pipeline_step::PipelineStep, smooth::smooth_pmap, util::CustomNoise,
};

pub struct Vegetation {
    pub noise: CustomNoise,
    pub noise2: CustomNoise,
    pub noise3: CustomNoise,
}

impl fmt::Debug for Vegetation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vegetation").finish()
    }
}

impl Vegetation {
    pub fn new() -> Self {
        Vegetation {
            noise: CustomNoise::new(0, 100.0, 30.0),
            noise2: CustomNoise::new(1, 200.0, 25.0),
            noise3: CustomNoise::new(2, 400.0, 15.0),
        }
    }
}

impl<S: MapShape> PipelineStep<S> for Vegetation {
    fn process_element(&self, x: usize, y: usize, complete_map: Arc<&CompleteMap<S>>) -> i32 {
        let [latitude, longitude] = complete_map.vegetation_density.convert_coords(x, y);
        let noise_value = self.noise.get_spheric::<S, i32>(latitude, longitude)
            + self.noise2.get_spheric::<S, i32>(latitude, longitude)
            + self.noise3.get_spheric::<S, i32>(latitude, longitude);

        let climate_type = complete_map.climate.get(latitude, longitude);
        match climate_type {
            Climate::HotDesert | Climate::ColdDesert | Climate::Glaciar => {
                return 0;
            }
            _ => {}
        }

        let mut avg_temperature = 0.0;
        let mut summer_precipitation = 0;
        let mut winter_precipitation = 0;
        let mut min_precipitation = 999;
        for i in 0..complete_map.precipitation.len() {
            let prec_map = &complete_map.precipitation[i];
            let precipitation = prec_map.get(latitude, longitude);
            if (latitude < 0.0)
                != (i < complete_map.precipitation.len() / 4
                    || i >= 3 * complete_map.precipitation.len() / 4)
            {
                winter_precipitation += precipitation;
            } else {
                summer_precipitation += precipitation;
            }
            if min_precipitation > precipitation {
                min_precipitation = precipitation;
            }
            avg_temperature += complete_map.temperature[i].get(latitude, longitude);
        }
        avg_temperature /= complete_map.temperature.len() as f32;
        let annual_precipitation = complete_map.annual_precipitation.get(latitude, longitude);

        let aridity_index;
        if summer_precipitation as f32 >= 0.7 * annual_precipitation as f32
            || winter_precipitation as f32 >= 0.7 * annual_precipitation as f32
        {
            aridity_index = (20.0 * avg_temperature + 400.0) - annual_precipitation as f32;
        } else if summer_precipitation as f32 >= 0.6 * annual_precipitation as f32
            || winter_precipitation as f32 >= 0.6 * annual_precipitation as f32
        {
            aridity_index = (20.0 * avg_temperature + 300.0) - annual_precipitation as f32;
        } else {
            aridity_index = (20.0 * avg_temperature + 200.0) - annual_precipitation as f32;
        }

        let mut vegetation = 500 + 60 * 8;
        if aridity_index > 0.0 {
            vegetation = vegetation.min((750 - (1.5 * aridity_index) as i32 + noise_value).max(0));
        }

        if min_precipitation < 60 {
            vegetation = vegetation.min(400 + min_precipitation * 8 + noise_value);
        }

        return vegetation + noise_value;
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.vegetation_density =
            PartialMap::new(input_map.height.circunference, input_map.height.height);
        let in2 = output_map.clone();
        // process elements in parallel with rayon
        output_map
            .vegetation_density
            .values
            .par_iter_mut()
            .enumerate()
            .for_each(|(x, inner_vec)| {
                inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
                    *num = self.process_element(x, y, Arc::new(&in2));
                });
            });
        output_map.vegetation_density = smooth_pmap(&output_map.vegetation_density, 2);

        return output_map;
    }
}
