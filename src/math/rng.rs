use crate::math::vectors::*;
pub struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        // Linear Congruential Generator constants
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }

    pub fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        min + (self.next_u32() % (max - min + 1))
    }

    pub fn random_argb(&mut self) -> u32 {
        let a = self.rand_range(0, 255) << 24;
        let r = self.rand_range(0, 255) << 16;
        let g = self.rand_range(0, 255) << 8;
        let b = self.rand_range(0, 255);
        a | r | g | b
    }

    pub fn rand_f32_range(&mut self, min: f32, max: f32) -> f32 {
        let unit = self.next_u32() as f32 / u32::MAX as f32; // [0.0, 1.0]
        min + unit * (max - min)
    }

    pub fn random_vec2(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Vec2 {
        vec2!(
            self.rand_f32_range(x_min, x_max),
            self.rand_f32_range(y_min, y_max)
        )
    }
}



