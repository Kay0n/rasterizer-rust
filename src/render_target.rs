pub struct RenderTarget {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u32>,
}

impl RenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer: Vec<u32> = vec![0; (width * height) as usize]; 
        Self { width, height, buffer }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        let index = (y * self.width + x) as usize;
        self.buffer[index] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height {
            panic!(
                "get_pixel out of bounds: ({}, {}) for dimensions ({}, {})",
                x, y, self.width, self.height
            );
        }
        self.buffer[(y * self.width + x) as usize]
    }
    pub fn fill(&mut self, color: u32) {
        self.buffer.fill(color);
    }
}
