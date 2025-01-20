use std::env;
use std::{fs::File, io::Read, time::Instant};

use complete_map::{load, CompleteMap};
use draw_functions::{draw_all, draw_all_equirectangular};

use shapes::{cylinder::Cylinder, flat::Flat, globe::Globe};

use crate::configuration::{Configuration, ShapeEnum};
use crate::recipe::standard_recipe;

mod complete_map;
mod configuration;
mod draw_functions;
mod map_pipeline;
mod map_view;
mod partial_map;
mod pipeline_steps;
mod recipe;
mod shapes;

fn generate_map(config_file: &str) {
    let start = Instant::now();

    let mut file = File::open(config_file).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: Configuration = serde_json::from_str(&data).unwrap();

    match config.shape {
        ShapeEnum::Cylinder => {
            let cmap: CompleteMap<Cylinder> = standard_recipe(&config).execute();
            let end = Instant::now();
            let generation_time = (end - start).as_secs_f32();
            println!("generation time: {generation_time}");
            let _ = cmap.save("map.bin");
            draw_all_equirectangular(&cmap);
        }
        ShapeEnum::Globe => {
            let cmap: CompleteMap<Globe> = standard_recipe(&config).execute();
            let end = Instant::now();
            let generation_time = (end - start).as_secs_f32();
            println!("generation time: {generation_time}");
            let _ = cmap.save("map.bin");
            draw_all(&cmap);
        }
        ShapeEnum::Flat => {
            let cmap: CompleteMap<Flat> = standard_recipe(&config).execute();
            let end = Instant::now();
            let generation_time = (end - start).as_secs_f32();
            println!("generation time: {generation_time}");
            let _ = cmap.save("map.bin");
            draw_all_equirectangular(&cmap);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Generating map from config.json");
        generate_map("config.json");
    }

    match args[1].as_str() {
        "load" => {
            println!("Loading map");
            let start = Instant::now();
            let cmap = load::<Globe>(args[2].as_str());
            let end = Instant::now();
            let loading_time = (end - start).as_secs_f32();
            println!("loading time: {loading_time}");
            cmap.height.save_as_img("height.png", -10000, 10000);
            draw_all(&cmap);
        }
        "generate" => {
            println!("Generating map from {}", args[2].as_str());
            generate_map(args[2].as_str());
        }
        _ => println!("unrecognized option: {}", args[1]),
    }
}
