use crate::basic_types::{Vector3, Ray, Hit,};
use crate::objects::Renderable;
use crate::util::float_zero;

pub struct Plane {
    id: usize,
    d: f32,
    normal: Vector3,
}

impl Renderable for Plane {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_normal_at(&self, point: Vector3) -> Vector3 {
        self.normal.clone()
    }

    fn get_intersection(&self, ray: &Ray) -> Option<Hit> {
        let nRo = self.normal.dot(&ray.origin); // dot product of plane normal and ray origin
        let nRd = self.normal.dot(&ray.dir);
    
        if float_zero(nRd) {
            None // this means tha t the ray is parallel to the normal, i.e. no intersections and a division by zero
        } else {
            let t = (self.d - nRo) / nRd;
            if t > 1e-6 { // correct for self intersection
                Some(Hit::new(t, (&ray.origin + &ray.dir) * t, self.normal.clone()))
            } else {
                None
            }
        }
    }

}