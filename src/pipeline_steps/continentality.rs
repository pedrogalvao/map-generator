use std::{f32::consts::PI, sync::Arc};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::rotate_coords};

#[derive(Debug)]
pub struct CalculateContinentality {}

impl CalculateContinentality {
    fn process_cont_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
    ) -> f32 {
        let [latitude, longitude] = input_map.continentality.convert_coords(x, y);
        let latitude_abs = latitude.abs();
        let latitude_influence;
        latitude_influence = 25.0 - 20.0 * (2.0 * latitude_abs * PI / 180.0).cos();

        let mut others = 0;
        for i in -10..10 {
            for j in -40..0 {
                let height2 = input_map
                    .height
                    .get(latitude + 1.0 * i as f32, longitude + 1.4 * j as f32);
                if height2 <= 0 {
                    others += 1;
                }
            }
        }
        for i in -10..10 {
            for j in -12..0 {
                let height2 = input_map
                    .height
                    .get(latitude + 0.5 * i as f32, longitude + 1.0 * j as f32);
                if height2 <= 0 {
                    others += 2;
                }
            }
        }
        let max_others = 20 * 12 + 20 * 40;
        let height: i32 = input_map.height.get(latitude, longitude);
        if height <= 0 {
            if latitude > 60.0 {
                return (latitude_influence
                    - 7.0
                    - (16.0 * others as f32 / max_others as f32 - 10.0))
                    .max(0.0) as f32
                    / 2.0;
            } else {
                return (latitude_influence
                    - 3.0
                    - (16.0 * others as f32 / max_others as f32 - 10.0))
                    .max(0.0) as f32
                    / 2.0;
            }
        }
        return (latitude_influence - (6.0 * others as f32 / max_others as f32 - 8.0)).max(0.0)
            as f32
            / 2.0;
    }
    fn process_cont_element2<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
    ) -> f32 {
        let [latitude, longitude] = input_map.continentality.convert_coords(x, y);
        let latitude_abs = latitude.abs();
        let latitude_influence;
        latitude_influence = 25.0 - 20.0 * (2.0 * latitude_abs * PI / 180.0).cos();

        let mut ocean_influence = 0.0;
        let mut max_ocean_influence = 0.0;
        for i in 0..12 {
            let bearing = i as f32 * 2.0 * PI / 12.0;
            let max_dist =
                ((1.0 - (latitude * PI / 60.0).cos().max(0.0)) * (0.4 - bearing.sin()) * 25.0)
                    .max(0.0)
                    + (latitude * PI / 60.0).cos().max(0.0) * 25.0;
            let mut multiplier = 1.5;
            for dist in 1..=max_dist as i32 {
                let dist_rad = 1.5 * dist as f32 * PI / 180.0;
                let [latitude2, longitude2] = rotate_coords(latitude, longitude, dist_rad, bearing);
                let height = input_map.height.get(latitude2, longitude2);
                if height <= 0 {
                    ocean_influence += multiplier * 1.0;
                }
                max_ocean_influence += multiplier * 1.0;
                if height > 2500 {
                    multiplier *= 0.4;
                } else if height > 1500 {
                    multiplier *= 0.6;
                }
                multiplier *= 0.95;
            }
        }

        return (0.6
            * (latitude_influence - 10.0 * ocean_influence as f32 / max_ocean_influence as f32))
            .max(0.0);
    }
}
impl<S: MapShape> PipelineStep<S> for CalculateContinentality {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.continentality =
            PartialMap::new(input_map.height.circunference, input_map.height.height);
        let in2 = output_map.clone();
        // process elements in parallel with rayon
        output_map
            .continentality
            .values
            .par_iter_mut()
            .enumerate()
            .for_each(|(x, inner_vec)| {
                inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
                    *num = self.process_cont_element(x, y, Arc::new(&in2));
                });
            });

        return output_map;
    }
}
