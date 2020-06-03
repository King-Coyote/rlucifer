use crate::basic_types::{Vector3, };

pub struct Hit {
    t: f32, // distance
    point: Vector3,
    normal: Vector3,
}

impl Hit {
    pub fn new(t: f32, point: Vector3, normal: Vector3) -> Self {
        Hit {
            t: t,
            point: point,
            normal: normal
        }
    }
}

// float          t; // length of ray ie distance away
// Vec             hitPoint;
// Vec             hitNormal;
// const RenderObject*   hitObject;