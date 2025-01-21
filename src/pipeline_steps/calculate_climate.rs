use std::sync::Arc;

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{
    adjust_percentiles::AdjustPrecipitationPercentiles,
    annual_precipitation::CalculateAnnualPrecipitation, climate::DefineKoppenClimate,
    continentality::CalculateContinentality, gradient_winds::DefineWindsGradient,
    pipeline_step::PipelineStep, precipitation::CalculatePrecipitation, pressure::DefinePressure,
    rivers::CreateRivers, temperature_from_continentality::TemperatureFromContinentality,
};

#[derive(Debug)]
pub struct CalculateClimate {
    precipitation_percentiles: Vec<(f32, i32)>,
    pole_temperature: f32,
    equator_temperature: f32,
}

impl CalculateClimate {
    pub fn new(
        precipitation_percentiles: &Vec<(f32, i32)>,
        equator_temperature: f32,
        pole_temperature: f32,
    ) -> Self {
        Self {
            precipitation_percentiles: precipitation_percentiles.clone(),
            pole_temperature,
            equator_temperature,
        }
    }
}

impl<S: MapShape> PipelineStep<S> for CalculateClimate {
    fn process_element(&self, _x: usize, _y: usize, _complete_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        output_map = CalculateContinentality {}.apply(&output_map);
        output_map =
            TemperatureFromContinentality::new(self.equator_temperature, self.pole_temperature)
                .apply(&output_map);
        output_map = DefinePressure::new().apply(&output_map);
        output_map = DefineWindsGradient::new().apply(&output_map);
        output_map = CalculatePrecipitation {}.apply(&output_map);
        output_map =
            AdjustPrecipitationPercentiles::new(&self.precipitation_percentiles).apply(&output_map);
        output_map = CalculateAnnualPrecipitation {}.apply(&output_map);
        output_map = DefineKoppenClimate {}.apply(&output_map);
        // output_map = CreateRivers {}.apply(&output_map);
        return output_map;
    }
}
