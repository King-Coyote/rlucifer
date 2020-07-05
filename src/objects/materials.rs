use crate::basic_types::*;

#[derive(Clone,)]
pub struct Material {
    pub color: Pixel,
    pub finish: Finish,
}

#[derive(Clone, )]
pub enum Finish {
    Diffuse {
        roughness: f32,
        specularity: f32,
    },
    Metallic {
        roughness: f32,
        specularity: f32,
    },
    Transparent {
        specularity: f32,
        refractive_index: f32,
    },
    Translucent,
    Mirror,
}

impl Material {
    pub fn new(color: Pixel, finish: Finish) -> Self {
        Material {
            color: color,
            finish: finish
        }
    }
}