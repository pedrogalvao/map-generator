use std::sync::Arc;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct DefineKoppenClimate {}

#[derive(Clone, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Climate {
    #[default]
    Ocean,
    Tropical,
    Monsoon,
    Savanah,
    HotDesert,
    ColdDesert,
    HotSemiarid,
    ColdSemiarid,
    HotMediterranean,
    WarmMediterranean,
    ColdMediterranean,
    HumidSubtropical,
    SubtropicalMonsoon,
    Oceanic,
    SubarcticOceanic,
    HotHumidContinental,
    HumidContinental,
    MonsoonContinental,
    HotMediterraneanContinental,
    ColdMediterraneanContinental,
    Subarctic,
    SevereSubarctic,
    Tundra,
    Glaciar,
    Undefined,
}

fn get_summer_driest_month<S: MapShape>(
    latitude: f32,
    longitude: f32,
    complete_map: &Arc<&CompleteMap<S>>,
) -> i32 {
    let mut min_precipitation = 9999;
    for i in 0..complete_map.precipitation.len() {
        let prec_map = &complete_map.precipitation[i];
        let precipitation = prec_map.get(latitude, longitude);
        if (latitude < 0.0)
            != (i < complete_map.precipitation.len() / 4
                || i >= 3 * complete_map.precipitation.len() / 4)
        {
            // summer
            if min_precipitation > precipitation {
                min_precipitation = precipitation;
            }
        }
    }
    min_precipitation
}

fn get_summer_wettest_month<S: MapShape>(
    latitude: f32,
    longitude: f32,
    complete_map: &Arc<&CompleteMap<S>>,
) -> i32 {
    let mut max_precipitation = -9999;
    for i in 0..complete_map.precipitation.len() {
        let prec_map = &complete_map.precipitation[i];
        let precipitation = prec_map.get(latitude, longitude);
        if (latitude < 0.0)
            != (i < complete_map.precipitation.len() / 4
                || i >= 3 * complete_map.precipitation.len() / 4)
        {
            // summer
            if max_precipitation < precipitation {
                max_precipitation = precipitation;
            }
        }
    }
    max_precipitation
}

fn get_winter_wettest_month<S: MapShape>(
    latitude: f32,
    longitude: f32,
    complete_map: &Arc<&CompleteMap<S>>,
) -> i32 {
    let mut max_precipitation = -9999;
    for i in 0..complete_map.precipitation.len() {
        let prec_map = &complete_map.precipitation[i];
        let precipitation = prec_map.get(latitude, longitude);
        if (latitude >= 0.0)
            != (i < complete_map.precipitation.len() / 4
                || i >= 3 * complete_map.precipitation.len() / 4)
        {
            // winter
            if max_precipitation < precipitation {
                max_precipitation = precipitation;
            }
        }
    }
    max_precipitation
}

fn get_winter_dryest_month<S: MapShape>(
    latitude: f32,
    longitude: f32,
    complete_map: &Arc<&CompleteMap<S>>,
) -> i32 {
    let mut min_precipitation = 9999;
    for i in 0..complete_map.precipitation.len() {
        let prec_map = &complete_map.precipitation[i];
        let precipitation = prec_map.get(latitude, longitude);
        if (latitude >= 0.0)
            != (i < complete_map.precipitation.len() / 4
                || i >= 3 * complete_map.precipitation.len() / 4)
        {
            // winter
            if min_precipitation > precipitation {
                min_precipitation = precipitation;
            }
        }
    }
    min_precipitation
}

enum SWF {
    S,
    W,
    F,
}

fn get_swf<S: MapShape>(latitude: f32, longitude: f32, complete_map: &Arc<&CompleteMap<S>>) -> SWF {
    let summer_dry_month = get_summer_driest_month(latitude, longitude, &complete_map);
    let winter_wet_month = get_winter_wettest_month(latitude, longitude, &complete_map);
    if summer_dry_month < 30 && 3 * summer_dry_month < winter_wet_month {
        return SWF::S;
    }
    let summer_wet_month = get_summer_wettest_month(latitude, longitude, &complete_map);
    let winter_dry_month = get_winter_dryest_month(latitude, longitude, &complete_map);
    if winter_dry_month * 10 < summer_wet_month {
        return SWF::W;
    } else {
        return SWF::F;
    }
}

