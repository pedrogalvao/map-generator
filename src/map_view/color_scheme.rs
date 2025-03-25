use crate::pipeline_steps::climate::Climate;
use image::{open, DynamicImage, GenericImageView, Rgba};
use lazy_static::lazy_static;
use rand::Rng;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, hash::Hash};

fn generate_random_color() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    Rgba([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), 255])
}

pub trait ColorScheme<T: Clone> {
    fn get(&self, value: T) -> Rgba<u8>;
}

fn rgba_from_str(s: &str) -> Result<Rgba<u8>, String> {
    if s.len() != 9 {
        return Err("Invalid RGBA string".into());
    }
    let r = u8::from_str_radix(&s[3..5], 16).map_err(|_| "Invalid red value")?;
    let g = u8::from_str_radix(&s[5..7], 16).map_err(|_| "Invalid green value")?;
    let b = u8::from_str_radix(&s[7..9], 16).map_err(|_| "Invalid blue value")?;
    let a = u8::from_str_radix(&s[1..3], 16).map_err(|_| "Invalid transparency value")?;
    Ok(Rgba([r, g, b, a]))
}

fn deserialize_rgba_tuple_vec<'de, D>(deserializer: D) -> Result<Vec<(i32, Rgba<u8>)>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_vec: Vec<(i32, String)> = Vec::deserialize(deserializer)?;
    let mut result = Vec::with_capacity(raw_vec.len());

    for (point, rgba_str) in raw_vec {
        let rgba = rgba_from_str(&rgba_str).map_err(de::Error::custom)?;
        result.push((point, rgba));
    }

    Ok(result)
}

fn deserialize_rgba_tuple_vecf32<'de, D>(deserializer: D) -> Result<Vec<(f32, Rgba<u8>)>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_vec: Vec<(f32, String)> = Vec::deserialize(deserializer)?;
    let mut result = Vec::with_capacity(raw_vec.len());

    for (point, rgba_str) in raw_vec {
        let rgba = rgba_from_str(&rgba_str).map_err(de::Error::custom)?;
        result.push((point, rgba));
    }

    Ok(result)
}

#[derive(Clone, Deserialize)]
pub struct GradientColorScheme {
    #[serde(deserialize_with = "deserialize_rgba_tuple_vec")]
    pub points: Vec<(i32, Rgba<u8>)>,
}

impl ColorScheme<i32> for GradientColorScheme {
    fn get(&self, value: i32) -> Rgba<u8> {
        let mut color: Rgba<u8> = self.points[0].1;
        for (i, (point_value, point_color)) in self.points.iter().enumerate() {
            if i + 1 < self.points.len() {
                let next_value = self.points[i + 1].0;
                let next_color = self.points[i + 1].1;
                if value >= *point_value && value <= next_value {
                    for k in 0..4 {
                        let weight1 = next_value - value;
                        let weight2 = value - point_value;
                        color[k] = ((point_color[k] as i32 * weight1
                            + next_color[k] as i32 * weight2)
                            / (weight1 + weight2)) as u8;
                    }
                    return color;
                }
            } else if value > *point_value {
                return self.points[self.points.len() - 1].1;
            }
        }
        return color;
    }
}

#[derive(Clone, Deserialize)]
pub struct GradientColorSchemef32 {
    #[serde(deserialize_with = "deserialize_rgba_tuple_vecf32")]
    pub points: Vec<(f32, Rgba<u8>)>,
}
impl ColorScheme<f32> for GradientColorSchemef32 {
    fn get(&self, value: f32) -> Rgba<u8> {
        let mut color: Rgba<u8> = self.points[0].1;
        for (i, (point_value, point_color)) in self.points.iter().enumerate() {
            if i + 1 < self.points.len() {
                let next_value = self.points[i + 1].0 as f32;
                let next_color = self.points[i + 1].1;
                if value >= *point_value as f32 && value <= next_value {
                    for k in 0..4 {
                        let weight1 = (next_value - value) as f32;
                        let weight2 = (value - point_value) as f32;
                        color[k] = ((point_color[k] as f32 * weight1
                            + next_color[k] as f32 * weight2)
                            / (weight1 + weight2)) as u8;
                    }
                    return color;
                }
            } else if value > *point_value {
                return self.points[self.points.len() - 1].1;
            }
        }
        return color;
    }
}

#[derive(Clone)]
pub struct TextureColorScheme {
    points: Vec<(i32, Option<DynamicImage>)>,
}

impl TextureColorScheme {
    pub fn get(&self, value: i32, x: u32, y: u32) -> Rgba<u8> {
        let texture_opt = &self.points[0].1;
        for (i, (point_value, texture_opt)) in self.points.iter().enumerate() {
            if i + 1 < self.points.len() {
                let next_value = self.points[i + 1].0;
                if value >= *point_value && value <= next_value {
                    if let Some(texture) = texture_opt {
                        let tex_height = texture.height();
                        let tex_width = texture.width();
                        return texture.get_pixel(x % tex_width, y % tex_height);
                    } else {
                        return Rgba([0, 0, 0, 0]);
                    }
                }
            } else if value > *point_value {
                if let Some(texture) = &self.points[self.points.len() - 1].1 {
                    let tex_height = texture.height();
                    let tex_width = texture.width();
                    return texture.get_pixel(x % tex_width, y % tex_height);
                } else {
                    return Rgba([0, 0, 0, 0]);
                }
            }
        }
        if let Some(texture) = texture_opt {
            let tex_height = texture.height();
            let tex_width = texture.width();
            return texture.get_pixel(x % tex_width, y % tex_height);
        } else {
            return Rgba([0, 0, 0, 0]);
        }
    }
}

