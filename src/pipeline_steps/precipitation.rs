use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};
use std::{f32::consts::PI, fmt, sync::Arc};

use super::{pipeline_step::PipelineStep, resize::resize, smooth::smooth_pmap};

pub struct CalculatePrecipitation {
    humidity: f32,
}

impl fmt::Debug for CalculatePrecipitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CalculatePrecipitation").finish()
    }
}

fn calculate_itcz<S: MapShape>(complete_map: &CompleteMap<S>, div_number: usize) -> Vec<[f32; 2]> {
    let mut result = vec![];
    for i in -45..45 {
        let longitude = i as f32 * 4.0;
        let mut hottest_latitude = 0.0;
        let mut highest_temperature = 0.0;
        for j in -45..=45 {
            let latitude = j as f32 * 2.0;
            let temperature = complete_map.temperature[div_number].get(latitude, longitude);
            if temperature > highest_temperature {
                hottest_latitude = latitude;
                highest_temperature = temperature;
            }
        }
        result.push([hottest_latitude / 1.2, longitude]);
    }
    let mut result2 = vec![];
    for i in 0..result.len() {
        let longitude = result[i][1];
        let lat1 = result[i][0];
        let lat2 = result[(i as i32 - 1) as usize % result.len()][0];
        let lat3 = result[(i + 1) % result.len()][0];
        result2.push([(lat1 + lat2 + lat3) / 3.0, longitude]);
    }
    return result2;
}

