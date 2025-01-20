use std::{f32::consts::PI, fmt};

use image::Rgba;
use serde::{
    de::{self, Visitor},
    Deserializer, Serializer,
};

pub fn color_over(original_color: &Rgba<u8>, layer_color: &Rgba<u8>) -> Rgba<u8> {
    let mut result_color = original_color.clone();
    for k in 0..3 {
        result_color[k] = ((original_color[k] as u32 * (255 - layer_color[3] as u32)
            + layer_color[k] as u32 * (layer_color[3] as u32))
            / 255) as u8;
    }
    result_color[3] = result_color[3].max(layer_color[3]);
    return result_color;
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}

pub fn spin_coords(latitude: f32, longitude: f32, phi: f32, theta: f32) -> [f32; 2] {
    let phi_rad = degrees_to_radians(phi);
    let theta_rad = degrees_to_radians(theta);
    let latitude_rad = degrees_to_radians(latitude);
    let longitude_rad = degrees_to_radians(longitude);

    // Convert spherical coordinates to Cartesian coordinates
    let x = longitude_rad.cos() * latitude_rad.cos();
    let y = longitude_rad.sin() * latitude_rad.cos();
    let z = latitude_rad.sin();

    // Apply rotation
    let new_x = x * theta_rad.cos() * phi_rad.cos() - y * phi_rad.sin()
        + z * theta_rad.sin() * phi_rad.cos();
    let new_y = x * theta_rad.cos() * phi_rad.sin()
        + y * phi_rad.cos()
        + z * theta_rad.sin() * phi_rad.sin();
    let new_z = -x * theta_rad.sin() + z * theta_rad.cos();

    // Convert Cartesian coordinates back to spherical coordinates
    let radius = (new_x.powi(2) + new_y.powi(2) + new_z.powi(2)).sqrt();
    let radius_xy = (new_x.powi(2) + new_y.powi(2)).sqrt();
    let mut new_latitude = radians_to_degrees((radius_xy / radius).acos());
    if new_z < 0.0 {
        new_latitude = -new_latitude;
    }
    let new_longitude = radians_to_degrees(new_y.atan2(new_x));

    [new_latitude, new_longitude]
}

pub fn reverse_spin_coords(latitude: f32, longitude: f32, phi: f32, theta: f32) -> [f32; 2] {
    let [latitude2, longitude2] = spin_coords(latitude, longitude, -phi, 0.0);
    let [new_latitude, new_longitude] = spin_coords(latitude2, longitude2, 0.0, -theta);

    [new_latitude, new_longitude]
}

pub fn _serialize_rgba<S>(color: &Rgba<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_color = format!(
        "{:02X}{:02X}{:02X}{:02X}",
        color[0], color[1], color[2], color[3]
    );
    serializer.serialize_str(&hex_color)
}

pub fn deserialize_rgba<'de, D>(deserializer: D) -> Result<Rgba<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct RgbaVisitor;

    impl<'de> Visitor<'de> for RgbaVisitor {
        type Value = Rgba<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representing a hex color code with 9 digits (#RRGGBBAA)")
        }

        fn visit_str<E>(self, value: &str) -> Result<Rgba<u8>, E>
        where
            E: de::Error,
        {
            if value.len() != 9 {
                return Err(E::custom(format!(
                    "invalid length for RGBA hex code: {}",
                    value.len()
                )));
            }
            let r = u8::from_str_radix(&value[3..5], 16).map_err(E::custom)?;
            let g = u8::from_str_radix(&value[5..7], 16).map_err(E::custom)?;
            let b = u8::from_str_radix(&value[7..9], 16).map_err(E::custom)?;
            let a = u8::from_str_radix(&value[1..3], 16).map_err(E::custom)?;
            Ok(Rgba([r, g, b, a]))
        }
    }

    deserializer.deserialize_str(RgbaVisitor)
}
