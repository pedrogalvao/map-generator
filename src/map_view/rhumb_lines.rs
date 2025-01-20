use image::{ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{layer::MapViewLayer, projection::projection::Projection, util::color_over};

pub struct RhumbLinesLayer {
    pub color: Rgba<u8>,
    center_points: Vec<[f32; 2]>,
    lines_per_point: u32,
    in_land: bool,
}

impl RhumbLinesLayer {
    pub fn default() -> Self {
        Self {
            center_points: vec![
                [0.0, 0.0],
                [60.0, 60.0],
                [-60.0, -60.0],
                [0.0, 180.0],
                [-60.0, 120.0],
                [60.0, -120.0],
            ],
            lines_per_point: 16,
            color: Rgba([0, 0, 0, 60]),
            in_land: false,
        }
    }

    fn add_line<P: Projection, S: MapShape>(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        center_point: [f32; 2],
        angle: f32,
        complete_map: &CompleteMap<S>,
    ) {
        let mut prev_img_x = 0;
        let mut prev_img_y = 0;
        let angle_sin = angle.to_radians().sin();
        let angle_cos = angle.to_radians().cos();
        for i in 0..4 * base_img.width() {
            let latitude =
                center_point[0] + angle_cos * i as f32 * (180.0 / base_img.width() as f32);
            let longitude =
                center_point[1] + angle_sin * i as f32 * (180.0 / base_img.width() as f32);
            if !self.in_land && complete_map.height.get(latitude, longitude) > 0 {
                continue;
            }
            if let Some([img_x, img_y]) = projection.map_to_img_coords(
                latitude,
                longitude,
                base_img.width(),
                base_img.height(),
                center_longitude,
            ) {
                if latitude >= 90.0 || latitude <= -90.0 {
                    break;
                }
                if angle == 0.0 && longitude >= center_point[1] + 360.0 {
                    break;
                }
                if img_x == prev_img_x && img_y == prev_img_y {
                    continue;
                }
                if img_x < base_img.height() && img_y < base_img.width() {
                    let color = base_img.get_pixel_mut(img_y as u32, img_x as u32);
                    let new_color = color_over(color, &self.color);
                    base_img.put_pixel(img_y as u32, img_x as u32, new_color);
                }
                prev_img_x = img_x;
                prev_img_y = img_y;
            } else {
                break;
            }
        }
    }
}

impl<P, S> MapViewLayer<P, S> for RhumbLinesLayer
where
    S: MapShape,
    P: Projection,
{
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {
        for center_point in &self.center_points {
            for i in 0..self.lines_per_point {
                self.add_line(
                    base_img,
                    projection,
                    center_longitude,
                    *center_point,
                    (i * 360 / self.lines_per_point) as f32,
                    complete_map,
                )
            }
        }
    }
}
