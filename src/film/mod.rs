use crate::common::*;
use crate::Tile;

#[derive(Debug, Default, Clone)]
pub struct Film {
    pub image: Vec<Spectrum>,
    pub height: i32,
    pub width: i32,
    pub fov: fp,
    pub distance_to_film: fp,
    pub aspect_ratio: fp,
}

impl Film {
    pub fn new_film(&mut self, width: i32, height: i32, fovdegrees: fp) {
        self.distance_to_film = 1.0;
        self.aspect_ratio = fp::from(width) / fp::from(height);
        self.fov = fovdegrees * std::f64::consts::PI / 180.0;

        self.width = width;
        self.height = height;
        self.image
            .resize((width * height) as usize, Vector3::new(0.0, 0.3, 0.7));
    }

    pub fn write_tile(&mut self, tile: Tile) {
        let starting_index = tile.start_index as usize;
        let num_pixels_to_write = tile.num_pixels;
        self.image[starting_index..(starting_index + num_pixels_to_write)]
            .clone_from_slice(&tile.pixels[0..num_pixels_to_write]);
    }
}
