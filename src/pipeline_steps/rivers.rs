use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{complete_map::CompleteMap, partial_map::PartialMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Clone, Serialize, Deserialize)]
pub struct RiverPoint {
    pub position: [usize; 2],
    pub volume: u32,
    pub direction: [i32; 2],
}

pub type River = Vec<RiverPoint>;

fn contains_point(river: &River, position: [usize; 2]) -> bool {
    for p in river {
        if p.position == position {
            return true;
        }
    }
    return false;
}

fn _close_to_point<S: MapShape>(
    river: &River,
    position: [usize; 2],
    height_map: &PartialMap<S, i32>,
) -> bool {
    for row in height_map.get_pixel_neighbours_coords(position, 1) {
        for [x, y] in row {
            if contains_point(river, [x, y]) {
                return true;
            }
        }
    }
    return false;
}

#[derive(Debug)]
pub struct CreateRivers {}

fn get_next_position<S: MapShape>(
    tmp_height_map: &PartialMap<S, i32>,
    position: [usize; 2],
) -> ([usize; 2], [i32; 2]) {
    let mut distance = 1;
    let mut next_position = [0, 0];
    let mut direction = [0, 0];
    while next_position == [0, 0] || next_position == position && distance < 10 {
        let neighbours_coords = tmp_height_map.get_pixel_neighbours_coords(position, distance);
        let mut minimum = 100000000;
        for (i, row) in neighbours_coords.iter().enumerate() {
            for (j, [x2, y2]) in row.iter().enumerate() {
                let value = tmp_height_map.values[*x2][*y2];
                if value <= minimum {
                    minimum = value;
                    direction = [i as i32 - distance as i32, j as i32 - distance as i32];
                    next_position = [*x2, *y2];
                }
            }
        }
        distance += 1;
    }
    return (next_position, direction);
}

fn shortcut<S: MapShape>(input_map: &CompleteMap<S>, river: &River) -> River {
    let mut new_river = vec![];
    let mut river_idx = 0;
    while river_idx < river.len() {
        let point1 = river[river_idx].clone();
        new_river.push(point1.clone());
        for j in (river_idx + 3)..river.len() {
            let point2 = river[j].clone();
            'neighbors_cycle: for row in input_map
                .height
                .get_pixel_neighbours_coords(point1.position, 2)
            {
                for neighbor_position in row {
                    if point1.position != neighbor_position && neighbor_position == point2.position
                    {
                        river_idx = j - 1;
                        break 'neighbors_cycle;
                    }
                }
            }
        }
        river_idx += 1;
    }
    return new_river;
}

fn make_river<S: MapShape>(input_map: &CompleteMap<S>, start_point: [usize; 2]) -> River {
    let mut river: River = vec![];
    let mut position = start_point;
    let mut tmp_height_map = input_map.height.clone();

    let mut temp_fresh_water = input_map.fresh_water.clone();

    let mut direction;
    let mut next_position;
    loop {
        tmp_height_map.values[position[0]][position[1]] += 20;
        temp_fresh_water.values[position[0]][position[1]] = 1;
        let [x, y] = position;
        if tmp_height_map.values[x][y] < 0 {
            return river;
        }
        (next_position, direction) = get_next_position(&tmp_height_map, position);
        let rp = RiverPoint {
            position: position,
            volume: 1,
            direction: direction,
        };
        river.push(rp.clone());
        if next_position[0] == position[0] && next_position[1] == position[1] {
            return river;
        } else if next_position[0] == 0 && next_position[1] == 0 {
            return vec![];
        } else if tmp_height_map.values[next_position[0]][next_position[1]] <= 0 {
            // river ends in sea level
            let rp = RiverPoint {
                position: next_position,
                volume: 1,
                direction: direction,
            };
            river.push(rp.clone());
            return river;
        } else {
            // check if the river should end in another river
            for [x2, y2] in input_map
                .fresh_water
                .get_pixel_neighbours_coords(position, 1)
                .iter()
                .flatten()
            {
                let neighbor_value = input_map.fresh_water.values[*x2][*y2];
                if neighbor_value > 0 {
                    let rp: RiverPoint = RiverPoint {
                        position: [*x2, *y2],
                        volume: 1,
                        direction: direction,
                    };
                    if contains_point(&river, [*x2, *y2]) {
                        // same river, ignore
                        continue;
                    }
                    river.push(rp);
                    return river;
                }
            }
            // for river2 in &input_map.rivers {
            //     for neighbor in input_map
            //         .height
            //         .get_pixel_neighbours_coords(position, 1)
            //         .iter()
            //         .flatten()
            //     {
            //         if contains_point(&river2, neighbor.clone()) {
            //             let rp: RiverPoint = RiverPoint {
            //                 position: neighbor.clone(),
            //                 volume: 1,
            //                 direction: direction,
            //             };
            //             river.push(rp);
            //             return river;
            //         }
            //     }
            // }
        }
        position = next_position;
    }
}

fn update_fresh_water<S: MapShape>(input_map: &mut CompleteMap<S>, river: &River) {
    for rp in river {
        input_map.fresh_water.values[rp.position[0]][rp.position[1]] = 1;
    }
}

