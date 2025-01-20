use std::{fmt, sync::Arc};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{pipeline_step::PipelineStep, util::CustomNoise};

pub struct AddMountains {
    noise: CustomNoise,
    intensity: f32,
}

impl AddMountains {
    pub fn new(seed: u32, frequency: f32, intensity: f32) -> Self {
        AddMountains {
            noise: CustomNoise::new(seed, frequency, intensity),
            intensity,
        }
    }
}

fn scalar_prod(v1: &[f32; 2], v2: &[f32; 2]) -> f32 {
    return v1[0] * v2[0] + v1[1] * v2[1];
}

impl fmt::Debug for AddMountains {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddMountains3")
            .field("intensity", &self.intensity)
            .finish()
    }
}

impl<S: MapShape> PipelineStep<S> for AddMountains {
    fn process_element(&self, x: usize, y: usize, input_map: Arc<&CompleteMap<S>>) -> i32 {
        let plate1 = input_map.tectonic_plates.values[x][y];
        let mut height = input_map.height.values[x][y];
        let direction1 = input_map.tectonic_plates_directions[plate1];

        let [lat, lon] = input_map.tectonic_plates.convert_coords(x, y);
        let mut k = 0;
        for i in 1..10 {
            let [lat2, lon2] = [
                lat + 0.6 * i as f32 * direction1[0],
                lon + 0.6 * i as f32 * direction1[1],
            ];
            let plate2 = input_map.tectonic_plates.get(lat2, lon2);
            let direction2 = input_map.tectonic_plates_directions[plate2];

            let pixel_direction = [lat2 - lat, lon2 - lon];

            // let pixel_direction_norm = ((x as f32 - x2 as f32).powf(2.0)
            //     + (y as f32 - y2 as f32).powf(2.0))
            // .powf(0.5);
            // let pixel_direction_normalized = [
            //     pixel_direction[0] / pixel_direction_norm,
            //     pixel_direction[1] / pixel_direction_norm,
            // ];
            let collision = scalar_prod(&direction1, &pixel_direction)
                - scalar_prod(&pixel_direction, &direction2);
            if plate1 != plate2 && collision > 0.5 {
                let height2 = input_map.height.get(lat2, lon2);
                let noise_value =
                    2.0 * (self.intensity - self.noise.get_spheric_f32::<S, i32>(lat2, lon2).abs());
                if height2 > -300 {
                    height += (noise_value * (height2 + 400) as f32 / i as f32) as i32;
                    k += 1;
                } else {
                    height += (noise_value * 100.0 / i as f32) as i32;
                    k += 2;
                }
            }
            if k > 6 {
                break;
            }
        }
        return height;
    }
}
