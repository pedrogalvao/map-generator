use std::sync::Arc;

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

fn hydraulic_erosion<S: MapShape>(complete_map: &mut CompleteMap<S>, 
    iterations: usize, rain_amount: f32, evaporation_rate: f32, sediment_capacity: f32) {

    let heightmap = &mut complete_map.height;
    let mut sediment_map = PartialMap::<S, f32>::new(heightmap.circunference, heightmap.height);
    let mut water_map = PartialMap::<S, f32>::new(heightmap.circunference, heightmap.height);

    for _ in 0..iterations {
        // Step 1: Simulate Rainfall
        for x in 0..water_map.values.len() {
            for y in 0..water_map.values[x].len() {
                water_map.values[x][y] += rain_amount;
            }
        }

        // Step 2: Calculate Water Flow
        let mut water_flow = vec![vec![(0.0, 0.0); heightmap.circunference*2]; heightmap.circunference*2]; // (flow to right, flow down)

        for x in 1..heightmap.values.len()-1 {
            for y in 1..heightmap.values[x].len()-1 {
                let neighbors_coords = heightmap.get_pixel_neighbours_coords([x, y], 1);
                let [x2, y2] = neighbors_coords[2][1];
                if x2 < heightmap.values.len() && y2 < heightmap.values[x2].len() {
                    let height_diff = (heightmap.values[x][y] as f32 - heightmap.values[x2][y2] as f32) + (water_map.values[x][y] - water_map.values[x2][y2]);
                    if height_diff > 0.0 {
                        water_flow[x][y].1 += (height_diff / 2.0).max(10.0);
                        water_flow[x2][y2].1 -= (height_diff / 2.0).max(10.0);
                    }
                }
                let neighbors_coords = heightmap.get_pixel_neighbours_coords([x, y], 1);
                let [x2, y2] = neighbors_coords[1][2];
                if y2 < heightmap.values[x2].len() {
                    let height_diff = (heightmap.values[x][y] as f32 - heightmap.values[x2][y2] as f32) + (water_map.values[x][y] - water_map.values[x2][y2]);
                    if height_diff > 0.0 {
                        water_flow[x][y].0 += (height_diff / 2.0).max(10.0);
                        water_flow[x2][y2].0 -= (height_diff / 2.0).max(10.0);
                    }
                }
            }
        }

        // Step 3: Erode the Terrain
        for x in 0..heightmap.values.len() {
            for y in 0..heightmap.values[x].len() {
                let total_flow = water_flow[x][y].0.abs() + water_flow[x][y].1.abs();
                if total_flow > 0.0 {
                    let eroded_sediment = (total_flow * 0.1).min(sediment_capacity - sediment_map.values[x][y]);
                    heightmap.values[x][y] -= eroded_sediment as i32;
                    sediment_map.values[x][y] += eroded_sediment;
                }
            }
        }

        // Step 4: Transport Sediment
        for x in (1..heightmap.values.len()-1).rev() {
            for y in (1..heightmap.values[x].len()-1).rev() {
                let neighbors_coords = heightmap.get_pixel_neighbours_coords([x, y], 1);
                let [x2, y2] = neighbors_coords[2][1];
                if water_flow[x][y].1 > 0.0 && x2 < heightmap.values.len() {
                    sediment_map.values[x2][y2] += sediment_map.values[x][y] * water_flow[x][y].1;
                    // sediment_map.values[x][y] -= sediment_map.values[x][y] * water_flow[x][y].1;
                    sediment_map.values[x][y] *= 1.0 - water_flow[x][y].1;
                }
                let neighbors_coords = heightmap.get_pixel_neighbours_coords([x, y], 1);
                let [x2, y2] = neighbors_coords[1][2];
                if water_flow[x][y].0 > 0.0 && y2 < heightmap.values[x].len() {
                    sediment_map.values[x2][y2] += sediment_map.values[x][y] * water_flow[x][y].0;
                    // sediment_map.values[x][y] -= sediment_map.values[x][y] * water_flow[x][y].0;
                    sediment_map.values[x][y] *= 1.0 - water_flow[x][y].0;
                }
            }
        }

        // Step 5: Deposit Sediment
        for x in 0..heightmap.values.len() {
            for y in 0..heightmap.values[x].len() {
                // if water_map.values[x][y] > 0.0 {
                    let deposit_amount = (sediment_map.values[x][y] * 0.1).min(water_map.values[x][y]);
                    // let deposit_amount = sediment_map.values[x][y];
                    heightmap.values[x][y] += deposit_amount as i32;
                    sediment_map.values[x][y] -= deposit_amount;
                // }
            }
        }

        // Step 6: Evaporation and Update
        for x in 0..heightmap.values.len() {
            for y in 0..heightmap.values[x].len() {
                water_map.values[x][y] *= 1.0 - evaporation_rate;
            }
        }
    }
}

pub struct HidraulicErosion {}

impl HidraulicErosion {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S: MapShape> PipelineStep<S> for HidraulicErosion {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        hydraulic_erosion(&mut output_map, 10, 0.1, 0.05, 1.0);
        return output_map;
    }
}
