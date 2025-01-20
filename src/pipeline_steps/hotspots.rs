use std::sync::Arc;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct Hotspots {
    pub n_regions: usize,
    seed: u32,
}

impl Hotspots {
    pub fn new(seed: u32, n_regions: usize) -> Self {
        Hotspots {
            n_regions: n_regions,
            seed,
        }
    }
}

fn pseudo_random_float(seed: u32) -> f32 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash = hasher.finish();
    let random_float: f32 = hash as f32 / u64::MAX as f32;
    return random_float.abs();
}

impl<S: MapShape> PipelineStep<S> for Hotspots {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        // let mut plates = vec![];
        output_map.hotspots = vec![];

        let points: Vec<[usize; 2]> = output_map
            .tectonic_plates
            .get_random_points_from_seed(self.n_regions, self.seed);

        output_map.tectonic_plates_centers = vec![];
        for i in 0..points.len() {
            let [x, y] = points[i];
            let [mut latitude, mut longitude] = output_map.tectonic_plates.convert_coords(x, y);
            let mut height = output_map.height.get(latitude, longitude);
            if height > -300 {
                continue;
            }
            let mut hotspot = vec![];
            let mut i = 0;
            let mut multiplier = 1.0;
            while height < -200 && i < 50 {
                i += 1;
                let plate = output_map.tectonic_plates.get(latitude, longitude);
                let plate_direction = output_map.tectonic_plates_directions[plate];
                latitude -= 0.7 * plate_direction[0]
                    + 0.5
                        * (2.0
                            * pseudo_random_float(
                                i + (100.0 * plate_direction[0] + latitude) as u32,
                            )
                            - 1.0);
                longitude -= 0.7 * plate_direction[1]
                    + 0.5
                        * (2.0
                            * pseudo_random_float(
                                i + (100.0 * plate_direction[1] + longitude) as u32,
                            )
                            - 1.0);
                hotspot.push([latitude, longitude]);
                height = output_map.height.get(latitude, longitude);
                let [x, y] = output_map.height.convert_to_vec_coords(latitude, longitude);
                // output_map.height.values[x][y] = ((output_map.height.values[x][y] as f32) / (5.0 * multiplier)) as i32;
                output_map.height.values[x][y] =
                    ((output_map.height.values[x][y] as f32) / 5.0) as i32;
                let random_h: f32 = pseudo_random_float(
                    i + (100.0 * plate_direction[0] + latitude) as u32
                        + (100.0 * plate_direction[1] + longitude) as u32,
                );
                output_map.height.values[x][y] += (800.0 * multiplier * random_h) as i32;
                for row in output_map.height.get_pixel_neighbours_coords([x, y], 1) {
                    for [x2, y2] in row {
                        output_map.height.values[x2][y2] =
                            ((output_map.height.values[x][y] as f32) / 5.0) as i32;
                        let random_h2: f32 = pseudo_random_float(
                            i + (100.0 * plate_direction[0] + x2 as f32) as u32
                                + (100.0 * plate_direction[1] + y2 as f32) as u32,
                        );
                        output_map.height.values[x2][y2] +=
                            (800.0 * multiplier * random_h2 * random_h) as i32;
                    }
                }
                multiplier *= 0.9
            }
        }

        return output_map;
    }
}