fn erosion<S: MapShape>(output_map: &mut CompleteMap<S>, river: &River) {
    let mut h = 99990;
    for rp in river[0..river.len() - 2].iter() {
        // output_map.height.values[rp.position[0]][rp.position[1]] = 0;
        output_map.height.values[rp.position[0]][rp.position[1]] -= 5;
        output_map.height.values[rp.position[0]][rp.position[1]] =
            output_map.height.values[rp.position[0]][rp.position[1]].max(1);
        output_map.height.values[rp.position[0]][rp.position[1]] =
            (0.96 * output_map.height.values[rp.position[0]][rp.position[1]] as f32) as i32;
        // output_map.height.values[rp.position[0]][rp.position[1]] = (output_map.height.values[rp.position[0]][rp.position[1]] as f32 * 0.95) as i32;
        output_map.height.values[rp.position[0]][rp.position[1]] =
            output_map.height.values[rp.position[0]][rp.position[1]].min(h);
        h = output_map.height.values[rp.position[0]][rp.position[1]].min(h);
        for row in output_map
            .height
            .get_pixel_neighbours_coords(rp.position, 1)
        {
            for p in row {
                output_map.height.values[p[0]][p[1]] =
                    (output_map.height.values[p[0]][p[1]] as f32 * 0.95).max(1.0) as i32;
            }
        }
    }
}

fn get_start_points_from_precipitation<S: MapShape>(input_map: &CompleteMap<S>) -> Vec<[usize; 2]> {
    let mut points = vec![];
    let mut precipitation_temp = input_map.annual_precipitation.clone();
    for x in 0..precipitation_temp.values.len() {
        for y in 0..precipitation_temp.values[x].len() {
            if x % 15 != 0 || y % 15 != 0 {
                continue;
            }
            let [latitude, longitude] = precipitation_temp.convert_coords(x, y);
            let height = input_map.height.get(latitude, longitude);
            if height > 500 {
                let max_temperature = input_map.temperature[0].get(latitude, longitude).max(
                    input_map.temperature[input_map.temperature.len() / 2].get(latitude, longitude),
                );
                if precipitation_temp.values[x][y] > 500 && max_temperature > 0.0 {
                    let [x_h, y_h] = input_map.height.convert_to_vec_coords(latitude, longitude);
                    points.push([x_h, y_h]);
                    for row in precipitation_temp.get_pixel_neighbours_coords([x, y], 6) {
                        for [i, j] in row {
                            precipitation_temp.values[i][j] = 0;
                        }
                    }
                }
            } else if precipitation_temp.values[x][y] > 1000 {
                let [x_h, y_h] = input_map.height.convert_to_vec_coords(latitude, longitude);
                points.push([x_h, y_h]);
                for row in precipitation_temp.get_pixel_neighbours_coords([x, y], 6) {
                    for [i, j] in row {
                        precipitation_temp.values[i][j] = 0;
                    }
                }
            }
        }
    }
    for x in 0..precipitation_temp.values.len() {
        for y in 0..precipitation_temp.values[x].len() {
            if x % 15 != 0 || y % 15 != 0 {
                continue;
            }
            let [latitude, longitude] = precipitation_temp.convert_coords(x, y);
            let height = input_map.height.get(latitude, longitude);
            if height > 100 {
                let max_temperature = input_map.temperature[0].get(latitude, longitude).max(
                    input_map.temperature[input_map.temperature.len() / 2].get(latitude, longitude),
                );
                if precipitation_temp.values[x][y] > 500 && max_temperature > 0.0 {
                    let [x_h, y_h] = input_map.height.convert_to_vec_coords(latitude, longitude);
                    points.push([x_h, y_h]);
                    for row in precipitation_temp.get_pixel_neighbours_coords([x, y], 5) {
                        for [i, j] in row {
                            precipitation_temp.values[i][j] = 0;
                        }
                    }
                }
            }
        }
    }
    dbg!("Number of river starting points: ", points.len());
    points
}

impl<S: MapShape> PipelineStep<S> for CreateRivers {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }
    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();
        output_map.fresh_water =
            PartialMap::new(output_map.height.circunference, output_map.height.height);
        if output_map.fresh_water.values[0].len() != input_map.height.values[0].len() {
            for i in 0..input_map.height.values.len() {
                output_map.fresh_water.values[i] = vec![0; input_map.height.values[i].len()];
            }
        }

        let start_points: Vec<[usize; 2]> = get_start_points_from_precipitation(input_map);
        'river_creation_loop: for start_point in start_points {
            if output_map.height.values[start_point[0]][start_point[1]] <= 0 {
                continue 'river_creation_loop;
            }
            for river in &output_map.rivers {
                let neighbors = output_map
                    .height
                    .get_pixel_neighbours_coords(start_point, 5);
                for neighbor in neighbors.iter().flatten() {
                    if contains_point(&river, neighbor.clone()) {
                        continue 'river_creation_loop;
                    }
                }
            }
            let river = make_river(&mut output_map, start_point);
            if river.len() < 3 {
                continue;
            }
            let shortcutted_river = shortcut(input_map, &river);
            update_fresh_water(&mut output_map, &shortcutted_river);
            erosion(&mut output_map, &shortcutted_river);
            output_map.rivers.push(shortcutted_river);
        }
        dbg!("Number of rivers: ", output_map.rivers.len());
        return output_map;
    }
}
