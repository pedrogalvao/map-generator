use std::{fs, time::Instant};

use image::Rgba;

use crate::{
    complete_map::CompleteMap,
    map_view::{
        color_scheme::DEFAULT_COLORS, map_view::MapView,
        parallels_meridians_layer::ParallelsMeridiansLayer, partial_map_layer::PartialMapLayer,
        projection::equirectangular::Equirectangular, rivers_layer::RiversLayer,
    },
    pipeline_steps::pipeline_step::PipelineStep,
    shapes::map_shape::MapShape,
};

pub struct MapPipeline<S: MapShape> {
    pub circunference: usize,
    pub height: usize,
    pub steps: Vec<Box<dyn PipelineStep<S> + 'static>>,
}

impl<S: MapShape + 'static> MapPipeline<S> {
    pub fn new() -> Self {
        Self {
            circunference: 400,
            height: 200,
            steps: vec![],
        }
    }

    pub fn add_step(&mut self, step: impl PipelineStep<S> + 'static) {
        self.steps.push(Box::new(step));
    }

    pub fn execute(&mut self) -> CompleteMap<S> {
        let mut complete_map = CompleteMap::new(self.circunference, self.height);
        let start = Instant::now();
        for (i, step) in self.steps.iter_mut().enumerate() {
            let t1 = Instant::now();
            complete_map = step.apply(&mut complete_map);
            let t2 = Instant::now();
            dbg!("step", i);
            dbg!("duration: ", t2 - t1);
            draw_step(&complete_map, i);
            let t3 = Instant::now();
        }
        let end = Instant::now();
        println!("Total generation time: {}", end - start);
        return complete_map;
    }
}

fn draw_step<S: MapShape + 'static>(cmap: &CompleteMap<S>, i: usize) {
    let output_path = "out/pipeline/";
    if !std::path::Path::new(&output_path).exists() {
        let _ = fs::create_dir_all(output_path);
    }
    let mut mv: MapView<Equirectangular, S> = MapView::new();

    let mut pm_layer = ParallelsMeridiansLayer::default();
    pm_layer.color = Rgba([255, 255, 255, 150]);
    mv.resolution = [cmap.height.circunference, cmap.height.height];
    mv.center[1] = 0.0;
    mv.layers = vec![
        Box::new(PartialMapLayer::new(
            |m| &m.height,
            Box::new(DEFAULT_COLORS.clone()),
        )),
        Box::new(RiversLayer::default()),
        Box::new(pm_layer),
    ];
    mv.draw(
        &cmap,
        ("out/pipeline/".to_owned() + &i.to_string() + ".png").as_str(),
    );
}
