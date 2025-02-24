use std::sync::Arc;

use rocket::tokio::time::Instant;

use crate::{
    complete_map::CompleteMap, pipeline_steps::vegetation::Vegetation, shapes::map_shape::MapShape,
};

use super::{
    annual_precipitation::CalculateAnnualPrecipitation, climate::DefineKoppenClimate,
    continentality::CalculateContinentality, pipeline_step::PipelineStep,
    precipitation::CalculatePrecipitation,
    temperature_from_continentality::TemperatureFromContinentality,
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

        let i0 = Instant::now();
        output_map = CalculateContinentality {}.apply(&output_map);
        let i1 = Instant::now();
        dbg!("CalculateContinentality time:", i1 - i0);
        output_map =
            TemperatureFromContinentality::new(self.equator_temperature, self.pole_temperature)
                .apply(&output_map);
        let i2 = Instant::now();
        dbg!("TemperatureFromContinentality time:", i2 - i1);
        // output_map = DefinePressure::new().apply(&output_map);
        // output_map = DefineWindsGradient::new().apply(&output_map);
        output_map = CalculatePrecipitation {}.apply(&output_map);
        let i3 = Instant::now();
        dbg!("CalculatePrecipitation time:", i3 - i2);
        // output_map =
        //     AdjustPrecipitationPercentiles::new(&self.precipitation_percentiles).apply(&output_map);
        output_map = CalculateAnnualPrecipitation {}.apply(&output_map);
        output_map = DefineKoppenClimate {}.apply(&output_map);
        output_map = Vegetation::new().apply(&output_map);
        // output_map = CreateRivers {}.apply(&output_map);
        let i3 = Instant::now();
        dbg!("Total climate calculation time:", i3 - i0);
        return output_map;
    }
}
