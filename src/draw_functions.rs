use std::fs;

use image::Rgba;

use crate::{
    complete_map::CompleteMap,
    map_view::{
        color_scheme::{
            ClimateColorScheme, ANNUAL_PRECIPITATION_COLORS, ATLAS_COLORS, DARK_MOUNTAINS,
            DEFAULT_COLORS, HIGH_CONTRAST_COLORS, LAND_WATER_COLORS, OLD_STYLE_COLORS,
            PRECIPITATION_COLORS, PRESSURE_COLORS, TEMPERATURE_COLORS, TEXTURE_SCHEME, WHITE,
        },
        contour_layer::ContourLayer,
        map_view::MapView,
        mountain_img_layer::MountainImgLayer,
        parallels_meridians_layer::ParallelsMeridiansLayer,
        partial_map_layer::PartialMapLayer,
        projection::{
            azimutal::Azimutal, double_azimutal::DoubleAzimutal, equirectangular::Equirectangular,
            mercator::Mercator, mollweide::Mollweide, oblique::Oblique, orthographic::Orthographic,
        },
        rhumb_lines::RhumbLinesLayer,
        rivers_layer::RiversLayer,
        shadow_layer::ShadowLayer,
        texture_layer::TextureLayer,
        wind_layer::WindMapLayer,
    },
    pmap_layer,
    shapes::map_shape::MapShape,
};

