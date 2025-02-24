use std::{
    fs::File,
    io::{Read, Write},
};

use bincode::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{
    partial_map::PartialMap,
    pipeline_steps::{climate::Climate, rivers::River},
    shapes::map_shape::MapShape,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct CompleteMap<S: MapShape> {
    pub tectonic_plates: PartialMap<S, usize>,
    pub tectonic_plates_directions: Vec<[f32; 2]>,
    pub height: PartialMap<S, i32>,
    pub temperature: Vec<PartialMap<S, f32>>,
    pub winds: Vec<PartialMap<S, [f32; 3]>>,
    pub precipitation: Vec<PartialMap<S, i32>>,
    pub annual_precipitation: PartialMap<S, i32>,
    pub atm_pressure: Vec<PartialMap<S, i32>>,
    pub climate: PartialMap<S, Climate>,
    pub continentality: PartialMap<S, i32>,
    pub ice_height: Vec<PartialMap<S, i32>>,
    pub rivers: Vec<River>,
    pub tectonic_edges: Vec<[f32; 2]>,
    pub mountain_chains: Vec<[f32; 2]>,
    pub andean_chains: Vec<[f32; 2]>,
    pub hymalayan_chains: Vec<[f32; 2]>,
    pub trenches: Vec<[f32; 2]>,
    pub tectonic_plates_centers: Vec<[f32; 2]>,
    pub oceanic_plates: Vec<usize>,
    pub hotspots: Vec<[f32; 2]>,
    pub fresh_water: PartialMap<S, i32>,
    pub coastline: Option<HashSet<[usize; 2]>>,
    pub vegetation_density: PartialMap<S, i32>,
}

impl<S: MapShape> CompleteMap<S> {
    pub fn new(circunference: usize, height: usize) -> Self {
        Self {
            tectonic_plates: PartialMap::new(circunference, height),
            height: PartialMap::new(circunference, height),
            temperature: vec![],
            tectonic_plates_directions: vec![],
            winds: vec![],
            precipitation: vec![],
            annual_precipitation: PartialMap::new(0, 0),
            atm_pressure: vec![],
            climate: PartialMap::new(circunference, height),
            continentality: PartialMap::new(0, 0),
            ice_height: vec![],
            rivers: vec![],
            tectonic_edges: vec![],
            mountain_chains: vec![],
            andean_chains: vec![],
            hymalayan_chains: vec![],
            trenches: vec![],
            tectonic_plates_centers: vec![],
            oceanic_plates: vec![],
            hotspots: vec![],
            fresh_water: PartialMap::new(0, 0),
            coastline: None,
            vegetation_density: PartialMap::new(0, 0),
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), Error> {
        let serialized = bincode::serialize(&self).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(&serialized)?;
        Ok(())
    }
}

pub fn load<S: MapShape + for<'a> Deserialize<'a>>(filename: &str) -> CompleteMap<S> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let deserialized: CompleteMap<S> = bincode::deserialize(&buffer).unwrap();

    return deserialized;
}
