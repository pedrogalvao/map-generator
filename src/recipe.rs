use crate::{
    configuration::Configuration,
    map_pipeline::MapPipeline,
    pipeline_steps::{
        adjust_percentiles::{
            AdjustLandHeightPercentiles, AdjustOceanDepthPercentiles,
            AdjustPrecipitationPercentiles,
        },
        annual_precipitation::CalculateAnnualPrecipitation,
        climate::DefineKoppenClimate,
        continentality::CalculateContinentality,
        define_coastlines::DefineCoastline,
        gradient_winds::DefineWindsGradient,
        height_in_plates::HeightInPlates,
        height_noise::HeightNoise,
        height_noise_mult::HeightNoiseMult,
        height_noise_poles::HeightNoisePoles,
        hotspots::Hotspots,
        hydraulic_erosion::HydraulicErosion,
        load_height::LoadHeight,
        mountains::AddMountains,
        noisy_voronoi::NoisyVoronoi,
        noisy_voronoi_supercontinent::NoisyVoronoiSupercontinent,
        plate_gap::AddPlateGap,
        precipitation::CalculatePrecipitation,
        pressure::DefinePressure,
        resize::Resize,
        rivers::CreateRivers,
        smooth::{Smooth, SmoothOcean},
        supercontinent_height_noise::SupercontinentHeightNoise,
        tectonic_edges::DefineTecEdges,
        temperature_from_continentality::TemperatureFromContinentality,
        translation_noise::TranslationNoise,
        water_level::WaterLevel,
    },
    shapes::map_shape::MapShape,
};

fn standard_base_height<T: MapShape + 'static>(config: &Configuration) -> MapPipeline<T> {
    let mut map_pipeline: MapPipeline<T> = MapPipeline::new();

    let exp = ((config.width_pixels as f32 / 250.0).ln() / (2.0 as f32).ln()) as u32;
    map_pipeline.circunference = (config.width_pixels / (2 as usize).pow(exp)) as usize;
    map_pipeline.height = (config.height_pixels / (2 as usize).pow(exp)) as usize;

    let step1 = NoisyVoronoi::new(config.seed, config.number_of_plates);
    map_pipeline.add_step(step1);

    // base height noise
    let step2 = HeightInPlates::new(config.seed, 1.5, 200.0);
    map_pipeline.add_step(step2);
    let step2 = HeightInPlates::new(config.seed, 2.5, 150.0);
    map_pipeline.add_step(step2);

    let step2 = HeightInPlates::new(config.seed, 4.0, 100.0);
    map_pipeline.add_step(step2);

    let step2 = HeightInPlates::new(config.seed, 8.0, 80.0);
    map_pipeline.add_step(step2);
    let step2 = HeightInPlates::new(config.seed, 16.0, 50.0);
    map_pipeline.add_step(step2);

    let mut step: WaterLevel = WaterLevel::new();
    step.percentage = config.water_percentage;
    map_pipeline.add_step(step);

    map_pipeline.add_step(AdjustLandHeightPercentiles::new(
        &vec![(0.0, 0), (90.0, 300), (100.0, 700)],
        0.0,
    ));

    map_pipeline.add_step(AdjustOceanDepthPercentiles::new(
        &vec![
            (0.0, -3000),
            (50.0, -2000),
            (70.0, -1000),
            (80.0, -200),
            (100.0, 0),
        ],
        0.0,
    ));

    let step2 = HeightInPlates::new(config.seed, 3.5, 20.0);
    map_pipeline.add_step(step2);

    return map_pipeline;
}

