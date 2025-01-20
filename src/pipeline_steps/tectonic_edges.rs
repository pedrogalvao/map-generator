use std::sync::Arc;

use crate::{complete_map::CompleteMap, shapes::map_shape::MapShape};

use super::pipeline_step::PipelineStep;

#[derive(Debug)]
pub struct DefineTecEdges {}

impl DefineTecEdges {
    pub fn new() -> Self {
        DefineTecEdges {}
    }
}

fn scalar_prod(v1: &[f32; 2], v2: &[f32; 2]) -> f32 {
    return v1[0] * v2[0] + v1[1] * v2[1];
}

impl DefineTecEdges {
    fn process_edge_element<S: MapShape>(
        &self,
        x: usize,
        y: usize,
        input_map: &CompleteMap<S>,
        output_map: &mut CompleteMap<S>,
    ) {
        let plate1 = input_map.tectonic_plates.values[x][y];
        for (i, row) in input_map
            .tectonic_plates
            .get_pixel_neighbours_coords([x, y], 1)
            .iter()
            .enumerate()
        {
            for (j, [x2, y2]) in row.iter().enumerate() {
                let plate2 = input_map.tectonic_plates.values[*x2][*y2];
                if plate1 != plate2 {
                    let direction1 = input_map.tectonic_plates_directions[plate1];
                    let direction2 = input_map.tectonic_plates_directions[plate2];
                    let pixel_direction = [i as f32 - 1.0, j as f32 - 1.0];

                    let pixel_direction_norm =
                        ((pixel_direction[0]).powf(2.0) + (pixel_direction[1]).powf(2.0)).powf(0.5);
                    let pixel_direction_normalized = [
                        pixel_direction[0] / pixel_direction_norm,
                        pixel_direction[1] / pixel_direction_norm,
                    ];
                    let collision = scalar_prod(&direction1, &pixel_direction_normalized)
                        - scalar_prod(&pixel_direction_normalized, &direction2);
                    let [latitude, longitude] = input_map.tectonic_plates.convert_coords(x, y);
                    output_map.tectonic_edges.push([latitude, longitude]);
                    if collision > 0.5 {
                        let height1 = input_map.height.values[x][y];
                        let height2 = input_map.height.values[*x2][*y2];
                        if height1 > -200 {
                            if height1 > 0 && height2 > 0 {
                                let height3 = input_map.height.get(
                                    latitude - &pixel_direction_normalized[0],
                                    longitude - &pixel_direction_normalized[1],
                                );
                                let height4 = input_map.height.get(
                                    latitude - 2.0 * &pixel_direction_normalized[0],
                                    longitude - 2.0 * &pixel_direction_normalized[1],
                                );
                                if height3 > 0 && height4 > 0 {
                                    output_map.hymalayan_chains.push([
                                        latitude - pixel_direction_normalized[0],
                                        longitude - pixel_direction_normalized[1],
                                    ]);
                                } else {
                                    output_map.mountain_chains.push([
                                        latitude - pixel_direction_normalized[0],
                                        longitude - pixel_direction_normalized[1],
                                    ]);
                                }
                            }
                            if height1 > 0 && height2 < 0 {
                                let height3 = input_map.height.get(
                                    latitude - &pixel_direction_normalized[0],
                                    longitude - &pixel_direction_normalized[1],
                                );
                                let height4 = input_map.height.get(
                                    latitude - 2.0 * &pixel_direction_normalized[0],
                                    longitude - 2.0 * &pixel_direction_normalized[1],
                                );
                                if height3 > 0 && height4 > 0 {
                                    output_map.andean_chains.push([
                                        latitude - pixel_direction_normalized[0],
                                        longitude - pixel_direction_normalized[1],
                                    ]);
                                } else {
                                    output_map.mountain_chains.push([
                                        latitude - pixel_direction_normalized[0],
                                        longitude - pixel_direction_normalized[1],
                                    ]);
                                }
                            } else {
                                output_map.mountain_chains.push([
                                    latitude - pixel_direction_normalized[0],
                                    longitude - pixel_direction_normalized[1],
                                ]);
                            }
                        } else if height1 <= -200 && height2 <= -200 {
                            output_map.mountain_chains.push([
                                latitude - pixel_direction_normalized[0],
                                longitude - pixel_direction_normalized[1],
                            ]);
                        } else if height1 <= -200 && height2 > -200 {
                            output_map.trenches.push([
                                latitude - pixel_direction_normalized[0],
                                longitude - pixel_direction_normalized[1],
                            ]);
                        }
                    }
                }
            }
        }
    }
}

impl<S: MapShape> PipelineStep<S> for DefineTecEdges {
    fn process_element(&self, _x: usize, _y: usize, _input_map: Arc<&CompleteMap<S>>) -> i32 {
        todo!()
    }

    fn apply(&self, input_map: &CompleteMap<S>) -> CompleteMap<S> {
        let mut output_map = input_map.clone();

        for x in 0..output_map.tectonic_plates.values.len() {
            for y in 0..output_map.tectonic_plates.values[x].len() {
                self.process_edge_element(x, y, input_map, &mut output_map);
            }
        }

        return output_map;
    }
}