#[derive(Clone)]
pub struct CategoryColorScheme {
    pub color_map: HashMap<usize, Rgba<u8>>,
}

impl<'de> Deserialize<'de> for CategoryColorScheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            color_map: HashMap<usize, [u8; 4]>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let color_map = helper
            .color_map
            .into_iter()
            .map(|(k, arr)| (k, Rgba(arr)))
            .collect();

        Ok(CategoryColorScheme { color_map })
    }
}

impl Serialize for CategoryColorScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper {
            color_map: HashMap<usize, [u8; 4]>,
        }

        let helper = Helper {
            color_map: self
                .color_map
                .iter()
                .map(|(&k, rgba)| (k, rgba.0))
                .collect(),
        };

        helper.serialize(serializer)
    }
}

impl CategoryColorScheme {
    pub fn new_random(n_categories: u32) -> Self {
        let mut color_map = HashMap::new();
        for i in 0..n_categories {
            color_map.insert(i as usize, generate_random_color());
        }
        Self { color_map }
    }
}

#[derive(Clone)]
pub struct ClimateColorScheme {}

impl ColorScheme<Climate> for ClimateColorScheme {
    fn get(&self, value: Climate) -> Rgba<u8> {
        match value {
            Climate::Ocean => Rgba([0, 0, 0, 0]),
            Climate::Tropical => Rgba([0, 0, 255, 255]),
            Climate::Monsoon => Rgba([0, 119, 255, 255]),
            Climate::Savanah => Rgba([70, 169, 255, 255]),
            Climate::HotDesert => Rgba([255, 0, 0, 255]),
            Climate::HotSemiarid => Rgba([245, 163, 0, 255]),
            Climate::ColdDesert => Rgba([255, 150, 150, 255]),
            Climate::ColdSemiarid => Rgba([255, 219, 99, 255]),
            Climate::HotMediterranean => Rgba([255, 255, 0, 255]),
            Climate::WarmMediterranean => Rgba([228, 208, 0, 255]),
            Climate::ColdMediterranean => Rgba([198, 199, 0, 255]),
            Climate::HumidSubtropical => Rgba([198, 255, 78, 255]), // Cfa
            Climate::SubtropicalMonsoon => Rgba([150, 255, 150, 255]), // Cwa
            Climate::Oceanic => Rgba([99, 199, 100, 255]),          // Cwb
            Climate::SubarcticOceanic => Rgba([50, 150, 51, 255]),  // Cwc
            Climate::HotHumidContinental => Rgba([0, 255, 255, 255]), // Dfa
            Climate::HumidContinental => Rgba([55, 200, 255, 255]), // Dfb
            Climate::MonsoonContinental => Rgba([171, 177, 255, 255]), // Dwa
            Climate::HotMediterraneanContinental => Rgba([255, 0, 255, 255]), // Dsa
            Climate::ColdMediterraneanContinental => Rgba([200, 0, 200, 255]), // Dsb
            Climate::Subarctic => Rgba([0, 126, 125, 255]),         // Dfc
            Climate::SevereSubarctic => Rgba([0, 69, 94, 255]),     // Dfd
            Climate::Tundra => Rgba([178, 178, 178, 255]),
            Climate::Glaciar => Rgba([104, 104, 104, 255]),
            _ => Rgba([0, 0, 0, 255]),
        }
    }
}

impl ColorScheme<usize> for CategoryColorScheme {
    fn get(&self, value: usize) -> Rgba<u8> {
        return *self.color_map.get(&value).unwrap();
    }
}

