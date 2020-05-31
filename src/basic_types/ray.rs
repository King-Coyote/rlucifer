use crate::basic_types::Vector3;

pub struct Ray {
    origin: Vector3,
    dir: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, dir: Vector3) -> Self {
        Ray {
            origin: origin,
            dir: dir
        }
    }
}