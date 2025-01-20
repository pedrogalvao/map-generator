use std::sync::Arc;

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

fn get_lowest_neighbor<S: MapShape>(
    complete_map: &mut CompleteMap<S>,
    x: usize,
    y: usize,
) -> ([usize; 2], i32) {
    let neighbors_coords = complete_map.height.get_pixel_neighbours_coords([x, y], 1);
    let mut min_height = 999999;
    let mut lowest_neighbor = [x, y];
    for row in neighbors_coords {
        for [x2, y2] in row {
            let height2 = complete_map.height.values[x2][y2];
            if height2 < min_height {
                min_height = height2;
                lowest_neighbor = [x2, y2];
            }
        }
    }
    return (lowest_neighbor, min_height);
}

fn erode<S: MapShape>(
    complete_map: &mut CompleteMap<S>,
    height_vec: Vec<i32>,
    position_vec: Vec<[usize; 2]>,
) {
    if height_vec.len() < 2 {
        return;
    }
    let mut sediment = 0.0;
    for i in 1..height_vec.len() - 1 {
        let droplet_capacity = (height_vec[i - 1] - height_vec[i]) as f32 * 4.0;
        let slope = height_vec[i] - height_vec[i + 1];
        let [x, y] = position_vec[i];
        if slope as f32 * 0.2 + sediment >= droplet_capacity {
            complete_map.height.values[x][y] +=
                (slope as f32 * 0.2 + sediment - droplet_capacity) as i32;
            sediment = droplet_capacity;
        } else {
            sediment += slope as f32 * 0.2;
            complete_map.height.values[x][y] -= (slope as f32 * 0.2) as i32;
            // sediment = droplet_capacity;
        }
    }
    let [x, y] = position_vec.last().unwrap();
    complete_map.height.values[*x][*y] += sediment as i32;
}

fn droplet<S: MapShape>(complete_map: &mut CompleteMap<S>, x0: usize, y0: usize) {
    let ([mut x, mut y], mut curr_height) = get_lowest_neighbor(complete_map, x0, y0);
    if curr_height < -5 {
        return;
    }
    let mut height_vec = vec![];
    let mut position_vec = vec![];
    let mut prev_height = 99999;
    loop {
        ([x, y], curr_height) = get_lowest_neighbor(complete_map, x, y);
        position_vec.push([x, y]);
        height_vec.push(curr_height);
        if curr_height == prev_height || curr_height < -5 {
            break;
        }
        prev_height = curr_height;
    }
    erode(complete_map, height_vec, position_vec);
}

#[derive(Debug)]
pub struct HydraulicErosion {
    iterations: u32,
}

impl HydraulicErosion {
    pub fn new(iterations: u32) -> Self {
        Self { iterations }
    }
}

impl<S: MapShape> PipelineStep<S> for HydraulicErosion {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        for _ in 0..self.iterations {
            for r1 in 0..10 {
                for r2 in 0..10 {
                    for x in 3..output_map.height.values.len() - 3 {
                        for y in 3..output_map.height.values[x].len() - 3 {
                            if x % 10 == r1 && y % 10 == r2 {
                                droplet(&mut output_map, x, y);
                            }
                        }
                    }
                }
            }
        }
        return output_map;
    }
}