pub fn draw_mollweide_height<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/height/mollweide";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Mollweide, S> = MapView::new();
    mv.layers = vec![
        pmap_layer!(height, DEFAULT_COLORS),
        Box::new(ParallelsMeridiansLayer::default()),
    ];
    mv.resolution = [4000, 2000];
    for i in 0..60 {
        mv.center[1] = i as f32 * 360.0 / 60.0;
        mv.draw(
            &cmap,
            ("out/height/mollweide/mollweide".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_azimutal<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/height/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Azimutal, S> = MapView::new();
    mv.layers = vec![
        pmap_layer!(height, HIGH_CONTRAST_COLORS),
        Box::new(RiversLayer::default()),
        Box::new(ParallelsMeridiansLayer::default()),
    ];
    mv.resolution = [2000, 2000];
    for i in 0..6 {
        mv.center[1] = i as f32 * 360.0 / 6.0;
        mv.draw(
            &cmap,
            ("out/height/azimutal".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_double_azimutal<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/height/azimuthal";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Oblique<DoubleAzimutal>, S> = MapView::new();
    let mut pm_layer = ParallelsMeridiansLayer::default();
    pm_layer.set_meridians_interval(30.0);
    pm_layer.set_parallels_interval(30.0);
    // pm_layer.color = Rgba([255, 255, 255, 255]);
    mv.layers = vec![pmap_layer!(height, DEFAULT_COLORS), Box::new(pm_layer)];
    mv.resolution = [4000, 2000];
    mv.projection.latitude = -90.0;
    for i in 0..60 {
        // mv.projection.latitude = -i as f32 * 180.0 / 6.0;
        mv.projection.longitude = i as f32 * 360.0 / 60.0;
        // mv.center[1] = i as f32 * 360.0 / 60.0;
        mv.draw(
            &cmap,
            ("out/height/azimuthal/".to_owned() + &(i + 10).to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_orthographic<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let mut mv: MapView<Orthographic, S> = MapView::new();
    mv.layers = vec![
        pmap_layer!(height, HIGH_CONTRAST_COLORS),
        Box::new(RiversLayer::default()),
        Box::new(ParallelsMeridiansLayer::default()),
    ];
    mv.resolution = [4000, 2000];
    for i in 0..6 {
        mv.center[1] = i as f32 * 360.0 / 6.0;
        mv.draw(
            &cmap,
            ("out/height/orthographic".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_orthographic_inclined<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/height/orthographic";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Oblique<Orthographic>, S> = MapView::new();

    let pml = ParallelsMeridiansLayer::default();
    mv.layers = vec![pmap_layer!(height, DEFAULT_COLORS), Box::new(pml)];
    mv.resolution = [2000, 1000];
    mv.projection.latitude = -90.0;
    mv.center[1] = -23.4;
    for i in 0..60 {
        // mv.projection.longitude = i as f32 * 360.0 / 60.0;
        mv.projection.longitude = i as f32 * 360.0 / 60.0;
        mv.draw(
            &cmap,
            ("out/height/orthographic/".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_orthographic_oblique<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let mut mv: MapView<Oblique<Orthographic>, S> = MapView::new();

    let pml = ParallelsMeridiansLayer::default();
    mv.layers = vec![pmap_layer!(height, HIGH_CONTRAST_COLORS), Box::new(pml)];
    mv.resolution = [1200, 600];
    for i in 0..16 {
        mv.projection.latitude = -i as f32 * 180.0 / 8.0;
        mv.projection.longitude = i as f32 * 360.0 / 8.0;
        mv.center[1] = i as f32 * 180.0 / 16.0 - 90.0;
        mv.draw(
            &cmap,
            ("out/height/orthographic_oblique".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_orthographic_oblique_plates<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let mut mv: MapView<Oblique<Orthographic>, S> = MapView::new();

    let mut pml = ParallelsMeridiansLayer::default();
    pml.color = Rgba([0, 0, 0, 255]);
    mv.layers = vec![
        pmap_layer!(height, ATLAS_COLORS),
        Box::new(ContourLayer::new(|m| &m.height, Rgba([0, 0, 0, 255]), 1)),
        Box::new(ContourLayer::new(
            |m| &m.tectonic_plates,
            Rgba([255, 0, 0, 255]),
            2,
        )),
        pmap_layer!(height, DARK_MOUNTAINS),
        Box::new(pml),
    ];
    mv.resolution = [1200, 600];
    for i in 0..16 {
        mv.projection.latitude = -i as f32 * 180.0 / 8.0;
        mv.projection.longitude = i as f32 * 360.0 / 8.0;
        mv.center[1] = i as f32 * 180.0 / 8.0 - 90.0;
        mv.draw(
            &cmap,
            ("out/plates/orthographic_oblique".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_mollweide_oblique<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let mut mv: MapView<Oblique<Mollweide>, S> = MapView::new();

    let mut pml = ParallelsMeridiansLayer::default();
    pml.set_meridians_interval(180.0);
    pml.set_parallels_interval(45.0);
    pml.color = Rgba([255, 100, 100, 255]);
    mv.layers = vec![
        pmap_layer!(height, HIGH_CONTRAST_COLORS),
        Box::new(ParallelsMeridiansLayer::default()),
        Box::new(pml),
    ];
    mv.resolution = [1200, 600];
    for i in 0..16 {
        mv.projection.latitude = i as f32 * 180.0 / 8.0;
        mv.projection.longitude = i as f32 * 180.0 / 8.0;
        mv.center[1] = i as f32 * 180.0 / 16.0 - 90.0;
        mv.draw(
            &cmap,
            ("out/height/mollweide_oblique".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_plates<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/plates/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Mollweide, S> = MapView::new();
    mv.layers = vec![
        pmap_layer!(height, LAND_WATER_COLORS),
        Box::new(ContourLayer::new(|m| &m.height, Rgba([0, 0, 0, 255]), 1)),
        Box::new(ContourLayer::new(
            |m| &m.tectonic_plates,
            Rgba([255, 0, 0, 255]),
            1,
        )),
        pmap_layer!(height, DARK_MOUNTAINS),
    ];
    mv.resolution = [2000, 1000];
    for i in 0..6 {
        mv.center[1] = i as f32 * 360.0 / 6.0;
        mv.draw(
            &cmap,
            ("out/plates/".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_precipitation<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/precipitation/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Mollweide, S> = MapView::new();
    mv.resolution = [2000, 1000];
    for t in 0..cmap.precipitation.len() {
        mv.time_of_year = t;
        mv.layers = vec![
            Box::new(PartialMapLayer::new(|m| &m.height, Box::new(WHITE.clone()))),
            pmap_layer!(height, DARK_MOUNTAINS),
            Box::new(RiversLayer::default()),
            pmap_layer!(precipitation, t, PRECIPITATION_COLORS),
        ];
        mv.draw(
            &cmap,
            ("out/precipitation/".to_owned() + &t.to_string() + ".png").as_str(),
        );
    }
    mv.layers = vec![
        pmap_layer!(height, WHITE),
        pmap_layer!(height, DARK_MOUNTAINS),
        pmap_layer!(annual_precipitation, ANNUAL_PRECIPITATION_COLORS),
    ];
    mv.draw(&cmap, "out/precipitation/annual.png");
}

pub fn draw_temperature<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/temperature/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Mollweide, S> = MapView::new();
    mv.resolution = [1000, 500];
    for t in 0..cmap.temperature.len() {
        mv.time_of_year = t;
        mv.layers = vec![
            pmap_layer!(temperature, t, TEMPERATURE_COLORS),
            Box::new(ParallelsMeridiansLayer::default()),
            Box::new(ParallelsMeridiansLayer::tropics()),
            Box::new(ParallelsMeridiansLayer::polar_circle()),
        ];
        mv.draw(
            &cmap,
            ("out/temperature/".to_owned() + &t.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_equirectangular<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/height/equirectangular";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Equirectangular, S> = MapView::new();

    let mut pm_layer = ParallelsMeridiansLayer::default();
    pm_layer.color = Rgba([255, 255, 255, 150]);
    mv.resolution = [4000, 2000];
    mv.center[1] = 0.0;
    mv.layers = vec![
        pmap_layer!(height, DEFAULT_COLORS),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([255, 255, 255, 255]),
            1,
        )),
        Box::new(RiversLayer::new(Rgba([110, 180, 255, 255]))),
        Box::new(pm_layer),
    ];
    mv.resolution = [2000, 1000];
    for i in 0..60 {
        mv.center[1] = i as f32 * 360.0 / 60.0;
        mv.draw(
            &cmap,
            ("out/height/equirectangular/equirectangular".to_owned() + &i.to_string() + ".png")
                .as_str(),
        );
    }
    mv.resolution = [500, 250];
    mv.layers = vec![pmap_layer!(height, ATLAS_COLORS)];
    for i in 0..6 {
        mv.center[1] = i as f32 * 360.0 / 6.0;
        mv.draw(
            &cmap,
            ("out/height/equirectangular_small".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_precipitation_equirectangular<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/precipitation/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Equirectangular, S> = MapView::new();
    mv.resolution = [2000, 1000];
    for t in 0..24 {
        mv.time_of_year = t;
        mv.layers = vec![
            pmap_layer!(height, WHITE),
            pmap_layer!(height, DARK_MOUNTAINS),
            pmap_layer!(precipitation, t, PRECIPITATION_COLORS),
            Box::new(RiversLayer::default()),
            Box::new(ContourLayer::new(
                move |m| &m.height,
                Rgba([0, 0, 0, 255]),
                1,
            )),
        ];
        mv.draw(
            &cmap,
            ("out/precipitation/".to_owned() + &t.to_string() + ".png").as_str(),
        );
    }
    mv.layers = vec![
        pmap_layer!(height, WHITE),
        pmap_layer!(height, DARK_MOUNTAINS),
        pmap_layer!(annual_precipitation, ANNUAL_PRECIPITATION_COLORS),
        Box::new(ParallelsMeridiansLayer::default()),
        Box::new(ParallelsMeridiansLayer::tropics()),
        Box::new(ParallelsMeridiansLayer::polar_circle()),
    ];
    mv.draw(&cmap, "out/precipitation/annual.png");

    draw_temperature(&cmap);
}

pub fn draw_continentality<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/continentality/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Equirectangular, S> = MapView::new();
    mv.resolution = [2000, 1000];
    let mut pml = ParallelsMeridiansLayer::default();
    pml.color = Rgba([0, 0, 0, 255]);
    mv.layers = vec![
        pmap_layer!(height, WHITE),
        pmap_layer!(height, DARK_MOUNTAINS),
        pmap_layer!(continentality, PRECIPITATION_COLORS),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            1,
        )),
        Box::new(pml),
    ];
    mv.draw(&cmap, "out/continentality/equirectangular.png");
}

pub fn draw_old_style_no_contour<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/old";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Oblique<DoubleAzimutal>, S> = MapView::new();
    mv.projection.latitude = -90.0;
    mv.projection.longitude = -120.0;
    mv.resolution = [4000, 2000];
    let mut parallels_meridians_layer = ParallelsMeridiansLayer::new(15.0);
    parallels_meridians_layer.color = Rgba([0, 0, 0, 255]);
    mv.layers = vec![
        pmap_layer!(height, OLD_STYLE_COLORS),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            2,
        )),
        Box::new(RiversLayer::new(Rgba([0, 0, 0, 200]))),
        Box::new(MountainImgLayer::new()),
        Box::new(parallels_meridians_layer),
        Box::new(TextureLayer::new(
            move |m| &m.height,
            Box::new(TEXTURE_SCHEME.clone()),
        )),
    ];
    for i in 0..60 {
        mv.projection.longitude = 360.0 * i as f32 / 60.0;
        mv.draw(
            &cmap,
            ("out/old/".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_old_style<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<DoubleAzimutal, S> = MapView::new();
    mv.resolution = [2000, 1000];
    mv.layers = vec![
        pmap_layer!(height, OLD_STYLE_COLORS),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            1,
        )),
        Box::new(ShadowLayer::default(move |m| &m.height)),
        Box::new(MountainImgLayer::new()),
    ];
    // mv.draw(&cmap, "out/old.png");

    let mut parallels_meridians_layer = ParallelsMeridiansLayer::new(15.0);
    parallels_meridians_layer.color = Rgba([0, 0, 0, 255]);
    let mut mv: MapView<Equirectangular, S> = MapView::new();
    mv.resolution = [3000, 1500];
    mv.layers = vec![
        pmap_layer!(height, OLD_STYLE_COLORS),
        Box::new(ShadowLayer::default(move |m| &m.height)),
        Box::new(RiversLayer::new(Rgba([0, 0, 0, 200]))),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            2,
        )),
        Box::new(MountainImgLayer::new()),
        Box::new(parallels_meridians_layer),
        Box::new(TextureLayer::new(
            move |m| &m.height,
            Box::new(TEXTURE_SCHEME.clone()),
        )),
        Box::new(RhumbLinesLayer::default()),
    ];
    mv.draw(&cmap, "out/old_equirectangular.png");

    let mut mv: MapView<Oblique<DoubleAzimutal>, S> = MapView::new();
    mv.projection.latitude = -90.0;
    mv.projection.longitude = -120.0;
    mv.resolution = [4000, 2000];
    let mut parallels_meridians_layer = ParallelsMeridiansLayer::new(15.0);
    parallels_meridians_layer.color = Rgba([0, 0, 0, 255]);
    mv.layers = vec![
        pmap_layer!(height, OLD_STYLE_COLORS),
        Box::new(RiversLayer::new(Rgba([0, 0, 0, 200]))),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            2,
        )),
        Box::new(MountainImgLayer::new()),
        Box::new(parallels_meridians_layer),
        Box::new(TextureLayer::new(
            move |m| &m.height,
            Box::new(TEXTURE_SCHEME.clone()),
        )),
    ];
    mv.draw(&cmap, "out/old_big.png");
    mv.projection.longitude = -110.0;
    mv.draw(&cmap, "out/old_big3.png");
    let mut mv: MapView<Oblique<Azimutal>, S> = MapView::new();
    mv.projection.latitude = -90.0;
    mv.projection.longitude = -30.0;
    mv.resolution = [4000, 2000];
    let mut parallels_meridians_layer = ParallelsMeridiansLayer::new(15.0);
    parallels_meridians_layer.color = Rgba([0, 0, 0, 255]);
    mv.layers = vec![
        pmap_layer!(height, OLD_STYLE_COLORS),
        Box::new(RiversLayer::new(Rgba([0, 0, 0, 200]))),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            1,
        )),
        Box::new(MountainImgLayer::new()),
        Box::new(parallels_meridians_layer),
        Box::new(TextureLayer::new(
            move |m| &m.height,
            Box::new(TEXTURE_SCHEME.clone()),
        )),
    ];
    mv.draw(&cmap, "out/old_big2.png");
}

pub fn draw_climate<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/climate/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Equirectangular, S> = MapView::new();
    mv.resolution = [4000, 2000];
    mv.layers = vec![
        pmap_layer!(height, LAND_WATER_COLORS),
        pmap_layer!(climate, ClimateColorScheme {}),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            1,
        )),
        Box::new(ParallelsMeridiansLayer::default()),
    ];
    mv.draw(&cmap, "out/climate/equirectangular.png");
    mv.center[1] = 180.0;
    mv.draw(&cmap, "out/climate/equirectangular180.png");

    let mut mv: MapView<Mercator, S> = MapView::new();
    mv.resolution = [2000, 2000];
    mv.layers = vec![
        pmap_layer!(height, LAND_WATER_COLORS),
        pmap_layer!(climate, ClimateColorScheme {}),
        Box::new(RiversLayer::default()),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            1,
        )),
        Box::new(ParallelsMeridiansLayer::default()),
    ];
    mv.center[1] = 0.0;
    mv.draw(&cmap, "out/climate/mercator.png");

    let mut mv: MapView<Mollweide, S> = MapView::new();
    mv.resolution = [4000, 2000];
    mv.layers = vec![
        pmap_layer!(height, HIGH_CONTRAST_COLORS),
        pmap_layer!(climate, ClimateColorScheme {}),
        Box::new(RiversLayer::default()),
        pmap_layer!(height, DARK_MOUNTAINS),
        Box::new(ContourLayer::new(
            move |m| &m.height,
            Rgba([0, 0, 0, 255]),
            1,
        )),
        Box::new(ParallelsMeridiansLayer::default()),
        Box::new(ParallelsMeridiansLayer::tropics()),
        Box::new(ParallelsMeridiansLayer::polar_circle()),
    ];
    mv.center[1] = 0.0;
    mv.draw(&cmap, "out/climate/mollweide.png");
    mv.center[1] = 180.0;
    mv.draw(&cmap, "out/climate/mollweide180.png");
}

pub fn draw_pressure<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/pressure/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Equirectangular, S> = MapView::new();
    mv.resolution = [2000, 1000];
    for t in 0..12 {
        mv.time_of_year = t;
        mv.layers = vec![
            pmap_layer!(height, WHITE),
            pmap_layer!(height, DARK_MOUNTAINS),
            pmap_layer!(atm_pressure, t, PRESSURE_COLORS),
            Box::new(ContourLayer::new(
                move |m| &m.height,
                Rgba([0, 0, 0, 255]),
                1,
            )),
            Box::new(ParallelsMeridiansLayer::default()),
            Box::new(ParallelsMeridiansLayer::tropics()),
            Box::new(ParallelsMeridiansLayer::polar_circle()),
            Box::new(WindMapLayer::new(t, Rgba([0, 255, 0, 255]))),
        ];
        mv.draw(
            &cmap,
            ("out/pressure/".to_owned() + &t.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_mercator<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    let output_path = "out/height/mercator/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Mercator, S> = MapView::new();
    mv.resolution = [3000, 3000];
    mv.layers = vec![
        pmap_layer!(height, DEFAULT_COLORS),
        Box::new(ParallelsMeridiansLayer::default()),
    ];
    for i in 0..60 {
        mv.center[1] = i as f32 * 360.0 / 60.0;
        mv.draw(
            &cmap,
            ("out/height/mercator/mercator".to_owned() + &i.to_string() + ".png").as_str(),
        );
    }
}

pub fn draw_all<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    // println!("draw_orthographic_oblique");
    // draw_orthographic_inclined(&cmap);
    // println!("draw_equirectangular");
    // draw_equirectangular(&cmap);
    // println!("draw_double_azimutal");
    // draw_double_azimutal(&cmap);
    // println!("draw_mollweide_height");
    // draw_mollweide_height(&cmap);
    // draw_mercator(&cmap);
    // println!("draw_old");
    // draw_old_style_no_contour(&cmap);
    // draw_old_style(&cmap);
    draw_continentality(&cmap);
    draw_climate(&cmap);
    println!("draw_precipitation");
    draw_precipitation(&cmap);
    draw_precipitation_equirectangular(&cmap);
    println!("draw_pressure");
    draw_pressure(&cmap);
    println!("draw_temperature");
    draw_temperature(&cmap);
    println!("draw_azimutal");
    draw_azimutal(&cmap);
    println!("draw_plates");
    draw_plates(&cmap);
    println!("draw_orthographic");
    draw_orthographic(&cmap);
    println!("draw_orthographic_oblique");
    draw_orthographic_oblique(&cmap);
    println!("draw_orthographic_oblique_plates");
    draw_orthographic_oblique_plates(&cmap);
    println!("draw_mollweide_oblique");
    draw_mollweide_oblique(&cmap);
}

pub fn draw_all_equirectangular<S: MapShape + 'static>(cmap: &CompleteMap<S>) {
    draw_equirectangular(&cmap);
    draw_precipitation_equirectangular(&cmap)
}
