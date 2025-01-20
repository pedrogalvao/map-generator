use std::{fmt, sync::Arc};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

pub struct AddPlateGap {
    pub noise_intensity: f32,
    pub noise_frequency: f32,
    pub distance: usize,
    pub oceanic_plates: usize,
}

impl AddPlateGap {
    pub fn new() -> Self {
        AddPlateGap {
            noise_intensity: 5.0,
            noise_frequency: 10.0,
            distance: 16,
            oceanic_plates: 0,
        }
    }
}

fn scalar_prod(v1: &[f32; 2], v2: &[f32; 2]) -> f32 {
    return v1[0] * v2[0] + v1[1] * v2[1];
}

impl fmt::Debug for AddPlateGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddPlateGap")
            .field("noise_intensity", &self.noise_intensity)
            .field("noise_frequency", &self.noise_frequency)
            .field("distance", &self.distance)
            .finish()
    }
}

impl<S: MapShape> PipelineStep<S> for AddPlateGap {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        if input_map.height.values[x][y] < 0 {
            return input_map.height.values[x][y];
        }
        let plate1 = input_map.tectonic_plates.values[x][y];
        let mut value = input_map.height.values[x][y];
        for dist in 1..self.distance {
            'neighbors_loop: for row in input_map.tectonic_plates.get_pixel_neighbours([x, y], dist)
            {
                for plate2 in row {
                    if plate1 != plate2 {
                        let direction1 = input_map.tectonic_plates_directions[plate1]; //self.get_direction(plate1).clone();
                        let direction2 = input_map.tectonic_plates_directions[plate2];

                        if scalar_prod(&direction1, &direction2) > 0.2 {
                            value -= 15 + input_map.height.values[x][y] / 2;
                        }
                        break 'neighbors_loop;
                    }
                }
            }
        }
        return value;
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        for x in 0..output_map.height.values.len() {
            for y in 0..output_map.height.values[x].len() {
                // if output_map.height.values[x][y] < 0 {
                //     continue;
                // }
                let plate1 = output_map.tectonic_plates.values[x][y];

                if plate1 < self.oceanic_plates {
                    output_map.height.values[x][y] = output_map.height.values[x][y].min(-2000);
                    continue;
                }

                let direction_plate1 = output_map.tectonic_plates_directions[plate1];
                let x2 = (x as i32 - (self.distance as f32 * direction_plate1[0]) as i32)
                    .max(0)
                    .min((output_map.height.values.len() - 1).max(0) as i32)
                    as usize;
                let y2 = (y as i32 - (self.distance as f32 * direction_plate1[1]) as i32)
                    .rem_euclid((output_map.height.values[x2].len() - 1).max(1) as i32)
                    as usize;
                let plate2 = output_map.tectonic_plates.values[x2][y2];
                if plate2 != plate1 {
                    let direction_plate2 = output_map.tectonic_plates_directions[plate2];

                    if scalar_prod(&direction_plate1, &direction_plate2) < 0.0 {
                        output_map.height.values[x][y] = output_map.height.values[x][y].min(-2000);
                    }
                }
            }
        }
        return output_map;
    }
}
