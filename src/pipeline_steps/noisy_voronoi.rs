use std::fmt;
use std::sync::Arc;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::partial_map::PartialMap;
use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::CustomNoise};

pub struct NoisyVoronoi {
    pub n_regions: usize,
    seed: u32,
    noises: [CustomNoise; 2],
}

impl NoisyVoronoi {
    pub fn new(seed: u32, n_regions: usize) -> Self {
        NoisyVoronoi {
            n_regions: n_regions,
            seed,
            noises: [
                CustomNoise::new(seed, 3.0, 10.0),
                CustomNoise::new(seed + 1, 3.0, 10.0),
            ],
        }
    }
}

fn pseudo_random_float(seed: u32) -> f32 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash = hasher.finish();
    let random_float: f32 = hash as f32 / u64::MAX as f32;
    return random_float;
}

impl NoisyVoronoi {
    fn process_element_voronoi<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
        points: Vec<[usize; 2]>,
    ) -> usize {
        let pmap = &input_map.tectonic_plates;
        let perlin_value1 = self.noises[0].get_f32(x, y, pmap);
        let perlin_value2 = self.noises[1].get_f32(x, y, pmap);

        let mut min_distance = f32::MAX;
        let mut closest_idx = 0;

        let [latitude, longitude] = pmap.convert_coords(x, y);
        let coords1 = [latitude + perlin_value1, longitude + perlin_value2];
        for (idx, point) in points.iter().enumerate() {
            let coords2 = pmap.convert_coords(point[0], point[1]);
            let distance = pmap.get_distance(&coords1, &coords2);
            if distance < min_distance {
                min_distance = distance;
                closest_idx = idx;
            }
        }
        return closest_idx;
    }
}

impl fmt::Debug for NoisyVoronoi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoisyVoronoi")
            .field("n_regions", &self.n_regions)
            .finish()
    }
}

impl<S: MapShape> PipelineStep<S> for NoisyVoronoi {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.tectonic_plates =
            PartialMap::<S, usize>::new(input_map.height.circunference, input_map.height.height);

        let points: Vec<[usize; 2]> = output_map
            .tectonic_plates
            .get_random_points_from_seed(self.n_regions, self.seed);

        output_map.tectonic_plates_centers = vec![];
        for i in 0..points.len() {
            let [x, y] = points[i];
            let p_coords = output_map.tectonic_plates.convert_coords(x, y);
            output_map.tectonic_plates_centers.push(p_coords);
        }

        for i in 0..self.n_regions {
            let temp_seed = self.seed + 1 + i as u32;
            let random_float1 = pseudo_random_float(temp_seed);
            let random_float2 = pseudo_random_float(temp_seed + 100);
            let direction = [random_float1 * 2.0 - 1.0, random_float2 * 2.0 - 1.0];
            output_map.tectonic_plates_directions.push(direction);
        }

        let operator =
            move |x, y, arc_map| self.process_element_voronoi(x, y, arc_map, points.clone());
        output_map
            .tectonic_plates
            .iterate_operator(input_map, operator);

        return output_map;
    }
}
