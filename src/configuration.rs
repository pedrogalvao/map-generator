use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ShapeEnum {
    Globe,
    Cylinder,
    Flat,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Configuration {
    pub shape: ShapeEnum,
    pub seed: u32,
    pub height_pixels: usize,
    pub width_pixels: usize,
    pub number_of_plates: usize,
    pub water_percentage: f32,
    pub land_height_percentiles: Vec<(f32, i32)>,
    pub ocean_depth_percentiles: Vec<(f32, i32)>,
    pub precipitation_percentiles: Vec<(f32, i32)>,
    pub number_of_rivers: u32,
    pub make_climate: bool,
    pub hotspots: f32,
    pub erosion_iterations: u32,
    pub supercontinent: bool,
    pub islands: f32,
    // pub height_source_img: String
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClimateConfiguration {
    pub pole_temperature: f32,
    pub equator_temperature: f32,
    pub humidity: f32,
}
