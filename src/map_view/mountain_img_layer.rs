use std::time::Instant;

use image::{open, DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::{layer::MapViewLayer, projection::projection::Projection, util::color_over};

pub struct MountainImgLayer {}

impl MountainImgLayer {
    pub fn new() -> Self {
        Self {}
    }
}

fn draw_icon(
    base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    icon: &DynamicImage,
    position: [u32; 2],
) {
    let icon_height: u32 = icon.height();
    let icon_width = icon.width();
    for i in 0..icon_height {
        for j in 0..icon_width {
            let img_x = i + position[0] - icon_height;
            let img_y = j + position[1] - icon_width / 2;
            if img_x < base_img.height() && img_y < base_img.width() {
                let layer_color: Rgba<u8> = icon.get_pixel(j, i);
                let original_color = base_img.get_pixel(img_y, img_x);
                let result_color = color_over(original_color, &layer_color);
                base_img.put_pixel(img_y, img_x, result_color);
            }
        }
    }
    // base_img.put_pixel(position[1], position[0], Rgba([255,0,255,255]));
}

fn check_space_for_icon<P: Projection, S: MapShape>(
    complete_map: &CompleteMap<S>,
    projection: &P,
    img_x: u32,
    img_y: u32,
    base_img_width: u32,
    base_img_height: u32,
    center_longitude: f32,
    icon_height: u32,
    icon_width: u32,
) -> bool {
    if None
        == projection.img_to_map_coords(
            img_x - icon_height,
            img_y - icon_width / 2,
            base_img_width,
            base_img_height,
            center_longitude,
        )
    {
        return false;
    } else if None
        == projection.img_to_map_coords(
            img_x - icon_height,
            img_y + icon_width / 2,
            base_img_width,
            base_img_height,
            center_longitude,
        )
    {
        return false;
    }
    if let Some([lat, lon]) = projection.img_to_map_coords(
        img_x,
        img_y + icon_width / 2,
        base_img_width,
        base_img_height,
        center_longitude,
    ) {
        let height = complete_map.height.get(lat, lon).clone();
        if height <= 0 {
            return false;
        }
    } else {
        return false;
    }
    if let Some([lat, lon]) = projection.img_to_map_coords(
        img_x,
        img_y - icon_width / 2,
        base_img_width,
        base_img_height,
        center_longitude,
    ) {
        let height = complete_map.height.get(lat, lon).clone();
        if height <= 0 {
            return false;
        }
    } else {
        return false;
    }
    return true;
}

impl MountainImgLayer {
    fn choose_icon_positions<P: Projection, S: MapShape>(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) -> Vec<[u32; 2]> {
        let mut positions = vec![];

        let height = base_img.height();
        let width = base_img.width();

        let mut img_x0: i32 = -(base_img.height() as i32);
        while img_x0 < height as i32 - 20 {
            img_x0 += 15;
            let mut img_x;
            let mut img_y;
            if img_x0 <= 0 {
                img_x = 20;
                img_y = (15 - img_x0 * 6) as u32;
            } else {
                img_x = img_x0 as u32;
                img_y = 15;
            }
            while img_y < width - 20 && img_x < height - 20 {
                img_y += 6;
                img_x += 1;
                if let Some([lat, long]) = projection.img_to_map_coords(
                    img_x,
                    img_y,
                    base_img.width(),
                    base_img.height(),
                    center_longitude,
                ) {
                    let h = complete_map.height.get(lat, long).clone();
                    let [icon_height, icon_width];
                    if h > 2300 {
                        icon_height = 40;
                        icon_width = 40;
                    } else if h > 1300 {
                        icon_height = 19;
                        icon_width = 30;
                    } else if h > 800 {
                        icon_height = 13;
                        icon_width = 25;
                    } else {
                        continue;
                    }
                    if check_space_for_icon(
                        complete_map,
                        projection,
                        img_x,
                        img_y,
                        base_img.width(),
                        base_img.height(),
                        center_longitude,
                        icon_height,
                        icon_width,
                    ) {
                        positions.push([img_x, img_y]);
                        img_y += 2 * icon_width / 3;
                        img_x += icon_width / 9;
                    }
                }
            }
        }
        return positions;
    }
}

impl<P: Projection, S: MapShape> MapViewLayer<P, S> for MountainImgLayer {
    fn draw_layer(
        &self,
        base_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        projection: &P,
        center_longitude: f32,
        complete_map: &CompleteMap<S>,
    ) {

        let Ok(icon) = open("img2/mountain1.png") else {
            println!("Error: mountain1.png not found");
            return;
        };
        let Ok(icon2) = open("img2/mountain2.png") else {
            println!("Error: mountain2.png not found");
            return;
        };
        let Ok(icon3) = open("img2/hill.png") else {
            println!("Error: hill.png not found");
            return;
        };

        let icon_positions =
            self.choose_icon_positions(base_img, projection, center_longitude, complete_map);

        for [x, y] in icon_positions {
            if let Some([lat, long]) = projection.img_to_map_coords(
                x,
                y,
                base_img.width(),
                base_img.height(),
                center_longitude,
            ) {
                let h = complete_map.height.get(lat, long).clone();
                if h > 2300 {
                    draw_icon(base_img, &icon, [x, y]);
                } else if h > 1300 {
                    draw_icon(base_img, &icon2, [x, y]);
                } else if h > 800 {
                    draw_icon(base_img, &icon3, [x, y]);
                }
            }
        }
    }
}
