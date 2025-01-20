use std::sync::Arc;

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct Ice {}

impl Ice {
    fn process_ice_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
        month: usize,
    ) -> i32 {
        let [latitude, longitude] = input_map.ice_height[month].convert_coords(x, y);
        let mut max_temperature = -99999.0;
        let temperature = input_map.temperature[month].get(latitude, longitude);
        for temperature_map in &input_map.temperature {
            let t = temperature_map.get(latitude, longitude);
            if t > max_temperature {
                max_temperature = t;
            }
        }
        let mut ice_height = 0;
        let height = input_map.height.get(latitude, longitude);
        if height > 0 && max_temperature < 0.0 {
            ice_height += 10;
        } else if max_temperature < -2.0 {
            ice_height += 10;
        }
        if temperature < 0.0 {
            ice_height -= temperature as i32;
        }
        return ice_height;
    }

    fn _update_temperature<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
        month: usize,
    ) -> i32 {
        let [latitude, longitude] = input_map.temperature[month].convert_coords(x, y);
        let _temperature = input_map.temperature[month].get(latitude, longitude);
        let ice_height = input_map.ice_height[month].get(latitude, longitude);
        if ice_height == 0 {
            todo!()
        }
        return ice_height;
    }
}

impl<S: MapShape> PipelineStep<S> for Ice {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.ice_height = vec![];

        for t in 0..input_map.temperature.len() {
            let mut ice_height =
                PartialMap::new(input_map.height.circunference, input_map.height.height);
            let operator = move |x, y, arc_map| self.process_ice_element(x, y, arc_map, t);
            ice_height.iterate_operator(input_map, operator);
            output_map.ice_height.push(ice_height);
        }
        return output_map;
    }
}
