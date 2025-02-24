use std::fs;

use crate::{
    complete_map::CompleteMap,
    map_view::{
        projection::{
            azimutal::Azimutal, double_azimutal::DoubleAzimutal,
            double_orthographic::DoubleOrthographic, equirectangular::Equirectangular,
            mercator::Mercator, mollweide::Mollweide, oblique::Oblique, orthographic::Orthographic,
            pseudocylindrical::PseudoCylindrical,
        },
        util::deserialize_rgba,
    },
    pmap_layer,
    shapes::map_shape::MapShape,
};

use super::{
    color_scheme::{
        CategoryColorScheme, ClimateColorScheme, GradientColorScheme, ANNUAL_PRECIPITATION_COLORS, CONTINENTALITY_COLORS, TEMPERATURE_COLORS, TEXTURE_SCHEME
    },
    contour_layer::ContourLayer,
    map_view::MapView,
    mountain_img_layer::MountainImgLayer,
    parallels_meridians_layer::ParallelsMeridiansLayer,
    partial_map_layer::PartialMapLayer,
    projection::projection::Projection,
    relief_shadow::ReliefShadowLayer,
    rhumb_lines::RhumbLinesLayer,
    rivers_layer::RiversLayer,
    satellite::SatelliteLayer,
    texture_layer::TextureLayer,
};

use image::{ImageBuffer, Rgba};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ViewConfiguration {
    center: [f32; 2],
    rotation: f32,
    resolution: [usize; 2],
    layers: Vec<String>,
    // #[serde(serialize_with = "serialize_rgba")]
    #[serde(deserialize_with = "deserialize_rgba")]
    pub land_color: Rgba<u8>,
    // #[serde(serialize_with = "serialize_rgba")]
    #[serde(deserialize_with = "deserialize_rgba")]
    pub water_color: Rgba<u8>,
    // #[serde(serialize_with = "serialize_rgba")]
    #[serde(deserialize_with = "deserialize_rgba")]
    pub contour_color: Rgba<u8>,
    pub output_path: String,
    pub projection: String,
    pub rotation_frames: u32,
    pub parallels_interval: f32,
    // #[serde(serialize_with = "serialize_rgba")]
    #[serde(deserialize_with = "deserialize_rgba")]
    pub parallels_color: Rgba<u8>,
    pub height_colors: Option<GradientColorScheme>,
}

impl ViewConfiguration {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            rotation: 0.0,
            resolution: [1000, 500],
            land_color: Rgba([0, 0, 0, 0]),
            water_color: Rgba([0, 0, 0, 0]),
            contour_color: Rgba([0, 0, 0, 0]),
            layers: vec![String::from("height")],
            projection: String::from("mollweide"),
            output_path: String::from("height"),
            rotation_frames: 1,
            parallels_interval: 0.0,
            parallels_color: Rgba([0, 0, 0, 0]),
            height_colors: None,
        }
    }
}

pub fn create_view<P: Projection, S: MapShape + 'static>(
    view_config: &ViewConfiguration,
) -> MapView<P, S> {
    let mut mv = MapView::<P, S>::new();
    mv.center = view_config.center;
    mv.resolution = view_config.resolution;

    if view_config.land_color != Rgba([0, 0, 0, 0]) || view_config.water_color != Rgba([0, 0, 0, 0])
    {
        mv.layers.push(pmap_layer!(
            height,
            GradientColorScheme {
                points: vec![(0, view_config.water_color), (1, view_config.land_color)]
            }
        ));
    }

    if let Some(color_scheme) = &view_config.height_colors {
        mv.layers.push(pmap_layer!(height, color_scheme));
    }

    for layer in &view_config.layers {
        match layer.as_str() {
            "climate" => {
                mv.layers.push(pmap_layer!(climate, ClimateColorScheme {}));
            }
            "annual_precipitation" => {
                mv.layers.push(pmap_layer!(
                    annual_precipitation,
                    ANNUAL_PRECIPITATION_COLORS
                ));
            }
            "plates" => {
                if mv.layers.len() == 0 {
                    mv.layers.push(pmap_layer!(
                        tectonic_plates,
                        CategoryColorScheme::new_random(50)
                    ));
                } else {
                    mv.layers.push(Box::new(ContourLayer::new(
                        move |m| &m.tectonic_plates,
                        Rgba([255, 0, 0, 255]),
                        2,
                    )));
                }
            }
            "rhumb_lines" => {
                mv.layers.push(Box::new(RhumbLinesLayer::default()));
            }
            "parallels_and_meridians" => {
                if view_config.parallels_interval > 0.0 {
                    let mut parallels_meridians_layer =
                        ParallelsMeridiansLayer::new(view_config.parallels_interval);
                    parallels_meridians_layer.color = view_config.parallels_color;
                    mv.layers.push(Box::new(parallels_meridians_layer));
                }
            }
            "rivers" => {
                mv.layers.push(Box::new(RiversLayer::default()));
            }
            "contour" => {
                if view_config.contour_color != Rgba([0, 0, 0, 0]) {
                    mv.layers.push(Box::new(ContourLayer::new(
                        move |m| &m.height,
                        view_config.contour_color,
                        1,
                    )));
                }
            }
            "paper_texture" => {
                mv.layers.push(Box::new(TextureLayer::new(
                    move |m| &m.height,
                    Box::new(TEXTURE_SCHEME.clone()),
                )));
            }
            "mountains" => {
                mv.layers.push(Box::new(MountainImgLayer::new()));
            }
            "continentality" => {
                mv.layers
                    .push(pmap_layer!(continentality, CONTINENTALITY_COLORS));
            }
            "satellite" => {
                mv.layers.push(Box::new(SatelliteLayer {}));
            }
            "relief_shadow" => {
                mv.layers.push(Box::new(ReliefShadowLayer {}));
            }
            "temperature" => {
                mv.layers
                    .push(pmap_layer!(temperature, 6, TEMPERATURE_COLORS));
            }
            _ => {
                dbg!("Error: invalid layer name:", layer.as_str());
            }
        }
    }

    return mv;
}

