use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}



#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2 { x: $x as f32, y: $y as f32 }
    };
}
pub use crate::vec2;



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



impl Vec2 {
    pub fn dot(self, other: Vec2) -> f32 {
        return self.x * other.x + self.y * other.y;
    }
    
}


