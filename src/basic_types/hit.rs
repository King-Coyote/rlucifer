use crate::basic_types::{Vector3, Ray, };
use crate::objects::{Material};

pub struct Hit {
    pub t: f32, // distance
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl Hit {
    pub fn new(t: f32, point: Vector3, normal: Vector3, material: Material) -> Self {
        Hit {
            t: t,
            point: point,
            normal: normal,
            material: material
        }
    }

}

// float          t; // length of ray ie distance away
// Vec             hitPoint;
// Vec             hitNormal;
// const RenderObject*   hitObject;