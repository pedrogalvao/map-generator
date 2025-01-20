use std::sync::Arc;

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct DiamondSquare {}

fn diamond_square<S: MapShape>(pmap: &mut PartialMap<S, i32>) {
    let height = pmap.values.len();
    let mut new_values = vec![];

    for i in 0..pmap.values.len() {
        new_values.push(vec![]);
        new_values.push(vec![]);
        for j in 0..pmap.values[i].len() {
            let width = pmap.values[i].len();
            let curr_value = pmap.values[i][(j + 1) % width];
            let right_value = pmap.values[i][(j + 1) % width];
            let down_value = pmap.values[(i + 1).min(height - 1)][(j + 1) % width];
            let right_down_value = pmap.values[(i + 1).min(height - 1)][j];

            let new_values_len = new_values.len();

            let central_value = (curr_value + right_value + down_value + right_down_value) / 4;

            new_values[new_values_len - 2].push(curr_value);

            if new_values_len >= 3 {
                let new_value_up =
                    new_values[new_values_len - 3][new_values[new_values_len - 1].len() + 1];
                let new_value_right = (curr_value + right_value + new_value_up + central_value) / 4;
                // new_value_right = (new_value_right as f32 * (0.9 + rng.gen::<f32>() * 0.2)) as i32;
                // new_value_right += (rng.gen::<f32>() * 2.0 - 1.0) as i32;
                new_values[new_values_len - 2].push(new_value_right);
            } else {
                let new_value_right = (curr_value + right_value + central_value) / 3;
                new_values[new_values_len - 2].push(new_value_right);
            }

            if new_values[new_values_len - 1].len() > 0 {
                let new_value_left =
                    new_values[new_values_len - 1][new_values[new_values_len - 1].len() - 1];
                let new_value_down = (curr_value + down_value + new_value_left + central_value) / 4;
                // new_value_down = (new_value_down as f32 * (0.9 + rng.gen::<f32>() * 0.2)) as i32;
                // new_value_down += (rng.gen::<f32>() * 2.0 - 1.0) as i32;
                new_values[new_values_len - 1].push(new_value_down);
            } else {
                let new_value_down = (curr_value + down_value + central_value) / 3;
                new_values[new_values_len - 1].push(new_value_down);
            }

            new_values[new_values_len - 1].push(central_value);
        }
    }
    pmap.height *= 2;
    pmap.circunference *= 2;
    pmap.values = new_values;
}

fn diamond_square_usize<S: MapShape>(pmap: &mut PartialMap<S, usize>) {
    let mut new_values = vec![];

    for i in 0..pmap.values.len() {
        new_values.push(vec![]);
        new_values.push(vec![]);
        for j in 0..pmap.values[i].len() {
            let width = pmap.values[i].len();
            let new_values_len = new_values.len();
            let curr_value = pmap.values[i][(j + 1) % width];
            new_values[new_values_len - 2].push(curr_value);
            new_values[new_values_len - 2].push(curr_value);
            new_values[new_values_len - 1].push(curr_value);
            new_values[new_values_len - 1].push(curr_value);
        }
    }
    pmap.height *= 2;
    pmap.circunference *= 2;
    pmap.values = new_values;
}

impl<S: MapShape> PipelineStep<S> for DiamondSquare {
    fn process_element(&self, _x: usize, _y: usize, _pmap: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        diamond_square(&mut output_map.height);
        diamond_square_usize(&mut output_map.tectonic_plates);
        return output_map;
    }
}
