use crate::basic_types::Vector3;

pub struct Camera {
    pos: Vector3,
    dir: Vector3,
    width: i32,
    height: i32,
    up: Vector3,
    right: Vector3,
}

impl Camera {
    pub fn new(pos: Vector3, dir: Vector3, width: i32, height: i32) -> Self {
        let up = Vector3::new(0.0, 0.0, 1.0);
        Camera {
            pos: pos,
            dir: dir.clone(),
            width: width,
            height: height,
            up: up.clone(),
            right: dir.cross(&up).normalise(),
        }
    }

    pub fn get_position(&self) -> Vector3 {
        self.pos.clone()
    }

    // pixel coordinates to image plane transformation
    pub fn pixel_to_img(&self, x: usize, y: usize) -> Vector3 {
        let alpha = (x as f32 / self.width as f32) - 0.5; // alpha and beta are normalisation factors. Here I have assumed FOV is 45 degrees and that the screen varies from 0 to 1.
        let beta = (y as f32 / self.height as f32) - 0.5;
    
        let fovX = 3.141593 / 2.0;
        let fovY = ((self.height / self.width) as f32) * fovX;
    
        (&self.right * alpha*fovX) +
        (&self.up * beta*fovY) +
        (self.dir.normalise())
    }
}