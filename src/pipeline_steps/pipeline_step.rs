use std::{fmt::Debug, sync::Arc};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

pub trait PipelineStep<S: MapShape + Send>: Sync + Debug {
    fn process_element(&self, x: usize, y: usize, complete_map: Arc<&CompleteMap<S>>) -> i32;

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map
            .height
            .iterate_operator(input_map, |x, y, arc_map| {
                self.process_element(x, y, arc_map)
            });

        // version without parallelization
        // output_map.height.values.iter_mut().enumerate().for_each(|(x, inner_vec)| {
        //     inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
        //         *num = self.process_element(x, y, Arc::new(input_map));
        //     });
        // });

        return output_map;
    }
}
