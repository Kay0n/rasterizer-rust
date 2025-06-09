use std::sync::Arc;
use std::num::NonZeroU32;
use crate::vec2;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, MouseButton, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::KeyCode,
    window::{CursorGrabMode, Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;
use softbuffer::{Context, Surface};
use crate::{graphics::render_target::RenderTarget, types::vec2::Vec2};





pub struct View {
    pub input: WinitInputHelper,
    pub window: Arc<Window>,
    pub surface: Surface<Arc<Window>, Arc<Window>>,
    pub is_focused: bool,
    pub width: u32,
    pub height: u32,
    event_loop: Option<EventLoop<()>>, 
}



impl View {
    pub fn new(width: u32, height: u32) -> Self {
        let input = WinitInputHelper::new();
        let event_loop = EventLoop::new().unwrap();

        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Softbuffer View")
                .with_inner_size(LogicalSize::new(width, height))
                .with_resizable(true)
                .build(&event_loop)
                .unwrap(),
        );

        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        Self {
            input,
            window,
            surface,
            width,
            height,
            event_loop: Some(event_loop),
            is_focused: false,
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        return (size.width, size.height)
    }

    pub fn key_held(&self, key: KeyCode) -> bool{
        return self.input.key_held(key);
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool{
        return self.input.mouse_pressed(button);
    }

    pub fn mouse_delta(&self) -> Vec2 {
        let (x, y) = self.input.mouse_diff();
        return vec2!(x, y);
    }


    pub fn set_mouse_grab(&mut self, grab: bool) {
        let mode = if grab {
            self.window.set_cursor_visible(false);
            self.is_focused = true;
            CursorGrabMode::Confined
            
        } else {
            self.window.set_cursor_visible(true);
            self.is_focused = false;
            CursorGrabMode::None
        };
    
        let _ = self.window.set_cursor_grab(mode);
    }

    pub fn run<F>(mut self, mut frame_fn: F)
    where
        F: 'static + FnMut(&mut Self),
    {
        let event_loop = self.event_loop.take().unwrap(); 
        let window = self.window.clone();

        event_loop
            .run(move |event, elwt| {
                if self.input.update(&event) {
                    // if self.input.key_released(KeyCode::Escape)
                    //     || self.input.close_requested()
                    //     || self.input.destroyed()
                    // {
                    //     elwt.exit();
                    //     return;
                    // }

                    self.handle_resize();

                    frame_fn(&mut self);

                    self.window.request_redraw();
                }

                if let Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } = &event
                {
                    window.request_redraw();
                }
            })
            .unwrap();
    }

    fn handle_resize(&mut self) {
        let size = self.window.inner_size();
        self.width = size.width;
        self.height = size.height;

        if let (Some(w), Some(h)) = (NonZeroU32::new(self.width), NonZeroU32::new(self.height)) {
            self.surface.resize(w, h).unwrap();
        }
    }

    pub fn draw(&mut self, target: &RenderTarget){
        let (win_width, win_height) = self.get_size();

        if win_width == 0 || win_height == 0 {
            return;
        }

        let mut window_buffer = self.surface.buffer_mut().unwrap();
        View::scale_buffer(
            &mut window_buffer,
            win_width,
            win_height,
            &target.color_buffer,
            target.width, 
            target.height,
        );

        window_buffer.present().unwrap();
    }

    pub fn scale_buffer(
        dst_buf: &mut [u32],
        dst_width: u32,
        dst_height: u32,
        src_buf: &[u32],
        src_width: u32,
        src_height: u32,
    ) {
        if src_buf.is_empty() || dst_buf.is_empty() {
            return;
        }
    
        if dst_width == src_width && dst_height == src_height {
            let len = src_buf.len().min(dst_buf.len());
            dst_buf[..len].copy_from_slice(&src_buf[..len]);
            return;
        }
    
        let x_scale_factor: u64 = (src_width as u64 * 65536) / dst_width as u64;
        let y_scale_factor: u64 = (src_height as u64 * 65536) / dst_height as u64;
    
        for dst_y in 0..dst_height {
            let src_y = ((dst_y as u64 * y_scale_factor) / 65536) as u32;
    
            if src_y >= src_height {
                continue;
            }
    
            let src_row_start_index = (src_y * src_width) as usize;
            let dst_row_start_index = (dst_y * dst_width) as usize;
    
            for dst_x in 0..dst_width {
                let src_x = ((dst_x as u64 * x_scale_factor) / 65536) as u32;
    
                let src_index = src_row_start_index + src_x as usize;
                let dst_index = dst_row_start_index + dst_x as usize;  

                dst_buf[dst_index] = src_buf[src_index];
            }
        }
    }
}



