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
use super::rivers::River;

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

pub fn resize_f32<S: MapShape>(pmap: &mut PartialMap<S, f32>, factor: f32) {
    let mut new_values = S::new_vec(
        (factor * pmap.circunference as f32) as usize,
        (factor * pmap.height as f32) as usize,
    );

    for i in 0..pmap.values.len() {
        // even rows
        let i2 = ((i as f32 * factor) as usize).min(new_values.len() - 1);
        for j in 0..pmap.values[i].len() {
            let j2 = ((j as f32 * new_values[i2].len() as f32 / pmap.values[i].len() as f32)
                as usize)
                .min(new_values[i2].len() - 1);
            new_values[i2][j2] = pmap.values[i][j];
            if j2 + 1 < new_values[i2].len() {
                new_values[i2][j2 + 1] =
                    (pmap.values[i][j] + pmap.values[i][(j + 1) % pmap.values[i].len()]) / 2.0;
            }
        }
    }
    for i in 0..new_values.len() {
        // odd rows
        if i % 2 == 0 {
            continue;
        }
        for j in 0..new_values[i].len() {
            let j_up = (j as f32 * new_values[i - 1].len() as f32 / new_values[i].len() as f32)
                as usize
                % new_values[i - 1].len();
            if i < new_values.len() - 1 {
                let j_down = (j as f32 * new_values[i + 1].len() as f32
                    / new_values[i].len() as f32) as usize
                    % new_values[i + 1].len();
                new_values[i][j] = (new_values[i - 1][j_up] + new_values[i + 1][j_down]) / 2.0;
            } else {
                new_values[i][j] = new_values[i - 1][j_up];
            }
        }
    }
    pmap.height = (factor * pmap.height as f32) as usize;
    pmap.circunference = (factor * pmap.circunference as f32) as usize;
    pmap.values = new_values;
}

pub fn resize_i32<S: MapShape>(pmap: &mut PartialMap<S, i32>, factor: f32) {
    let mut new_values = S::new_vec(
        (factor * pmap.circunference as f32) as usize,
        (factor * pmap.height as f32) as usize,
    );

    for i in 0..pmap.values.len() {
        // even rows
        let i2 = ((i as f32 * factor) as usize).min(new_values.len() - 1);
        for j in 0..pmap.values[i].len() {
            let j2 = ((j as f32 * new_values[i2].len() as f32 / pmap.values[i].len() as f32)
                as usize)
                .min(new_values[i2].len() - 1);
            new_values[i2][j2] = pmap.values[i][j];
            if j2 + 1 < new_values[i2].len() {
                new_values[i2][j2 + 1] =
                    (pmap.values[i][j] + pmap.values[i][(j + 1) % pmap.values[i].len()]) / 2;
            }
        }
    }
    for i in 0..new_values.len() {
        // odd rows
        if i % 2 == 0 {
            continue;
        }
        for j in 0..new_values[i].len() {
            let j_up = (j as f32 * new_values[i - 1].len() as f32 / new_values[i].len() as f32)
                as usize
                % new_values[i - 1].len();
            if i < new_values.len() - 1 {
                let j_down = (j as f32 * new_values[i + 1].len() as f32
                    / new_values[i].len() as f32) as usize
                    % new_values[i + 1].len();
                new_values[i][j] = (new_values[i - 1][j_up] + new_values[i + 1][j_down]) / 2;
            } else {
                new_values[i][j] = new_values[i - 1][j_up];
            }
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

pub fn resize_rivers(rivers: &mut Vec<River>, factor: f32) {
    for river in rivers {
        // new_values.push(chains[i-1]);
        for point in river.iter_mut() {
            point.position[0] = (point.position[0] as f32 * factor) as usize;
            point.position[1] = (point.position[1] as f32 * factor) as usize;
        }
    }
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
        while (output_map.height.values.len() as f32)
            < (input_map.height.values.len() as f32 * self.factor)
        {
            let factor = (self.factor * (output_map.height.values.len() as f32)
                / (input_map.height.values.len() as f32))
                .min(2.0);
            if factor == 2.0 {
                resize_i32(&mut output_map.height, factor);
            } else {
                resize(&mut output_map.height, factor);
            }
        }

        resize(&mut output_map.tectonic_plates, self.factor);
        smooth_plates(&mut output_map.tectonic_plates);
        smooth_plates(&mut output_map.tectonic_plates);
        output_map.mountain_chains = resize_chains(&output_map.mountain_chains);
        output_map.andean_chains = resize_chains(&output_map.andean_chains);
        output_map.hymalayan_chains = resize_chains(&output_map.hymalayan_chains);
        output_map.trenches = resize_chains(&output_map.trenches);
        // output_map.trenches =
        resize_rivers(&mut output_map.rivers, self.factor);
        return output_map;
    }
}