fn create_height_pipeline<T: MapShape + 'static>(config: &Configuration) -> MapPipeline<T> {
    let water_percentage = config.water_percentage;

    let mut map_pipeline: MapPipeline<T>;
    if config.supercontinent {
        map_pipeline = supercontinent_base_height(config);
    } else {
        map_pipeline = standard_base_height(config);
    }

    let mut plate_gap_step = AddPlateGap::new();
    plate_gap_step.oceanic_plates =
        (config.number_of_plates as f32 * config.water_percentage / 250.0) as usize;
    map_pipeline.add_step(plate_gap_step);
    map_pipeline.add_step(DefineTecEdges::new());

    let step = AddMountains::new(2, 140.0, 0.2);
    map_pipeline.add_step(step);
    let mut step: WaterLevel = WaterLevel::new();
    step.percentage = water_percentage;
    map_pipeline.add_step(step);
    let step = HeightNoiseMult::new(config.seed + 1, 150.0, 0.4);
    map_pipeline.add_step(step);

    map_pipeline.add_step(AdjustLandHeightPercentiles::new(
        &config.land_height_percentiles,
        config.water_percentage,
    ));
    map_pipeline.add_step(AdjustOceanDepthPercentiles::new(
        &config.ocean_depth_percentiles,
        config.water_percentage,
    ));

    let step = HydraulicErosion::new(config.erosion_iterations);
    map_pipeline.add_step(step);

    let exp = ((config.width_pixels as f32 / 250.0).ln() / (2.0 as f32).ln()) as u32;
    for k in 1..exp + 1 {
        map_pipeline.add_step(Resize { factor: 2.0 });

        let step = AddMountains::new(config.seed + k + 100, 50.0, 0.6 as f32);
        map_pipeline.add_step(step);

        let step = HeightNoise::new(
            config.seed + 1000 + k,
            k as f32 * 50.0,
            250.0 / (k * k) as f32,
        );
        map_pipeline.add_step(step);
        let step = HeightNoise::new(
            config.seed + 1000 + k,
            k as f32 * 100.0,
            70.0 / (k * k) as f32,
        );
        map_pipeline.add_step(step);
        let step = HeightNoise::new(
            config.seed + 1000 + k,
            k as f32 * 200.0,
            50.0 / (k * k) as f32,
        );
        map_pipeline.add_step(step);
        let step = HeightNoise::new(
            config.seed + 1000 + k,
            k as f32 * 400.0,
            30.0 / (k * k) as f32,
        );
        map_pipeline.add_step(step);

        let step = HeightNoisePoles::new(config.seed + 100 + k, 100.0, 200.0 / k as f32);
        map_pipeline.add_step(step);

        let step = HydraulicErosion::new(config.erosion_iterations);
        map_pipeline.add_step(step);
    }

    let mut smooth_step = Smooth::new();
    smooth_step.pixel_distance = 1;
    map_pipeline.add_step(smooth_step);

    map_pipeline.add_step(AdjustLandHeightPercentiles::new(
        &config.land_height_percentiles,
        config.water_percentage,
    ));
    map_pipeline.add_step(AdjustOceanDepthPercentiles::new(
        &config.ocean_depth_percentiles,
        config.water_percentage,
    ));
    let step = HydraulicErosion::new(config.erosion_iterations);
    map_pipeline.add_step(step);
    let mut smooth_step = Smooth::new();
    smooth_step.pixel_distance = 1;
    map_pipeline.add_step(smooth_step);
    let step = HydraulicErosion::new(config.erosion_iterations);
    map_pipeline.add_step(step);
    map_pipeline.add_step(Resize { factor: 2.0 });
    let step = TranslationNoise::new(config.seed);
    map_pipeline.add_step(step);
    map_pipeline.add_step(SmoothOcean::new());
    if config.islands > 0.0 {
        let step = HeightNoise::new(config.seed + 10000, 60.0, 70.0 as f32 * config.islands);
        map_pipeline.add_step(step);
        let step = HeightNoise::new(config.seed + 20000, 100.0, 40.0 as f32 * config.islands);
        map_pipeline.add_step(step);
        let step = HeightNoise::new(config.seed + 30000, 200.0, 20.0 as f32 * config.islands);
        map_pipeline.add_step(step);
    }
    let step = HydraulicErosion::new(config.erosion_iterations);
    map_pipeline.add_step(step);
    if config.hotspots > 0.0 {
        map_pipeline.add_step(Hotspots::new(1, 30));
    }

    return map_pipeline;
}

fn supercontinent_base_height<T: MapShape + 'static>(config: &Configuration) -> MapPipeline<T> {
    let water_percentage = config.water_percentage;

    let mut map_pipeline: MapPipeline<T> = MapPipeline::new();
    map_pipeline.circunference = (config.width_pixels / 4) as usize;
    map_pipeline.height = (config.height_pixels / 4) as usize;

    let step1 = NoisyVoronoiSupercontinent::new(config.seed, config.number_of_plates);
    map_pipeline.add_step(step1);

    let step2 = SupercontinentHeightNoise::new(config.seed, 1.5, 200.0);
    map_pipeline.add_step(step2);
    let step2 = SupercontinentHeightNoise::new(config.seed, 2.5, 150.0);
    map_pipeline.add_step(step2);

    let step2 = SupercontinentHeightNoise::new(config.seed, 4.0, 100.0);
    map_pipeline.add_step(step2);

    let step2 = SupercontinentHeightNoise::new(config.seed, 8.0, 80.0);
    map_pipeline.add_step(step2);
    let step2 = SupercontinentHeightNoise::new(config.seed, 16.0, 50.0);
    map_pipeline.add_step(step2);

    let mut step: WaterLevel = WaterLevel::new();
    step.percentage = water_percentage;
    map_pipeline.add_step(step);

    map_pipeline.add_step(AdjustLandHeightPercentiles::new(
        &vec![(0.0, 0), (90.0, 300), (100.0, 700)],
        0.0,
    ));

    map_pipeline.add_step(AdjustOceanDepthPercentiles::new(
        &vec![
            (0.0, -3000),
            (50.0, -2000),
            (70.0, -1000),
            (80.0, -200),
            (100.0, 0),
        ],
        config.water_percentage,
    ));

    let step2 = HeightInPlates::new(config.seed, 3.5, 20.0);
    map_pipeline.add_step(step2);

    return map_pipeline;
}

pub fn standard_recipe<T: MapShape + 'static>(config: &Configuration) -> MapPipeline<T> {
    let mut map_pipeline = create_height_pipeline(config);

    if config.make_climate {
        map_pipeline.add_step(CalculateContinentality {});
        map_pipeline.add_step(TemperatureFromContinentality::default());
        map_pipeline.add_step(CalculatePrecipitation {});
        map_pipeline.add_step(CalculateAnnualPrecipitation {});
        map_pipeline.add_step(DefineKoppenClimate {});
        map_pipeline.add_step(CreateRivers {});
    }
    map_pipeline.add_step(DefineCoastline {});

    map_pipeline
}

pub fn recipe_from_image<T: MapShape + 'static>(filepath: String) -> MapPipeline<T> {
    let mut map_pipeline: MapPipeline<T> = MapPipeline::new();
    map_pipeline.add_step(LoadHeight::new(filepath));

    map_pipeline.add_step(DefineCoastline {});

    map_pipeline
}
