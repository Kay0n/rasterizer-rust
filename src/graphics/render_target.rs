
pub struct RenderTarget {
    pub width: u32,
    pub height: u32,
    pub size: usize,
    pub color_buffer: Vec<u32>,
    pub depth_buffer: Vec<f32>,
}

impl RenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let color_buffer: Vec<u32> = vec![0; size]; 
        let depth_puffer:Vec<f32> = vec![f32::NEG_INFINITY; size];

        Self { width, height, size, color_buffer, depth_buffer: depth_puffer }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        let index = (y * self.width + x) as usize;
        self.color_buffer[index] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height {
            panic!(
                "get_pixel out of bounds: ({}, {}) for dimensions ({}, {})",
                x, y, self.width, self.height
            );
        }
        self.color_buffer[(y * self.width + x) as usize]
    }
    pub fn fill(&mut self, color: u32) {
        self.color_buffer.fill(color);
    }
}
