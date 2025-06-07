use std::ops::{Add, Sub, Mul, Div};



#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2 { x: $x as f32, y: $y as f32 }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        vec2!(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        vec2!(self.x - other.x, self.y - other.y)
    }
}

// component multiplication
impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        vec2!(self.x * other.x, self.y * other.y)
    }
}

// scalar multiplication
impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        vec2!(self.x * scalar, self.y * scalar)
    }
}

// scalar division
impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        vec2!(self.x / scalar, self.y / scalar)
    }
}

// component division
impl Div for Vec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        vec2!(self.x / other.x, self.y / other.y)
    }
}

pub use crate::vec2;
pub use crate::vec3;




#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        vec3!(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        vec3!(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        vec3!(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        vec3!(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        vec3!(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}
impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        vec3!(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
impl Vec3 {
    pub fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 {
            return Vec3 { x: self.x / len, y: self.y / len, z: self.z / len };
        }
        self
    }
}

// pub fn vec2(x: f32, y: f32) -> Vec2 {
//     Vec2{ x, y }
// }