fn get_min_temperature<S: MapShape>(
    latitude: f32,
    longitude: f32,
    complete_map: &Arc<&CompleteMap<S>>,
) -> f32 {
    let mut min_temperature = 999.0;
    for temp_map in &complete_map.temperature {
        let temperature = temp_map.get(latitude, longitude);
        if min_temperature > temperature {
            min_temperature = temperature;
        }
    }
    min_temperature
}

fn get_max_temperature<S: MapShape>(
    latitude: f32,
    longitude: f32,
    complete_map: &Arc<&CompleteMap<S>>,
) -> f32 {
    let mut max_temperature = -999.0;
    for temp_map in &complete_map.temperature {
        let temperature = temp_map.get(latitude, longitude);
        if max_temperature < temperature {
            max_temperature = temperature;
        }
    }
    max_temperature
}

enum ABCD {
    A,
    B,
    C,
    D,
}

fn get_abcd<S: MapShape>(
    latitude: f32,
    longitude: f32,
    complete_map: &Arc<&CompleteMap<S>>,
) -> ABCD {
    let max_temperature = get_max_temperature(latitude, longitude, complete_map);
    if max_temperature > 22.0 {
        return ABCD::A;
    } else {
        let mut n_months_above_10 = 0;
        for i in 0..complete_map.temperature.len() {
            if complete_map.temperature[i].get(latitude, longitude) >= 10.0 {
                n_months_above_10 += 1;
            }
        }
        if n_months_above_10 >= complete_map.temperature.len() / 3 {
            return ABCD::B;
        } else if n_months_above_10 >= complete_map.temperature.len() / 12 {
            return ABCD::C;
        } else {
            return ABCD::D;
        }
    }
}

