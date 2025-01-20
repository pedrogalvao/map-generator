use std::{f32::consts::PI, sync::Arc};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, resize::resize};

#[derive(Debug)]
pub struct DefinePressure {}

impl DefinePressure {
    pub fn new() -> Self {
        Self {}
    }

    fn define_month_pressure<S: MapShape>(
        &self,
        month: u32,
        total_months: u32,
        input_map: &CompleteMap<S>,
    ) -> PartialMap<S, i32> {
        let temperature_map = &input_map.temperature[month as usize];
        let mut pressure_map = PartialMap::<S, i32>::new(250, 125);
        for i in 0..pressure_map.values.len() {
            let mut expected_temperature = 0.0;
            let [latitude, _] = pressure_map.convert_coords(i, 0);
            for t in 0..30 {
                expected_temperature += temperature_map.get(latitude, t as f32 * 360.0 / 30.0);
            }
            expected_temperature /= 30.0;
            for j in 0..pressure_map.values[i].len() {
                let displacement = (month as f32 * 2.0 * PI / total_months as f32).cos() * 5.0;
                let [latitude, longitude] = pressure_map.convert_coords(i, j);
                let latitude_displaced = latitude - displacement;
                let mut temperature = temperature_map.get(latitude, longitude);
                let mut pressure = 0;
                if latitude_displaced.abs() > 15.0 && latitude_displaced.abs() < 45.0 {
                    pressure +=
                        (200.0 / ((30.0 - latitude_displaced.abs()).powf(2.0) / 2.0 + 10.0)) as i32;
                } else if latitude_displaced.abs() < 15.0 {
                    pressure -= (400.0 / (latitude_displaced.powf(2.0) / 2.0 + 10.0)) as i32;
                } else if latitude_displaced.abs() > 45.0 && latitude_displaced.abs() < 75.0 {
                    pressure -=
                        (200.0 / ((60.0 - latitude_displaced.abs()).powf(2.0) / 2.0 + 10.0)) as i32;
                }
                if latitude.abs() > 75.0 {
                    pressure += (1500.0 / ((90.0 - latitude.abs()).powf(2.0) / 2.0 + 10.0)) as i32;
                }
                // let mut expected_temperature;
                let height = input_map.height.get(latitude, longitude);
                // expected temperature based on latitude
                // expected_temperature = (50.0 * (latitude * PI / 180.0).cos() - 22.0) as i32;
                // expected_temperature +=
                //     (3.0 * (2.0 * latitude_displaced * PI / 180.0).cos() - 3.0) as i32;
                // expected_temperature += (55.0 * (latitude * PI / 180.0).cos() - 25.0) as i32;
                // expected_temperature +=
                //     (15.0 * (2.0 * latitude_displaced * PI / 180.0).cos() - 10.0) as i32;
                // expected_temperature /= 2;
                // expected_temperature = (55.0 * (latitude_displaced * PI / 180.0).cos() - 30.0) as i32;

                if height > 600 {
                    // undo height effect on temperature
                    temperature += (height as f32 - 600.0) / 150.0;
                }

                // pressure -= 5 * (temperature - expected_temperature);
                // pressure *= 2;
                // pressure_map.values[i][j] = pressure.max(-60).min(60);
                pressure =
                    (2.0 * (pressure as f32 - 5.0 * (temperature - expected_temperature))) as i32;
                pressure = (pressure as f32 * (90.0 - latitude_displaced.abs()) / 90.0) as i32;
                pressure_map.values[i][j] = pressure.max(-60).min(60);
            }
        }
        return pressure_map;
    }
}

impl<S: MapShape> PipelineStep<S> for DefinePressure {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.atm_pressure = vec![];
        for t in 0..input_map.temperature.len() {
            output_map.atm_pressure.push(self.define_month_pressure(
                t as u32,
                input_map.temperature.len() as u32,
                input_map,
            ));
        }
        for t in 0..output_map.temperature.len() {
            resize(&mut output_map.temperature[t as usize], 2.0);
        }
        output_map = SmoothPressure::new().apply(&output_map);
        for t in 0..output_map.temperature.len() {
            resize(&mut output_map.temperature[t as usize], 2.0);
        }
        output_map = SmoothPressure::new().apply(&output_map);
        return output_map;
    }
}

#[derive(Debug)]
pub struct SmoothPressure {
    pub pixel_distance: usize,
}

impl SmoothPressure {
    pub fn new() -> Self {
        Self { pixel_distance: 4 }
    }
}

impl SmoothPressure {
    fn process_pressure_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
        time_of_year: usize,
    ) -> i32 {
        let neighbors =
            input_map.atm_pressure[time_of_year].get_pixel_neighbours([x, y], self.pixel_distance);
        let mut sum = 0;
        let mut len = 0;
        for i in 0..neighbors.len() {
            for j in 0..neighbors[i].len() {
                sum += neighbors[i][j];
                len += 1;
            }
        }
        let mean = sum / len;
        return mean;
    }
}

impl<S: MapShape> PipelineStep<S> for SmoothPressure {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        for i in 0..input_map.atm_pressure.len() {
            let operator = move |x, y, arc_map| self.process_pressure_element(x, y, arc_map, i);
            // process elements in parallel with rayon
            output_map.atm_pressure[i].iterate_operator(input_map, operator)
        }
        return output_map;
    }
}
