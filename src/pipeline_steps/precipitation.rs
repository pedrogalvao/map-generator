use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};
use std::{f32::consts::PI, fmt, sync::Arc};

use super::{pipeline_step::PipelineStep, resize::resize};

pub struct SmoothPrecipitation {
    pub pixel_distance: usize,
}

impl SmoothPrecipitation {
    pub fn new() -> Self {
        Self { pixel_distance: 1 }
    }
}

impl fmt::Debug for SmoothPrecipitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SmoothPrecipitation").finish()
    }
}

impl SmoothPrecipitation {
    fn process_precipitation_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: Arc<&CompleteMap<S>>,
        time_of_year: usize,
    ) -> i32 {
        let neighbors =
            input_map.precipitation[time_of_year].get_pixel_neighbours([x, y], self.pixel_distance);
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

impl<S: MapShape> PipelineStep<S> for SmoothPrecipitation {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        for i in 0..input_map.precipitation.len() {
            // process elements in parallel with rayon
            output_map.precipitation[i]
                .values
                .par_iter_mut()
                .enumerate()
                .for_each(|(x, inner_vec)| {
                    inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
                        *num = self.process_precipitation_element(x, y, Arc::new(input_map), i);
                    });
                });
        }
        return output_map;
    }
}

pub struct CalculatePrecipitation {}

impl fmt::Debug for CalculatePrecipitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CalculatePrecipitation").finish()
    }
}

fn precipitation_from_direction<S: MapShape>(
    complete_map: &Arc<&CompleteMap<S>>,
    latitude: f32,
    longitude: f32,
    direction: [f32; 2],
    div_number: usize,
) -> f32 {
    let mut cumulative_multiplier = 1.0;
    let mut precipitation = 0.0;
    let mut curr_latitude = latitude;
    let mut curr_longitude = longitude;
    let mut curr_direction = direction;
    for _ in 1..150 {
        curr_latitude += curr_direction[0] as f32 / 4.0;
        if curr_latitude.abs() >= 90.0 {
            break;
        }
        curr_longitude += direction[1] as f32 / 4.0;
        let h = complete_map.height.get(curr_latitude, curr_longitude);
        cumulative_multiplier *= 0.995;
        let temperature = complete_map.temperature[div_number].get(latitude, longitude);
        if h > 600 {
            cumulative_multiplier *= (17000 - h).min(16400) as f32 / 16400.0;
        } else if h <= 0 && temperature >= 0.0 {
            if temperature >= 15.0 {
                precipitation += cumulative_multiplier * (latitude * PI / 180.0).cos();
            } else {
                precipitation += (temperature as f32 / 15.0)
                    * cumulative_multiplier
                    * (latitude * PI / 180.0).cos();
            }
        } else {
            precipitation += 0.1 * cumulative_multiplier * (latitude * PI / 180.0).cos();
        }
        let curr_direction_int = complete_map.winds[div_number].get(curr_latitude, curr_longitude);
        curr_direction = [curr_direction_int[0] as f32, curr_direction_int[1] as f32];
    }
    return precipitation;
}

impl CalculatePrecipitation {
    fn process_precipitation_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        complete_map: Arc<&CompleteMap<S>>,
        time_of_year: usize,
    ) -> i32 {
        let mut total_precipitation = 0.0;
        // let [latitude, longitude] = PartialMap::<S, i32>::new(1000, 500).convert_coords(x, y);
        let latitude = x as f32 * 180.0 / 500.0 - 90.0;
        let longitude = y as f32 * 360.0 / 1000.0 - 180.0;
        let [wx, wy, _wz] = complete_map.winds[time_of_year].get(latitude, longitude);
        let [wx, wy] = [-wx as f32, -wy as f32];
        total_precipitation += precipitation_from_direction(
            &complete_map,
            latitude,
            longitude,
            [1.0 * wx, 1.0 * wy],
            time_of_year,
        ) * 1.6;
        let mut height_multiplier_factor = 1.0;
        let height = complete_map.height.get(latitude, longitude);
        if height > 400 {
            height_multiplier_factor +=
                (5.0 * (height - 400) as f32 / (2000 + height) as f32).max(0.0);
        }
        let pressure = complete_map.atm_pressure[time_of_year].get(latitude, longitude);
        let pressure_multiplier_factor = 1.0 + (-pressure as f32 / 20.0);
        return (total_precipitation * height_multiplier_factor * pressure_multiplier_factor)
            as i32;
    }
}

impl<S: MapShape> PipelineStep<S> for CalculatePrecipitation {
    fn process_element(&self, _x: usize, _y: usize, _complete_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        for i in 0..input_map.winds.len() {
            let operator =
                move |x, y, arc_map| self.process_precipitation_element(x, y, arc_map, i);
            let mut prec_map = PartialMap::new(1000, 500);
            prec_map.iterate_operator(input_map, operator);
            if output_map.height.circunference > prec_map.circunference {
                let factor = output_map.height.circunference as f32 / prec_map.circunference as f32;
                resize(&mut prec_map, factor);
            }
            output_map.precipitation.push(prec_map);
        }
        let output_map = SmoothPrecipitation::new().apply(&output_map);
        let output_map = SmoothPrecipitation::new().apply(&output_map);
        return output_map;
    }
}
