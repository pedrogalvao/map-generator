#[macro_use]
extern crate rocket;

use std::{collections::HashMap, sync::Mutex, time::Instant};

use complete_map::{load, CompleteMap};
use configuration::{ClimateConfiguration, Configuration, ShapeEnum};
use map_view::view_config::{draw_with_config, img_from_config, ViewConfiguration};
use pipeline_steps::{
    calculate_climate::CalculateClimate, define_coastlines::DefineCoastline,
    height_noise::HeightNoise, hydraulic_erosion::HydraulicErosion, pipeline_step::PipelineStep,
    resize::Resize, smooth::Smooth, translation_noise::TranslationNoise, water_level::WaterLevel,
};
use recipe::{recipe_from_image, standard_recipe};
use rocket::{
    http::ContentType,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use shapes::{cylinder::Cylinder, flat::Flat, globe::Globe};

mod complete_map;
mod configuration;
mod draw_functions;
mod map_pipeline;
mod map_view;
mod partial_map;
mod pipeline_steps;
mod recipe;
mod shapes;

enum CompleteMapEnum {
    Cylinder(CompleteMap<Cylinder>),
    Globe(CompleteMap<Globe>),
    Flat(CompleteMap<Flat>),
}

type MapStore = Mutex<HashMap<String, CompleteMapEnum>>;

macro_rules! apply_operation {
    ($pipeline_step:expr, $key:expr, $store:expr) => {{
        let mut locked_store = $store.lock().unwrap();
        let Some(cmap_enum): Option<CompleteMapEnum> = locked_store.remove(&$key) else {
            return Json(Message {
                message: format!("no map found"),
            });
        };
        match cmap_enum {
            CompleteMapEnum::Globe(cmap) => {
                locked_store.insert($key, CompleteMapEnum::Globe($pipeline_step.apply(&cmap)));
            }
            CompleteMapEnum::Cylinder(cmap) => {
                locked_store.insert($key, CompleteMapEnum::Cylinder($pipeline_step.apply(&cmap)));
            }
            CompleteMapEnum::Flat(cmap) => {
                locked_store.insert($key, CompleteMapEnum::Flat($pipeline_step.apply(&cmap)));
            }
        };
    }};
}

#[derive(Deserialize)]
struct RequestData<T> {
    world_name: String,
    params: T,
}

fn generate_map(req_data: &GenerationRequest, store: &State<MapStore>) {
    let start = Instant::now();
    macro_rules! generate_map_with_shape {
        ($shape:ty, $cmap_enum:ident, $config:expr) => {{
            let cmap: CompleteMap<$shape> = standard_recipe(&req_data.world_config).execute();
            let end = Instant::now();
            let generation_time = (end - start).as_secs_f32();
            println!("generation time: {generation_time}");
            let _ = cmap.save("map.bin");
            store.lock().unwrap().insert(
                req_data.world_name.clone(),
                CompleteMapEnum::$cmap_enum(cmap),
            );
        }};
    }
    match req_data.world_config.shape {
        ShapeEnum::Cylinder => generate_map_with_shape!(Cylinder, Cylinder, config),
        ShapeEnum::Globe => generate_map_with_shape!(Globe, Globe, config),
        ShapeEnum::Flat => generate_map_with_shape!(Flat, Flat, config),
    }
}

fn generate_map_from_image(
    world_name: String,
    filepath: String,
    shape: &ShapeEnum,
    store: &State<MapStore>,
) {
    let start = Instant::now();
    macro_rules! generate_map_with_shape {
        ($shape:ty, $cmap_enum:ident, $config:expr) => {{
            let cmap: CompleteMap<$shape> = recipe_from_image(filepath).execute();
            let end = Instant::now();
            let generation_time = (end - start).as_secs_f32();
            println!("generation time: {generation_time}");
            let _ = cmap.save("map.bin");
            // draw_old_style(&cmap);
            store
                .lock()
                .unwrap()
                .insert(world_name, CompleteMapEnum::$cmap_enum(cmap));
        }};
    }
    match shape {
        ShapeEnum::Cylinder => generate_map_with_shape!(Cylinder, Cylinder, config),
        ShapeEnum::Globe => generate_map_with_shape!(Globe, Globe, config),
        ShapeEnum::Flat => generate_map_with_shape!(Flat, Flat, config),
    }
}

#[get("/draw", format = "json", data = "<input>")]
fn draw(input: Json<RequestData<ViewConfiguration>>, store: &State<MapStore>) -> Json<Message> {
    let req_params = input.into_inner();
    let view_config: ViewConfiguration = req_params.params;
    let key: String = req_params.world_name;

    let locked_store = store.lock().unwrap();
    let Some(cmap_enum): Option<&CompleteMapEnum> = locked_store.get(&key) else {
        return Json(Message {
            message: format!("no map found"),
        });
    };
    match cmap_enum {
        CompleteMapEnum::Globe(cmap) => draw_with_config(cmap, &view_config),
        CompleteMapEnum::Cylinder(cmap) => draw_with_config(cmap, &view_config),
        CompleteMapEnum::Flat(cmap) => draw_with_config(cmap, &view_config),
    }
    Json(Message {
        message: "Ok".to_string(),
    })
}

#[get("/get_image", format = "json", data = "<input>")]
fn get_image(
    input: Json<RequestData<ViewConfiguration>>,
    store: &State<MapStore>,
) -> Option<(ContentType, Vec<u8>)> {
    let input_inner = input.into_inner();

    let locked_store = store.lock().unwrap();
    let Some(cmap_enum): Option<&CompleteMapEnum> = locked_store.get(&input_inner.world_name)
    else {
        return None;
    };

    let img = match cmap_enum {
        CompleteMapEnum::Globe(cmap) => img_from_config(cmap, &input_inner.params),
        CompleteMapEnum::Cylinder(cmap) => img_from_config(cmap, &input_inner.params),
        CompleteMapEnum::Flat(cmap) => img_from_config(cmap, &input_inner.params),
    };

    let mut buffer = Vec::new();
    if image::codecs::png::PngEncoder::new(&mut buffer)
        .encode(&img, img.width(), img.height(), image::ColorType::Rgba8)
        .is_ok()
    {
        Some((ContentType::PNG, buffer))
    } else {
        None
    }
}

#[derive(Deserialize)]
struct SaveInput {
    world_name: String,
    path: String,
}

#[post("/save", format = "json", data = "<input>")]
fn save(input: Json<SaveInput>, store: &State<MapStore>) -> Json<Message> {
    let input_inner: SaveInput = input.into_inner();

    let locked_store = store.lock().unwrap();
    let Some(cmap_enum): Option<&CompleteMapEnum> = locked_store.get(&input_inner.world_name)
    else {
        return Json(Message {
            message: format!("no map found"),
        });
    };
    let filepath = input_inner.path.as_str();
    let result = match cmap_enum {
        CompleteMapEnum::Globe(cmap) => cmap.save(filepath),
        CompleteMapEnum::Cylinder(cmap) => cmap.save(filepath),
        CompleteMapEnum::Flat(cmap) => cmap.save(filepath),
    };
    match result {
        Ok(()) => Json(Message {
            message: "Ok".to_string(),
        }),
        _ => Json(Message {
            message: "Error: Failed to save map".to_string(),
        }),
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GenerationRequest {
    world_name: String,
    world_config: Configuration,
}

#[post("/generate", format = "json", data = "<input>")]
fn generate(input: Json<GenerationRequest>, store: &State<MapStore>) -> Json<Message> {
    let input_inner: GenerationRequest = input.into_inner();
    generate_map(&input_inner, store);
    Json(Message {
        message: format!("Generated map {}", input_inner.world_name),
    })
}

#[derive(Serialize, Deserialize)]
struct LoadConfig {
    world_name: String,
    file: String,
    shape: ShapeEnum,
}

#[post("/generate_from_image", format = "json", data = "<input>")]
fn generate_from_image(input: Json<LoadConfig>, store: &State<MapStore>) -> Json<Message> {
    let input_inner: LoadConfig = input.into_inner();
    generate_map_from_image(
        input_inner.world_name,
        input_inner.file,
        &input_inner.shape,
        store,
    );
    Json(Message {
        message: format!("Loaded height map!"),
    })
}

#[post("/load", format = "json", data = "<input>")]
fn load_map(input: Json<LoadConfig>, store: &State<MapStore>) -> Json<Message> {
    let input_inner: LoadConfig = input.into_inner();
    let filename = input_inner.file.as_str();
    let cmap = load::<Globe>(filename);
    store
        .lock()
        .unwrap()
        .insert(input_inner.world_name, CompleteMapEnum::Globe(cmap));
    Json(Message {
        message: format!("Loaded {}!", filename),
    })
}

#[post("/add_noise", format = "json", data = "<input>")]
fn add_noise(input: Json<BasicRequestParams>, store: &State<MapStore>) -> Json<Message> {
    let key = input.into_inner().world_name;
    apply_operation!(
        HeightNoise::new(rand::random(), 30.0, 70.0),
        key.clone(),
        store
    );
    apply_operation!(
        HeightNoise::new(rand::random(), 60.0, 70.0),
        key.clone(),
        store
    );
    apply_operation!(
        HeightNoise::new(rand::random(), 100.0, 40.0),
        key.clone(),
        store
    );
    apply_operation!(
        HeightNoise::new(rand::random(), 200.0, 20.0),
        key.clone(),
        store
    );
    apply_operation!(DefineCoastline {}, key, store);
    Json(Message {
        message: "Successfully added noise".to_string(),
    })
}

#[post("/smooth", format = "json", data = "<input>")]
fn smooth(input: Json<BasicRequestParams>, store: &State<MapStore>) -> Json<Message> {
    let key = input.into_inner().world_name;
    apply_operation!(Smooth::new(), key, store);
    Json(Message {
        message: "Successfully applied smooth".to_string(),
    })
}

#[post("/erosion", format = "json", data = "<input>")]
fn erosion(input: Json<BasicRequestParams>, store: &State<MapStore>) -> Json<Message> {
    let key = input.into_inner().world_name;
    apply_operation!(HydraulicErosion::new(4), key.clone(), store);
    apply_operation!(DefineCoastline {}, key, store);
    Json(Message {
        message: "Successfully added erosion".to_string(),
    })
}

#[post("/resize", format = "json", data = "<input>")]
fn resize(input: Json<RequestData<Resize>>, store: &State<MapStore>) -> Json<Message> {
    let inner_input = input.into_inner();
    let key = inner_input.world_name;
    apply_operation!(inner_input.params, key.clone(), store);
    apply_operation!(DefineCoastline {}, key, store);
    Json(Message {
        message: "Successfully resized".to_string(),
    })
}

#[post("/translation_noise", format = "json", data = "<input>")]
fn translation_noise(input: Json<BasicRequestParams>, store: &State<MapStore>) -> Json<Message> {
    let key = input.into_inner().world_name;
    apply_operation!(TranslationNoise::new(rand::random()), key, store);
    Json(Message {
        message: "Successfully added erosion".to_string(),
    })
}

#[post("/calculate_climate", format = "json", data = "<input>")]
fn post_calculate_climate(
    input: Json<RequestData<ClimateConfiguration>>,
    store: &State<MapStore>,
) -> Json<Message> {
    let inner_input = input.into_inner();
    let key = inner_input.world_name;
    let climate_config = inner_input.params;
    let precipitation_percentiles = vec![
        (15.0, 0),
        (18.0, 30),
        (22.0, 40),
        (30.0, 50),
        (55.0, 70),
        (88.0, 150),
        (100.0, 250),
    ];
    let operation = CalculateClimate::new(
        &precipitation_percentiles,
        climate_config.equator_temperature,
        climate_config.pole_temperature,
        climate_config.humidity
    );
    apply_operation!(operation, key, store);
    Json(Message {
        message: "Successfully added erosion".to_string(),
    })
}

#[post("/adjust_water_percentage", format = "json", data = "<input>")]
fn adjust_water_percentage(
    input: Json<RequestData<WaterLevel>>,
    store: &State<MapStore>,
) -> Json<Message> {
    let input_inner = input.into_inner();
    let key = input_inner.world_name;
    apply_operation!(input_inner.params, key.clone(), store);
    apply_operation!(DefineCoastline {}, key, store);
    Json(Message {
        message: "Successfully added erosion".to_string(),
    })
}

#[derive(Serialize)]
struct Dimensions {
    width: usize,
    height: usize,
}

#[derive(Deserialize)]
struct BasicRequestParams {
    world_name: String,
}

#[get("/get_size", format = "json", data = "<input>")]
fn get_size(input: Json<BasicRequestParams>, store: &State<MapStore>) -> Json<Dimensions> {
    let key = input.into_inner().world_name;
    let locked_store = store.lock().unwrap();
    let Some(cmap_enum) = &locked_store.get(&key) else {
        return Json(Dimensions {
            width: 0,
            height: 0,
        });
    };
    match cmap_enum {
        CompleteMapEnum::Globe(cmap) => Json(Dimensions {
            width: 2 * cmap.height.values.len(),
            height: cmap.height.values.len(),
        }),
        CompleteMapEnum::Cylinder(cmap) => Json(Dimensions {
            width: cmap.height.values[0].len(),
            height: cmap.height.values.len(),
        }),
        CompleteMapEnum::Flat(cmap) => Json(Dimensions {
            width: cmap.height.values[0].len(),
            height: cmap.height.values.len(),
        }),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(HashMap::<String, CompleteMapEnum>::new()))
        .mount(
            "/",
            routes![
                save,
                generate,
                draw,
                get_image,
                load_map,
                generate_from_image,
                add_noise,
                smooth,
                erosion,
                translation_noise,
                adjust_water_percentage,
                resize,
                post_calculate_climate,
                get_size
            ],
        )
}
