use crate::{Vec2, Vec3}; 
use crate::Transform;

pub struct Texture {
    pub data: Vec<u32>, 
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(data: Vec<u32>, width: u32, height: u32) -> Self {
        assert_eq!(data.len(), (width * height) as usize, "Texture data length does not match dimensions.");
        Texture { data, width, height }
    }


    pub fn sample(&self, u: f32, v: f32) -> u32 {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let x = (u * (self.width - 1) as f32).round() as u32;
        let y = (v * (self.height - 1) as f32).round() as u32;

        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        let index = (y * self.width + x) as usize;
        self.data[index]
    }
}