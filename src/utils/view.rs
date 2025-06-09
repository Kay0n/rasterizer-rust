use minifb::{Key, Window, WindowOptions};
use crate::{RenderTarget};


pub struct View {
    pub window: Window,
    width: u32,
    height: u32
}



impl View {

    pub fn new(width: u32, height: u32) -> View {
        let mut window = Window::new(
            "Rust Rasterizer",
            width as usize,
            height as usize,
            WindowOptions::default(),
        ).expect("Failed to create window");
        window.set_target_fps(500);

        return View {
            window,
            width,
            height
        }
    }


    pub fn draw(&mut self, render_target: &RenderTarget){
        self.window.update_with_buffer(
            &render_target.color_buffer, 
            self.width as usize, 
            self.height as usize
        ).unwrap();
    }


    pub fn is_active(&self) -> bool {
        return self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}


