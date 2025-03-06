use std::{f32::consts::PI, sync::Arc};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, resize::resize};

#[derive(Debug)]
pub struct TemperatureFromContinentality {
    year_divisions: u32,
    equator_temperature: f32,
    pole_temperature: f32,
}

impl TemperatureFromContinentality {
    pub fn default() -> Self {
        Self {
            year_divisions: 12,
            equator_temperature: 27.0,
            pole_temperature: -35.0,
        }
    }

    pub fn new(equator_temperature: f32, pole_temperature: f32) -> Self {
        Self {
            year_divisions: 12,
            equator_temperature,
            pole_temperature,
        }
    }

    fn define_month_temperature<S: MapShape>(
        &self,
        month: u32,
        total_months: u32,
        input_map: &CompleteMap<S>,
    ) -> PartialMap<S, f32> {
        let mut temperature_map = PartialMap::<S, f32>::new(125, 62);
        for i in 0..temperature_map.values.len() {
            for j in 0..temperature_map.values[i].len() {
                let [latitude, longitude] = temperature_map.convert_coords(i, j);
                let mut temperature;
                let height = input_map.height.get(latitude, longitude);
                // temperature = (57.0 * (latitude * PI / 180.0).cos() - 30.0) as i32;
                // temperature = (60.0 * (1.0 - (latitude.abs() / 90.0).powf(3.5)) - 33.0) as i32;
                temperature = (self.equator_temperature - self.pole_temperature)
                    * (1.0 - (latitude.abs() / 90.0).powf(3.0))
                    + self.pole_temperature;
                let continentality = input_map.continentality.get(latitude, longitude);
                let variation =
                    continentality as f32 * ((month as f32 / total_months as f32) * 2.0 * PI).cos();
                if latitude < 0.0 {
                    temperature -= variation;
                } else {
                    temperature += variation;
                }
                if height <= 0 {
                    let height_east = input_map.height.get(latitude, longitude + 5.0);
                    let height_west = input_map.height.get(latitude, longitude - 5.0);
                    if height_east > 0 {
                        temperature -= 2.0;
                    }
                    if height_west > 0 {
                        temperature += 2.0;
                    }
                    let height_east = input_map.height.get(latitude, longitude + 10.0);
                    let height_west = input_map.height.get(latitude, longitude - 10.0);
                    if height_east > 0 {
                        temperature -= 2.0;
                    }
                    if height_west > 0 {
                        if latitude.abs() < 60.0 {
                            temperature += 2.0;
                        } else if latitude.abs() > 70.0 {
                            temperature -= 2.0;
                        }
                    }
                }
                if height >= 0 && latitude.abs() >= 50.0 {
                    temperature -= ((90.0 - latitude.abs()) / 40.0) * 5.0;
                } else if height >= 0 && latitude.abs() < 50.0 {
                    temperature += ((50.0 - latitude.abs()) / 50.0) * 4.0;
                }
                temperature_map.values[i][j] = temperature;
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

impl<S: MapShape> PipelineStep<S> for TemperatureFromContinentality {
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
