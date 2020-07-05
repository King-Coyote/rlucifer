use std::ops;

#[derive(Default, Clone, Copy,)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Pixel {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Pixel {r: r, g: g, b: b}
    }

    pub fn new_default() -> Self {
        Pixel {r: 1.0, g: 0.0, b: 1.0}
    }

    pub fn new_black() -> Self {
        Pixel {r: 0.0, g: 0.0, b: 0.0}
    }
}

// OPERATORS

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    match val {
        c if c > max => max,
        c if c < min => min,
        c => c
    }
}

impl ops::Add<Pixel> for Pixel {
    type Output = Pixel;

    fn add (self, rhs: Pixel) -> Pixel {
        Pixel::new(
            clamp(self.r + rhs.r, 0.0, 1.0), 
            clamp(self.b + rhs.b, 0.0, 1.0), 
            clamp(self.g + rhs.g, 0.0, 1.0)
        )
    }
}

impl ops::Add<Pixel> for &Pixel {
    type Output = Pixel;

    fn add (self, rhs: Pixel) -> Pixel {
        Pixel::new(
            clamp(self.r + rhs.r, 0.0, 1.0), 
            clamp(self.b + rhs.b, 0.0, 1.0), 
            clamp(self.g + rhs.g, 0.0, 1.0)
        )
    }
}

impl ops::Sub<Pixel> for Pixel {
    type Output = Pixel;

    fn sub(self, rhs: Pixel) -> Pixel {
        Pixel::new(
            clamp(self.r - rhs.r, 0.0, 1.0), 
            clamp(self.b - rhs.b, 0.0, 1.0), 
            clamp(self.g - rhs.g, 0.0, 1.0)
        )
    }
}

impl ops::Mul<f32> for Pixel {
    type Output = Pixel;

    fn mul(self, rhs: f32) -> Pixel {
        Pixel::new(
            self.r * rhs, 
            self.g * rhs, 
            self.b * rhs
        )
    }
}

impl ops::Mul<Pixel> for Pixel {
    type Output = Pixel;

    fn mul(self, rhs: Pixel) -> Pixel {
        Pixel::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b
        )
    }
}

impl ops::Div<f32> for Pixel {
    type Output = Pixel;

    fn div(self, rhs: f32) -> Pixel {
        Pixel::new(
            self.r / rhs, 
            self.g / rhs, 
            self.b / rhs
        )
    }
}