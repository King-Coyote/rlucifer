use std::ops;

#[derive(Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn normalise(&self) -> Self {
        let inv_len = 1.0/self.length();
        Vector3::new(
            self.x * inv_len,
            self.y * inv_len,
            self.z * inv_len,
        )
    }

    pub fn cross(&self, rhs: &Vector3) -> Self {
        Vector3::new(
            self.y*rhs.z - self.z*rhs.y, 
            self.z*rhs.x - self.x*rhs.z, 
            self.x*rhs.y - self.y*rhs.x
        )
    }

    pub fn dot(&self, rhs: &Vector3) -> f32 {
            self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn form_ons(&self) -> (Vector3, Vector3, Vector3) {
        let v: Vector3;
        if self.x.abs() > self.y.abs() {
            let inv_len = 1.0 / (self.x * self.x + self.z * self.z).sqrt();
            v = Vector3::new(self.z * -1.0 * inv_len, 0.0, self.x * inv_len);
        } else {
            let inv_len = 1.0 / (self.y * self.y + self.z * self.z).sqrt();
            v = Vector3::new(0.0, self.z * inv_len, self.y * -1.0 * inv_len); // project to x=0 and get normalised vector
        }
        let w = self.cross(&v);
        return (self.clone(), v, w);
    }

    pub fn distance_to(&self, other: &Vector3) -> f32 {
        (other - self).length()
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    }
}

impl<'a, 'b> ops::Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &'b Vector3) -> Vector3 {
        Vector3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl<'a, 'b> ops::Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &'b Vector3) -> Vector3 {
        Vector3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl ops::Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Vector3 {
        let inv_div = 1.0/rhs;
        Vector3::new(
            self.x * inv_div,
            self.y * inv_div,
            self.z * inv_div,
        )
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}

impl ops::Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}