impl DefineKoppenClimate {
    fn process_climate_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        complete_map: Arc<&CompleteMap<S>>,
    ) -> Climate {
        if x >= complete_map.climate.values.len() {
            return Climate::Ocean;
        }
        let [latitude, longitude] = complete_map.climate.convert_coords(x, y);

        let mut avg_temperature = 0.0;
        let mut max_temperature = -999.0;
        let mut min_temperature = 999.0;
        for temp_map in &complete_map.temperature {
            let temperature = temp_map.get(latitude, longitude);
            if max_temperature < temperature {
                max_temperature = temperature;
            }
            if min_temperature > temperature {
                min_temperature = temperature;
            }
            avg_temperature += temperature;
        }

        let mut max_precipitation = -999;
        let mut min_precipitation = 999;
        let mut summer_precipitation = 0;
        let mut winter_precipitation = 0;
        for i in 0..complete_map.precipitation.len() {
            let prec_map = &complete_map.precipitation[i];
            let precipitation = prec_map.get(latitude, longitude);
            if (latitude < 0.0)
                != (i < complete_map.precipitation.len() / 4
                    || i >= 3 * complete_map.precipitation.len() / 4)
            {
                winter_precipitation += precipitation;
            } else {
                summer_precipitation += precipitation;
            }
            if max_precipitation < precipitation {
                max_precipitation = precipitation;
            }
            if min_precipitation > precipitation {
                min_precipitation = precipitation;
            }
        }
        avg_temperature /= complete_map.temperature.len() as f32;
        let height = complete_map.height.get(latitude, longitude);
        let annual_precipitation = complete_map.annual_precipitation.get(latitude, longitude);
        if height <= 0 {
            if max_temperature < -1.0 {
                return Climate::Glaciar;
            }
            return Climate::Ocean;
        } else if max_temperature < 10.0 {
            // E - Polar
            if max_temperature > 1.0 {
                // ET
                return Climate::Tundra;
            } else {
                // EF
                return Climate::Glaciar;
            }
        } else if (summer_precipitation as f32 >= 0.7 * annual_precipitation as f32
            && annual_precipitation <= (20.0 * avg_temperature + 280.0) as i32)
            || (winter_precipitation as f32 >= 0.7 * annual_precipitation as f32
                && annual_precipitation < (20.0 * avg_temperature) as i32)
            || annual_precipitation < (20.0 * avg_temperature + 140.0) as i32
        {
            // B - Arid
            // 70% or more of annual precipitation
            //    falls in the summer half of the year
            //    and r less than 20t + 280,
            // or 70% or more of annual precipitation
            //    falls in the winter half of the year
            //    and r less than 20t,
            // or neither half of the year
            //    has 70% or more of annual precipitation
            //    and r less than 20t + 1403
            if (summer_precipitation as f32 >= 0.7 * annual_precipitation as f32
                && annual_precipitation <= (10.0 * avg_temperature + 140.0) as i32)
                || (winter_precipitation as f32 >= 0.7 * annual_precipitation as f32
                    && annual_precipitation < (10.0 * avg_temperature) as i32)
                || annual_precipitation < (10.0 * avg_temperature + 70.0) as i32
            {
                if avg_temperature >= 18.0 {
                    // BWh
                    return Climate::HotDesert;
                } else {
                    // BWk
                    return Climate::ColdDesert;
                }
            } else {
                if avg_temperature >= 18.0 {
                    // BSh
                    return Climate::HotSemiarid;
                } else {
                    // BSk
                    return Climate::ColdSemiarid;
                }
            }
        } else if max_temperature >= 10.0 && min_temperature <= -3.0 {
            // D - Continental
            // temperature of warmest month greater than or equal to 10 °C,
            //  and temperature of coldest month –3 °C or lower

            let abcd = get_abcd(latitude, longitude, &complete_map);

            match abcd {
                ABCD::A => {
                    match get_swf(latitude, longitude, &complete_map) {
                        SWF::F => {
                            // Dfb
                            return Climate::HotHumidContinental;
                        }
                        SWF::S => {
                            // Dsa
                            return Climate::HotMediterraneanContinental;
                        }
                        SWF::W => {
                            // Dwa
                            return Climate::MonsoonContinental;
                        }
                    }
                }
                ABCD::B => {
                    match get_swf(latitude, longitude, &complete_map) {
                        SWF::F => {
                            // Dfb
                            return Climate::HumidContinental;
                        }
                        SWF::S => {
                            // Dsb
                            return Climate::ColdMediterraneanContinental;
                        }
                        SWF::W => {
                            // Dwb
                            return Climate::MonsoonContinental;
                        }
                    }
                }
                ABCD::C => {
                    // D*c
                    return Climate::Subarctic;
                }
                ABCD::D => {
                    // D*d
                    return Climate::SevereSubarctic;
                }
            }
        } else if min_temperature >= 18.0 {
            // A - Tropical
            if min_precipitation >= 60 {
                // Af
                return Climate::Tropical;
            } else if min_precipitation < 60
                && min_precipitation > (100 - annual_precipitation / 25)
            {
                // Am
                return Climate::Monsoon;
            } else {
                // Aw / As
                return Climate::Savanah;
            }
        } else if max_temperature >= 10.0 && min_temperature > -3.0 && min_temperature < 18.0 {
            // C - Temperate
            let abcd = get_abcd(latitude, longitude, &complete_map);
            match get_swf(latitude, longitude, &complete_map) {
                SWF::S => {
                    match abcd {
                        ABCD::A => {
                            // Csa
                            return Climate::HotMediterranean;
                        }
                        ABCD::B => {
                            // Csb / Csc / Csd
                            return Climate::WarmMediterranean;
                        }
                        ABCD::C | ABCD::D => {
                            // Csc / Csd
                            return Climate::ColdMediterranean;
                        }
                    }
                }
                SWF::W => {
                    match abcd {
                        ABCD::A => {
                            // Cwa
                            return Climate::SubtropicalMonsoon;
                        }
                        ABCD::B => {
                            // Cwb / Cwc - Subtropical highland
                            return Climate::Oceanic;
                        }
                        ABCD::C | ABCD::D => {
                            // Cfc - Subarctic Marine West Coast
                            return Climate::SubarcticOceanic;
                        }
                    }
                }
                SWF::F => {
                    match abcd {
                        ABCD::A => {
                            // Cfa
                            return Climate::HumidSubtropical;
                        }
                        ABCD::B => {
                            // Cfb - Marine West Coast
                            return Climate::Oceanic;
                        }
                        ABCD::C | ABCD::D => {
                            // Cfc - Subarctic Marine West Coast
                            return Climate::SubarcticOceanic;
                        }
                    }
                }
            }
        } else {
            // undefined
            return Climate::Undefined;
        }
    }
}

impl<S: MapShape> PipelineStep<S> for DefineKoppenClimate {
    fn process_element(&self, _x: usize, _y: usize, _complete_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        output_map.climate =
            PartialMap::new(output_map.height.circunference, output_map.height.height);
        // process elements in parallel with rayon
        let input_map2 = output_map.clone();
        output_map
            .climate
            .values
            .par_iter_mut()
            .enumerate()
            .for_each(|(x, inner_vec)| {
                inner_vec.iter_mut().enumerate().for_each(|(y, num)| {
                    *num = self.process_climate_element(x, y, Arc::new(&input_map2));
                });
            });
        return output_map;
    }
}
