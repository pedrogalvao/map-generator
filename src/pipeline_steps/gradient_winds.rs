use std::sync::Arc;

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::rotate_vector};

#[derive(Debug)]
pub struct DefineWindsGradient {
    year_divisions: u32,
}

impl DefineWindsGradient {
    pub fn new() -> Self {
        Self { year_divisions: 24 }
    }

    fn process_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
        month: usize,
    ) -> [f32; 3] {
        let pressure_map = &input_map.atm_pressure[month];
        let neighbours = pressure_map.get_pixel_neighbours([x, y], 1);
        let mut value: [f32; 3] = [0.0, 0.0, 0.0];
        if neighbours.len() <= 1 {
            return value;
        } else if neighbours[1].len() <= 1 || neighbours[0].len() <= 1 {
            return value;
        } else if pressure_map.values[x].len() <= 1 {
            return value;
        }
        value[0] = (pressure_map.values[x][y] - neighbours[0][1]) as f32;
        value[1] = (pressure_map.values[x][y] - neighbours[1][0]) as f32;
        value[2] = pressure_map.values[x][y] as f32;
        let rotated_vector;
        let [lat, _] = pressure_map.convert_coords(x, y);
        if lat > 15.0 {
            rotated_vector = rotate_vector([value[0], value[1]], -50.0);
        } else if lat < -15.0 {
            rotated_vector = rotate_vector([value[0], value[1]], 50.0);
        } else {
            if value[0] > 0.0 {
                rotated_vector = rotate_vector([value[0], value[1]], -50.0);
            } else {
                rotated_vector = rotate_vector([value[0], value[1]], 50.0);
            }
        }
        let vec_module = (rotated_vector[0].powf(2.0) + rotated_vector[1].powf(2.0)).powf(0.5);
        value[0] = rotated_vector[0] / vec_module;
        value[1] = rotated_vector[1] / vec_module;
        return value;
    }
}

impl<S: MapShape> PipelineStep<S> for DefineWindsGradient {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.winds = vec![];
        for t in 0..self.year_divisions {
            let mut wind_map = PartialMap::new(
                output_map.atm_pressure[t as usize].circunference,
                output_map.atm_pressure[t as usize].height,
            );
            let operator = |x, y, arc_map| self.process_element(x, y, arc_map, t as usize);
            wind_map.iterate_operator(input_map, operator);
            output_map.winds.push(wind_map);
        }
        return output_map;
    }
}
