use std::ops;

#[derive(Default, Clone,)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel {r: r, g: g, b: b}
    }

    pub fn new_default() -> Self {
        Pixel {r: 255, g: 0, b: 255}
    }

    pub fn new_black() -> Self {
        Pixel {r: 0, g: 0, b: 0}
    }
}

// OPERATORS

fn clamp(val: u8, min: u8, max: u8) -> u8 {
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
            clamp(self.r + rhs.r, 0, 255), 
            clamp(self.b + rhs.b, 0, 255), 
            clamp(self.g + rhs.g, 0, 255)
        )
    }
}

impl ops::Sub<Pixel> for Pixel {
    type Output = Pixel;

    fn sub(self, rhs: Pixel) -> Pixel {
        Pixel::new(
            clamp(self.r - rhs.r, 0, 255), 
            clamp(self.b - rhs.b, 0, 255), 
            clamp(self.g - rhs.g, 0, 255)
        )
    }
}

impl ops::Mul<u8> for Pixel {
    type Output = Pixel;

    fn mul(self, rhs: u8) -> Pixel {
        Pixel::new(
            self.r * rhs, 
            self.g * rhs, 
            self.b * rhs
        )
    }
}