use crate::objects::*;
use crate::basic_types::*;
use std::f32;

pub struct Scene {
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn Renderable + Send + Sync>>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: vec![],
            objects: vec![]
        }
    }

    pub fn get_closest_intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut min_distance = f32::INFINITY;
        let mut closest_hit: Option<Hit> = None;
        for o in self.objects.iter() {
            if let Some(intersect) = o.get_intersection(&ray) {
                if intersect.t < min_distance {
                    min_distance = intersect.t;
                    closest_hit = Some(intersect);
                }
            }
        }
    
        closest_hit
    }

    pub fn get_lights(&self) -> &[Light] {
        &self.lights
    }
}