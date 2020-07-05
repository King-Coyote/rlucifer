use crate::basic_types::{Vector3, Ray, Hit,};
use crate::objects::{Renderable, Material};
use crate::util::float_zero;
use std::sync::Mutex;

pub struct Plane {
    id: String,
    pos: Vector3,
    d: f32,
    normal: Vector3,
    material: Material,
}

impl Plane {
    pub fn new(id: String, pos: Vector3, normal: Vector3, material: Material) -> Self {
        Plane {
            id: id,
            pos: pos.clone(),
            d: pos.dot(&normal),
            normal: normal,
            material: material
        }
    }
}

impl Renderable for Plane {
    
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_normal_at(&self, point: Vector3) -> Vector3 {
        self.normal.clone()
    }

    fn get_intersection(&self, ray: &Ray) -> Option<Hit> {
        let nr_o = self.normal.dot(&ray.origin); // dot product of plane normal and ray origin
        let nr_d = self.normal.dot(&ray.dir);
    
        if float_zero(nr_d) {
            None // this means tha t the ray is parallel to the normal, i.e. no intersections and a division by zero
        } else {
            let t = (self.d - nr_o) / nr_d;
            if t > 1e-6 { // correct for self intersection
                Some(Hit::new(t, &ray.origin + &(&ray.dir * t), self.normal.clone(), self.material.clone()))
            } else {
                None
            }
        }
    }

}