use std::sync::Arc;

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;




fn add_water<S: MapShape>(water_map: &mut PartialMap<S, f32>) {
    for i in 0..water_map.values.len() {
        for j in 0..water_map.values[i].len() {
            water_map.values[i][j] += 5.0;
        }
    }
}

fn update_water_flow<S: MapShape>(water_flow: &mut PartialMap<S, [f32; 4]>, height_map: &PartialMap<S, i32>, water: &PartialMap<S, f32>) {
    for x in 1..water_flow.values.len()-1 {
        for y in 1..water_flow.values[x].len()-1 {
            let height = height_map.values[x][y];
            let neighbors_values = height_map.get_pixel_neighbours([x, y], 1);
            let up_pix = neighbors_values[0][1];
            let down_pix = neighbors_values[2][1];
            let left_pix = neighbors_values[1][0];
            let right_pix = neighbors_values[1][2];

            water_flow.values[x][y][0] = 0.2 * (height - up_pix).max(0) as f32;
            water_flow.values[x][y][1] = 0.2 * (height - right_pix).max(0) as f32;
            water_flow.values[x][y][2] = 0.2 * (height - down_pix).max(0) as f32;
            water_flow.values[x][y][3] = 0.2 * (height - left_pix).max(0) as f32;

            let scaling_var = (water.values[x][y] / (water_flow.values[x][y][0] + water_flow.values[x][y][1] +water_flow.values[x][y][2] +water_flow.values[x][y][3] )).min(1.0);
            
            for i in 0..4 {
                water_flow.values[x][y][i] *= scaling_var;
            }
        }
    }
}

fn move_water<S: MapShape>(water_flow: &PartialMap<S, [f32; 4]>, water: &mut PartialMap<S, f32>) {
    for x in 1..water_flow.values.len()-1 {
        for y in 1..water_flow.values[x].len()-1 {

            let neighbors_values = water_flow.get_pixel_neighbours([x, y], 1);
            let up_flow = neighbors_values[0][1][2];
            let down_flow = neighbors_values[2][1][0];
            let left_flow = neighbors_values[1][0][1];
            let right_flow = neighbors_values[1][2][3];

            for flow in [up_flow, down_flow, right_flow, left_flow] {
                water.values[x][y] -= flow;
            }

            for i in 0..4 {
                water.values[x][y] -= water_flow.values[x][y][i];
            }
        }
    }
}

fn move_sediment<S: MapShape>(suspended_sediment: &mut PartialMap<S, f32>, water_flow: &PartialMap<S, [f32; 4]>, water: &PartialMap<S, f32>) {

    for x in 1..water_flow.values.len()-1 {
        for y in 1..water_flow.values[x].len()-1 {

            let neighbors_values = water_flow.get_pixel_neighbours([x, y], 1);
            let up_flow = neighbors_values[0][1][2];
            let down_flow = neighbors_values[2][1][0];
            let left_flow = neighbors_values[1][0][1];
            let right_flow = neighbors_values[1][2][3];

            for i in 0..4 {
                suspended_sediment.values[x][y] -= water_flow.values[x][y][i] * suspended_sediment.values[x][y].max(0.0);
            }
            for flow in [up_flow, down_flow, right_flow, left_flow] {
                suspended_sediment.values[x][y] += flow * suspended_sediment.values[x][y];
            }
        }
    }
}

fn hydraulic_erosion<S: MapShape>(complete_map: &mut CompleteMap<S>, 
    iterations: usize, rain_amount: f32, evaporation_rate: f32, sediment_capacity: f32) {
    
    let mut water: PartialMap<S, f32> = PartialMap::<S, f32>::new(complete_map.height.circunference, complete_map.height.height);
    let mut water_flow = PartialMap::<S, [f32; 4]>::new(complete_map.height.circunference, complete_map.height.height);
    let mut suspended_sediment: PartialMap<S, f32> = PartialMap::<S, f32>::new(complete_map.height.circunference, complete_map.height.height);
    let mut deposited_sediment = PartialMap::<S, f32>::new(complete_map.height.circunference, complete_map.height.height);
    
    for _ in 0..10 {
        add_water(&mut water);
        update_water_flow(&mut water_flow, &complete_map.height, &water);
        move_water(&water_flow, &mut water);
        move_sediment(&mut suspended_sediment, &water_flow, &water);
    }

}

pub struct HydraulicErosion {}

impl HydraulicErosion {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S: MapShape> PipelineStep<S> for HydraulicErosion {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        hydraulic_erosion(&mut output_map, 10, 0.1, 0.05, 1.0);
        
        return output_map;
    }
}