impl CalculatePrecipitation {
    pub fn new(humidity: f32) -> Self {
        Self { humidity }
    }
    fn process_precipitation_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        complete_map: Arc<&CompleteMap<S>>,
        time_of_year: usize,
        itcz: &Vec<[f32; 2]>,
    ) -> i32 {
        let pmap_height = 250.0;
        let latitude = x as f32 * 180.0 / pmap_height - 90.0;

        let v_length = 1.0
            + (1.99
                * 2.0
                * ((pmap_height / 2.0).powf(2.0) - (x as f32 - pmap_height / 2.0).powf(2.0)).sqrt())
            .ceil() as f32;
        let longitude = y as f32 * 360.0 / v_length - 180.0;

        let mut itcz_distance = 9999.0;
        let itcz_interval = 360.0 / itcz.len() as f32;
        for i in 1..itcz.len() {
            let coords = itcz[i];
            if coords[1] >= longitude {
                let w1 = (coords[1] - longitude).abs() / itcz_interval;
                let w2 = (itcz[i - 1][1] - longitude).abs() / itcz_interval;
                let itcz_lat = w1 * itcz[i - 1][0] + w2 * itcz[i][0];
                itcz_distance = latitude - itcz_lat;
                // if latitude >= itcz_lat {
                //     itcz_distance = (latitude - itcz_lat) * (90.0 - itcz_lat.abs()) / 90.0;
                // } else {
                //     itcz_distance = (latitude - itcz_lat) * (itcz_lat.abs() - 90.0) / 90.0;
                // }
                // itcz_distance = itcz_lat + (90.0 - itcz_lat) * latitude / 90.0;
                // itcz_distance = (latitude + (90.0 - itcz_lat) * latitude / 90.0) / 2.0;
                break;
            }
        }
        if itcz_distance == 9999.0 {
            let coords = itcz[0];
            let w1 =
                (coords[1].rem_euclid(360.0) - longitude.rem_euclid(360.0)).abs() / itcz_interval;
            let w2 = (itcz[itcz.len() - 1][1].rem_euclid(360.0) - longitude.rem_euclid(360.0))
                .abs()
                / itcz_interval;
            let itcz_lat = w1 * itcz[itcz.len() - 1][0] + w2 * coords[0];
            itcz_distance = latitude - itcz_lat;
        }
        let mut max_dist;
        let mut precipitation = 0.0;
        let mut init_cumulative_multiplier = 0.8 * self.humidity;
        let mut n;
        if itcz_distance.abs() > 19.0 && itcz_distance.abs() < 33.0 {
            // init_cumulative_multiplier *= 0.7;
            n = 2;
            max_dist = 17;
        } else if itcz_distance.abs() > 15.0 && itcz_distance.abs() < 35.0 {
            // init_cumulative_multiplier *= 0.8;
            n = 3;
            max_dist = 17;
        } else if itcz_distance.abs() > 13.0 && itcz_distance.abs() < 37.0 {
            // init_cumulative_multiplier *= 0.9;
            n = 4;
            max_dist = 18;
        } else if itcz_distance.abs() > 12.0 && itcz_distance.abs() < 38.0 {
            n = 7;
            max_dist = 18;
        } else if itcz_distance.abs() > 10.0 && itcz_distance.abs() < 40.0 {
            n = 8;
            max_dist = 19;
        } else if itcz_distance.abs() < 10.0 {
            n = 9;
            init_cumulative_multiplier *= 1.0 + (10.0 - itcz_distance.abs()) / 8.0;
            max_dist = 22;
        } else {
            n = 7;
            max_dist = 28;
        }

        let mut angle_displacement = 0.0;
        if (itcz_distance > 15.0 && itcz_distance < 25.0)
            || (-itcz_distance > 35.0 && -itcz_distance < 45.0)
        {
            angle_displacement = PI / 18.0;
        } else if (itcz_distance > 35.0 && itcz_distance < 45.0)
            || (-itcz_distance > 15.0 && -itcz_distance < 25.0)
        {
            angle_displacement = -PI / 18.0;
        } else if itcz_distance.abs() > 55.0 {
            angle_displacement = PI;
            n = 7;
            init_cumulative_multiplier = (1.0 + (itcz_distance.abs() - 55.0) / 27.5).min(2.0);
            max_dist = 30;
            if itcz_distance.abs() > 65.0 {
                max_dist = 33;
            }
        } else if itcz_distance.abs() > 40.0 {
            angle_displacement = PI + PI / 18.0;
            n = 7;
            // init_cumulative_multiplier = (1.0 + (itcz_distance.abs() - 45.0) / 22.5).min(2.0);
        }

        for i in -n..=n {
            let mut cumulative_multiplier = init_cumulative_multiplier;
            let angle = i as f32 * PI / 9.0 + angle_displacement;

            let latitude2 = latitude + 2.0 * (angle as f32).sin();
            let longitude2 = longitude
                + 2.0 * (angle as f32).cos() / (latitude.abs() * PI / 180.0).min(PI / 2.0).cos();
            let height1 = complete_map.height.get(latitude, longitude);
            let height2 = complete_map.height.get(latitude2, longitude2);
            if height1 >= 0 && height2 >= 0 {
                // Effect of mountain shadows
                if height2 > height1 + 200 {
                    if height2 > height1 + 2000 {
                        continue;
                    } else if height2 > height1 + 1000 {
                        cumulative_multiplier *= 0.3;
                    } else {
                        cumulative_multiplier *= 0.6;
                    }
                } else if height1 > height2 + 200 {
                    if height1 > height2 + 1000 {
                        cumulative_multiplier *= 1.6;
                    } else {
                        cumulative_multiplier *= 1.3;
                    }
                }
            }
            for dist in 0..max_dist {
                let latitude2 = latitude + 2.0 * dist as f32 * (angle as f32).sin();
                let longitude2 = longitude
                    + 2.0 * dist as f32 * (angle as f32).cos()
                        / (latitude.abs() * PI / 180.0).min(PI / 2.0).cos();
                let height = complete_map.height.get(latitude2, longitude2);
                let temperature = complete_map.temperature[time_of_year].get(latitude2, longitude2);
                if height > 600 {
                    cumulative_multiplier *= (12000 - height).min(11400) as f32 / 11400.0;
                } else if height <= 0 && temperature >= 0.0 {
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
                if cumulative_multiplier < 0.2 {
                    break;
                }
                cumulative_multiplier *= 0.98;
            }
        }
        return precipitation as i32;
    }
}

impl<S: MapShape> PipelineStep<S> for CalculatePrecipitation {
    fn process_element(&self, _x: usize, _y: usize, _complete_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        output_map.precipitation = vec![];

        let half_length = (input_map.temperature.len() as f32 / 2.0).ceil() as usize;
        for i in 0..=half_length {
            let itcz = calculate_itcz(&input_map, i);
            let operator =
                move |x, y, arc_map| self.process_precipitation_element(x, y, arc_map, i, &itcz);
            let mut prec_map = PartialMap::new(500, 250);
            prec_map.iterate_operator(input_map, operator);
            let mut smooth_prec_map = smooth_pmap(&prec_map, 1);
            if output_map.height.circunference > smooth_prec_map.circunference {
                let factor =
                    output_map.height.circunference as f32 / smooth_prec_map.circunference as f32;
                resize(&mut smooth_prec_map, factor);
            }
            output_map.precipitation.push(smooth_prec_map);
        }

        // Remaining months mirror the previous ones
        for i in half_length + 1..input_map.temperature.len() {
            let prec_map = output_map.precipitation[input_map.temperature.len() - i].clone();
            output_map.precipitation.push(prec_map)
        }
        return output_map;
    }
}
