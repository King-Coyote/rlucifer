use crate::basic_types::Vector3;

#[derive(Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, dir: Vector3) -> Self {
        Ray {
            origin: origin,
            dir: dir.normalise()
        }
    }

    pub fn new_empty() -> Self {
        Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            dir: Vector3::new(0.0, 0.0, 0.0)
        }
    }
}