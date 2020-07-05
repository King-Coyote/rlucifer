use crate::basic_types::{Pixel, Vector3};

pub struct Light {
    pub position: Vector3,
    pub intensity: f32,
    pub color: Pixel,
}

impl Light {
    pub fn new(position: Vector3, intensity: f32, color: Pixel) -> Self {
        Light {
            position: position,
            intensity: intensity,
            color: color
        }
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }
}