use std::{f32::consts::PI, sync::Arc};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct CalculateContinentality {}

impl<S: MapShape> PipelineStep<S> for CalculateContinentality {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let [latitude, longitude] = input_map.continentality.convert_coords(x, y);
        let latitude_abs = latitude.abs();
        let latitude_influence;
        latitude_influence = 20.0 - 20.0 * (2.0 * latitude_abs * PI / 180.0).cos();
        // if latitude_abs < 10.0 {
        //     latitude_influence = -30 + 3 * latitude_abs as i32;
        // } else if latitude_abs < 25.0 {
        //     latitude_influence = (2.0 * (latitude_abs - 10.0)) as i32;
        // } else if latitude_abs < 55.0 {
        //     latitude_influence = 30 + (latitude_abs - 25.0) as i32;
        // } else {
        //     latitude_influence = 60;
        // }

        let mut others = 0;
        for i in -10..10 {
            for j in -40..15 {
                let height2 = input_map
                    .height
                    .get(latitude + 1.2 * i as f32, longitude + 2.4 * j as f32);
                if height2 <= 0 {
                    others += 1;
                }
            }
        }
        for i in -10..10 {
            for j in -15..15 {
                let height2 = input_map
                    .height
                    .get(latitude + 0.5 * i as f32, longitude + 1.0 * j as f32);
                if height2 <= 0 {
                    others += 3;
                }
            }
        }
        let max_others = 20 * 30 + 20 * 55;
        let height: i32 = input_map.height.get(latitude, longitude);
        if height <= 0 {
            return (latitude_influence - (17.0 * others as f32 / max_others as f32 - 10.0)).max(0.0)
                as i32
                / 2;
        }
        return (latitude_influence - (17.0 * others as f32 / max_others as f32 - 8.0)).max(0.0)
            as i32
            / 2;
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
                    *num = self.process_element(x, y, Arc::new(&in2));
                });
            });

        return output_map;
    }
}
