use std::collections::hash_map::DefaultHasher;
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
    vec,
};

use serde::Deserialize;

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Deserialize)]
pub struct Resize {
    pub factor: f32,
}

impl fmt::Debug for Resize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resize")
            .field("factor", &self.factor)
            .finish()
    }
}

pub fn resize<S: MapShape, T: Clone + Default>(pmap: &mut PartialMap<S, T>, factor: f32) {
    let mut new_values = S::new_vec(
        (factor * pmap.circunference as f32) as usize,
        (factor * pmap.height as f32) as usize,
    );

    for i in 0..new_values.len() {
        for j in 0..new_values[i].len() {
            let i0 = (i as f32 / factor) as usize;
            let row_len = pmap.values[i0].len();
            new_values[i][j] = pmap.values[i0][j * row_len / new_values[i].len()].clone();
        }
    }
    pmap.height = (factor * pmap.height as f32) as usize;
    pmap.circunference = (factor * pmap.circunference as f32) as usize;
    pmap.values = new_values;
}

fn pseudo_random_float(seed: u32) -> f32 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash = hasher.finish();
    let random_float: f32 = hash as f32 / u64::MAX as f32;
    return random_float;
}

pub fn resize_chains(chains: &Vec<[f32; 2]>) -> Vec<[f32; 2]> {
    let mut new_values = vec![];
    for i in 1..chains.len() {
        // new_values.push(chains[i-1]);
        if (chains[i - 1][0] - chains[i][0]).abs() < 2.0
            && (chains[i - 1][1] - chains[i][1]).abs() < 2.0
        {
            let r1 =
                (pseudo_random_float(((chains[i - 1][0] + chains[i][0]) * 5.0) as u32) - 0.5) * 2.0;
            let r2 =
                (pseudo_random_float(((chains[i - 1][1] + chains[i][1]) * 5.0) as u32) - 0.5) * 2.0;
            let r3 =
                (pseudo_random_float(((chains[i - 1][1] + chains[i][0]) * 5.0) as u32) - 0.5) * 2.0;
            let r4 =
                (pseudo_random_float(((chains[i - 1][0] + chains[i][1]) * 5.0) as u32) - 0.5) * 2.0;
            let new_point = [
                r1 + (chains[i - 1][0] * 2.0 + chains[i][0]) / 3.0,
                r2 + (chains[i - 1][1] * 2.0 + chains[i][1]) / 3.0,
            ];
            new_values.push(new_point);
            let new_point = [
                r3 + (chains[i - 1][0] + chains[i][0] * 2.0) / 3.0,
                r4 + (chains[i - 1][1] + chains[i][1] * 2.0) / 3.0,
            ];
            new_values.push(new_point);
        }
    }
    new_values
}

fn smooth_plates<S: MapShape>(pmap: &mut PartialMap<S, usize>) {
    let mut new_values = vec![];
    for (i, row) in pmap.values.iter().enumerate() {
        new_values.push(vec![]);
        for (j, _cell) in row.iter().enumerate() {
            let neighbors = pmap.get_pixel_neighbours([i, j], 1);
            if neighbors.len() < 3 || neighbors[0].len() < 3 || neighbors[2].len() < 3 {
                new_values[i].push(pmap.values[i][j]);
            } else if neighbors[0][0] == neighbors[2][2] {
                new_values[i].push(neighbors[0][0]);
            } else if neighbors[0][2] == neighbors[2][0] {
                new_values[i].push(neighbors[0][2]);
            } else {
                new_values[i].push(neighbors[1][1]);
            }
        }
    }
    pmap.values = new_values;
}

impl<S: MapShape> PipelineStep<S> for Resize {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        resize(&mut output_map.height, self.factor);
        resize(&mut output_map.tectonic_plates, self.factor);
        let mut new_values = vec![];
        for x in 0..output_map.height.values.len() {
            new_values.push(vec![]);
            for y in 0..output_map.height.values[x].len() {
                let neighbors = output_map.height.get_pixel_neighbours([x, y], 1);
                let mut sum = 0;
                let mut len = 0;
                for i in 0..neighbors.len() {
                    for j in 0..neighbors[i].len() {
                        sum += neighbors[i][j];
                        len += 1;
                    }
                }
                let mean = sum / len;
                new_values[x].push(mean);
            }
        }
        output_map.height.values = new_values;
        smooth_plates(&mut output_map.tectonic_plates);
        smooth_plates(&mut output_map.tectonic_plates);
        output_map.mountain_chains = resize_chains(&output_map.mountain_chains);
        output_map.andean_chains = resize_chains(&output_map.andean_chains);
        output_map.hymalayan_chains = resize_chains(&output_map.hymalayan_chains);
        output_map.trenches = resize_chains(&output_map.trenches);
        return output_map;
    }
}
