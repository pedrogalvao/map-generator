pub trait Projection {
    fn new() -> Self;

    fn img_to_map_coords(
        &self,
        x: u32,
        y: u32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[f32; 2]>;

    fn map_to_img_coords(
        &self,
        latitude: f32,
        longitude: f32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> Option<[u32; 2]>;

    fn get_latitude(&self, img_x: u32, img_y: u32, img_width: u32, img_height: u32) -> f32;

    fn get_img_y(
        &self,
        latitude: f32,
        longitude: f32,
        img_width: u32,
        img_height: u32,
        center_longitude: f32,
    ) -> u32;
}
