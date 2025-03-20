use std::{f32::consts::PI, sync::Arc};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, resize::resize};

#[derive(Debug)]
pub struct Temperature {
    year_divisions: u32,
    axial_tilt: f32,
    // eccentricity: f32,
    // perihelium: f32,
}

impl Temperature {
    pub fn default() -> Self {
        Self {
            year_divisions: 12,
            axial_tilt: 23.44,
        }
    }

    pub fn new(axial_tilt: f32) -> Self {
        Self {
            year_divisions: 12,
            axial_tilt,
        }
    }

    fn define_month_temperature<S: MapShape>(
        &self,
        month: u32,
        total_months: u32,
        input_map: &CompleteMap<S>,
    ) -> PartialMap<S, f32> {
        let mut temperature_map = input_map.temperature[month as usize].clone();
        for i in 0..temperature_map.values.len() {
            let [latitude, _] = temperature_map.convert_coords(i, 0);
            let incidence_angle = latitude
                + self.axial_tilt * ((month as f32 / total_months as f32) * 2.0 * PI).cos();
            let day_duration = (-(latitude * PI / 180.0).tan()
                * ((90.0 - incidence_angle) * PI / 180.0).tan())
            .acos();
            for j in 0..temperature_map.values[i].len() {
                let [latitude, longitude] = temperature_map.convert_coords(i, j);
                let height = input_map.height.get(latitude, longitude);
                let albedo = 0.4;
                let absorbed_radiation =
                    albedo * day_duration * (incidence_angle * PI / 180.0).cos().max(0.0);
                todo!();
                // let mut temperature;
                // temperature =
                // temperature_map.values[i][j] = temperature;
            }
        }
        return temperature_map;
    }

    fn decrease_mountain_temperature<S: MapShape>(
        &self,
        month: u32,
        input_map: &CompleteMap<S>,
    ) -> PartialMap<S, f32> {
        let mut temperature_map = input_map.temperature[month as usize].clone();
        for i in 0..temperature_map.values.len() {
            for j in 0..temperature_map.values[i].len() {
                let [latitude, longitude] = temperature_map.convert_coords(i, j);
                let mut temperature = temperature_map.values[i][j];
                let height = input_map.height.get(latitude, longitude);
                temperature -= height.max(0) as f32 / 154.0;
                temperature_map.values[i][j] = temperature;
            }
        }
        return temperature_map;
    }
}

#[derive(Debug)]
pub struct SmoothTemperature {
    pub pixel_distance: usize,
}

impl SmoothTemperature {
    pub fn new() -> Self {
        Self { pixel_distance: 4 }
    }
}

impl SmoothTemperature {
    fn process_temperature_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
        time_of_year: usize,
    ) -> f32 {
        let neighbors =
            input_map.temperature[time_of_year].get_pixel_neighbours([x, y], self.pixel_distance);
        let mut sum = 0.0;
        let mut len = 0;
        for i in 0..neighbors.len() {
            for j in 0..neighbors[i].len() {
                sum += neighbors[i][j];
                len += 1;
            }
        }
        let mean = sum / len as f32;
        return mean;
    }
}

impl<S: MapShape> PipelineStep<S> for SmoothTemperature {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        for i in 0..input_map.temperature.len() {
            // process elements in parallel with rayon
            output_map.temperature[i]
                .values
                .par_iter_mut()
                .enumerate()
                .for_each(|(x, inner_vec)| {
                    inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
                        *num = self.process_temperature_element(x, y, Arc::new(input_map), i);
                    });
                });
        }
        return output_map;
    }
}

impl<S: MapShape> PipelineStep<S> for Temperature {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.temperature = vec![];
        for t in 0..=self.year_divisions / 2 {
            output_map.temperature.push(self.define_month_temperature(
                t as u32,
                self.year_divisions as u32,
                input_map,
            ));
        }
        output_map = SmoothTemperature::new().apply(&output_map);
        for t in 0..output_map.temperature.len() {
            resize(&mut output_map.temperature[t as usize], 2.0);
        }
        output_map = SmoothTemperature::new().apply(&output_map);
        for t in 0..output_map.temperature.len() {
            resize(&mut output_map.temperature[t as usize], 2.0);
        }
        output_map = SmoothTemperature::new().apply(&output_map);
        for t in 0..output_map.temperature.len() {
            resize(&mut output_map.temperature[t as usize], 2.0);
        }
        for t in 0..output_map.temperature.len() {
            output_map.temperature[t as usize] =
                self.decrease_mountain_temperature(t as u32, &output_map);
        }
        let mut smooth_step = SmoothTemperature::new();
        smooth_step.pixel_distance = 1;
        output_map = smooth_step.apply(&output_map);

        // Remaining months mirror the previous ones
        for t in self.year_divisions / 2 + 1..self.year_divisions {
            let temperature_map =
                output_map.temperature[(self.year_divisions - t) as usize].clone();
            output_map.temperature.push(temperature_map);
        }

        return output_map;
    }
}