lazy_static! {
    pub static ref DEFAULT_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (-5000, Rgba([0, 0, 50, 255])),
            (0, Rgba([0, 0, 255, 255])),
            (1, Rgba([0, 100, 0, 255])),
            (1000, Rgba([0, 255, 0, 255])),
            (2000, Rgba([255, 255, 0, 255])),
            (5000, Rgba([255, 0, 0, 255])),
            (8000, Rgba([255, 255, 255, 255])),
        ]
    };
    pub static ref HIGH_CONTRAST_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (-8000, Rgba([0, 0, 0, 255])),
            (-4000, Rgba([0, 0, 50, 255])),
            (-1000, Rgba([0, 0, 255, 255])),
            (0, Rgba([30, 110, 255, 255])),
            (1, Rgba([0, 100, 0, 255])),
            (1000, Rgba([0, 255, 0, 255])),
            (2000, Rgba([255, 255, 0, 255])),
            (5000, Rgba([255, 0, 0, 255])),
            (8000, Rgba([255, 255, 255, 255])),
        ]
    };
    pub static ref LAND_WATER_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (0, Rgba([40, 130, 255, 255])),
            (1, Rgba([70, 255, 50, 255])),
        ]
    };
    pub static ref ATLAS_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (-1000, Rgba([90, 150, 255, 255])),
            (-200, Rgba([110, 180, 255, 255])),
            (0, Rgba([110, 180, 255, 255])),
            (1, Rgba([100, 255, 100, 255])),
            (1500, Rgba([100, 255, 100, 255])),
            (1501, Rgba([200, 255, 100, 255])),
            (4000, Rgba([200, 255, 100, 255])),
            (4001, Rgba([255, 150, 80, 255])),
        ]
    };
    pub static ref ATLAS_COLORS2: GradientColorScheme = GradientColorScheme {
        points: vec![
            (-1000, Rgba([90, 150, 255, 255])),
            (-300, Rgba([110, 180, 255, 255])),
            (0, Rgba([110, 180, 255, 255])),
            (1, Rgba([50, 200, 50, 255])),
            (200, Rgba([50, 200, 50, 255])),
            (201, Rgba([75, 230, 75, 255])),
            (500, Rgba([75, 230, 75, 255])),
            (501, Rgba([150, 255, 100, 255])),
            (1500, Rgba([150, 255, 100, 255])),
            (1501, Rgba([200, 200, 100, 255])),
            (3000, Rgba([200, 200, 100, 255])),
            (3001, Rgba([170, 120, 90, 255])),
            (5000, Rgba([170, 120, 90, 255])),
            (5001, Rgba([255, 255, 255, 255])),
        ]
    };
    pub static ref TRANSPARENCY_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![(0, Rgba([0, 0, 0, 0])), (1, Rgba([0, 0, 0, 255])),]
    };
    pub static ref PRECIPITATION_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![(0, Rgba([0, 0, 255, 0])), (500, Rgba([0, 0, 255, 255]))]
    };
    pub static ref ANNUAL_PRECIPITATION_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![(0, Rgba([0, 0, 255, 0])), (5000, Rgba([0, 0, 255, 255]))]
    };
    pub static ref WHITE: GradientColorScheme = GradientColorScheme {
        points: vec![(0, Rgba([255, 255, 255, 255])),]
    };
    pub static ref DARK_MOUNTAINS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (0, Rgba([0, 0, 0, 0])),
            (1000, Rgba([0, 0, 0, 0])),
            (1001, Rgba([0, 0, 0, 100])),
            (3000, Rgba([0, 0, 0, 100])),
            (3001, Rgba([0, 0, 0, 200])),
        ]
    };
    pub static ref TEMPERATURE_COLORS: GradientColorSchemef32 = GradientColorSchemef32 {
        points: vec![
            (-30.0, Rgba([0, 0, 255, 255])),
            (0.0, Rgba([255, 255, 255, 255])),
            (10.0, Rgba([0, 255, 0, 255])),
            (20.0, Rgba([255, 255, 0, 255])),
            (30.0, Rgba([255, 0, 0, 255])),
        ]
    };
    pub static ref CONTINENTALITY_COLORS: GradientColorSchemef32 = GradientColorSchemef32 {
        points: vec![
            (-30.0, Rgba([0, 0, 255, 255])),
            (0.0, Rgba([255, 255, 255, 255])),
            (10.0, Rgba([0, 255, 0, 255])),
            (20.0, Rgba([255, 255, 0, 255])),
            (30.0, Rgba([255, 0, 0, 255])),
        ]
    };
    pub static ref PRESSURE_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (-60, Rgba([255, 0, 0, 255])),
            (0, Rgba([255, 255, 255, 255])),
            (60, Rgba([0, 0, 255, 255])),
        ]
    };
    pub static ref OLD_STYLE_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (0, Rgba([220, 230, 255, 255])),
            (1, Rgba([255, 255, 200, 255])),
        ]
    };
    pub static ref OLD_STYLE_TRANSPARENT_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (0, Rgba([140, 110, 80, 155])),
            (1, Rgba([160, 140, 100, 155])),
        ]
    };
    pub static ref TEXTURE_SCHEME: TextureColorScheme = TextureColorScheme {
        points: vec![(0, Some(open("img/paper.jpg").unwrap())),]
    };
    pub static ref TREE_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (-1, Rgba([0, 0, 0, 0])),
            (0, Rgba([150, 220, 100, 0])),
            (100, Rgba([150, 220, 100, 50])),
            (500, Rgba([20, 80, 10, 255])),
        ]
    };
    pub static ref VEGETATION_COLORS: GradientColorScheme = GradientColorScheme {
        points: vec![
            (0, Rgba([200, 190, 170, 255])),
            (200, Rgba([160, 155, 120, 255])),
            (600, Rgba([92, 110, 80, 255])),
            (800, Rgba([70, 100, 65, 255])),
            (1000, Rgba([52, 85, 52, 255])),
        ]
    };
}