pub fn draw_with_config<S: MapShape + 'static>(
    cmap: &CompleteMap<S>,
    view_config: &ViewConfiguration,
) {
    macro_rules! draw_with_projection {
        ($projection:ty, $cmap:expr, $view_config:expr, $latitude:expr, $rotation:expr) => {{
            let output_path = &format!("{}/", $view_config.output_path);
            if !std::path::Path::new(output_path).exists() {
                let _ = fs::create_dir_all(output_path);
            }
            let mut mv = create_view::<Oblique<$projection>, S>(&$view_config);
            for i in 0..view_config.rotation_frames {
                mv.center[1] = $rotation;
                mv.projection.longitude = i as f32 * 360.0 / view_config.rotation_frames as f32;
                mv.projection.latitude = $latitude;
                mv.draw(&$cmap, &format!("{}/{}.png", $view_config.output_path, i));
            }
        }};
    }
    match view_config.projection.as_str() {
        "mollweide" => draw_with_projection!(
            Mollweide,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
        "equirectangular" => draw_with_projection!(
            Equirectangular,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
        "azimuthal" => draw_with_projection!(
            Azimutal,
            cmap,
            view_config,
            view_config.center[0] - 90.0,
            view_config.rotation
        ),
        "double azimuthal" => draw_with_projection!(
            DoubleAzimutal,
            cmap,
            view_config,
            view_config.center[0] - 90.0,
            view_config.rotation
        ),
        "mercator" => draw_with_projection!(
            Mercator,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
        "orthographic" => draw_with_projection!(
            Orthographic,
            cmap,
            view_config,
            view_config.center[0] - 90.0,
            view_config.rotation
        ),
        "double orthographic" => draw_with_projection!(
            DoubleOrthographic,
            cmap,
            view_config,
            view_config.center[0] - 90.0,
            view_config.rotation
        ),
        "pseudocylindrical" => draw_with_projection!(
            PseudoCylindrical,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
        _ => draw_with_projection!(
            Equirectangular,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
    };
}

pub fn img_from_config<S: MapShape + 'static>(
    cmap: &CompleteMap<S>,
    view_config: &ViewConfiguration,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    macro_rules! draw_with_projection {
        ($projection:ty, $cmap:expr, $view_config:expr, $latitude:expr, $rotation:expr) => {{
            let mut mv = create_view::<Oblique<$projection>, S>(&$view_config);
            mv.center[1] = $rotation;
            mv.projection.longitude = view_config.center[1];
            mv.projection.latitude = $latitude;
            return mv.return_image_buffer(&$cmap);
        }};
    }
    match view_config.projection.as_str() {
        "mollweide" => draw_with_projection!(
            Mollweide,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
        "equirectangular" => draw_with_projection!(
            Equirectangular,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
        "azimuthal" => draw_with_projection!(
            Azimutal,
            cmap,
            view_config,
            view_config.center[0] - 90.0,
            view_config.rotation
        ),
        "double azimuthal" => draw_with_projection!(
            DoubleAzimutal,
            cmap,
            view_config,
            view_config.center[0] - 90.0,
            view_config.rotation
        ),
        "mercator" => draw_with_projection!(
            Mercator,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
        "orthographic" => draw_with_projection!(
            Orthographic,
            cmap,
            view_config,
            view_config.center[0] - 90.0,
            view_config.rotation
        ),
        "double orthographic" => draw_with_projection!(
            DoubleOrthographic,
            cmap,
            view_config,
            view_config.center[0] - 90.0,
            view_config.rotation
        ),
        "pseudocylindrical" => draw_with_projection!(
            PseudoCylindrical,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
        _ => draw_with_projection!(
            Equirectangular,
            cmap,
            view_config,
            view_config.center[0],
            view_config.rotation
        ),
    }
}
