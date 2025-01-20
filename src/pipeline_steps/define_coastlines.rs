// use fasthash::spooky::Hash128;
use std::{collections::HashSet, sync::Arc};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct DefineCoastline {}

impl<S: MapShape> PipelineStep<S> for DefineCoastline {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        let mut coastline = HashSet::new();
        // let mut coastline = HashSet::with_hasher(Hash128);
        for x in 0..input_map.height.values.len() {
            for y in 0..input_map.height.values[x].len() {
                if input_map.height.values[x][y] <= 0 {
                    for row in input_map.height.get_pixel_neighbours([x, y], 1) {
                        for value in row {
                            if value > 0 {
                                coastline.insert([x, y]);
                            }
                        }
                    }
                }
            }
        }
        output_map.coastline = Some(coastline);
        return output_map;
    }
}
