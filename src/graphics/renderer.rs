
use crate::{RenderTarget, Scene, Model, Transform, Vec3, vec3, vec2, Vec2};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Renderer {
        return Renderer;
    }

    pub fn render(&self, render_target: &mut RenderTarget, scene: &Scene, fov: f32){

        // clear buffers
        render_target.fill(0);
        render_target.depth_buffer.iter_mut().for_each(|d| *d = f32::NEG_INFINITY);
        for model in &scene.models {
            for i in (0..model.points.len()).step_by(3) {
                let a = self.vertex_to_screen(model.points[i], &model.transform, render_target.width, render_target.height, fov);
                let b = self.vertex_to_screen(model.points[i + 1], &model.transform, render_target.width, render_target.height, fov);
                let c = self.vertex_to_screen(model.points[i + 2], &model.transform, render_target.width, render_target.height, fov);
        
                self.draw_triangle(render_target, a, b, c, model.colors[i / 3]);
            }
        }

    
    }


    fn vertex_to_screen(&self, vertex: Vec3, transform: &Transform, screen_width: u32, screen_height: u32, fov: f32) -> Vec3 {

        let vertex_world = transform.to_world_point(vertex);
        if vertex_world.z > -0.0001 {
            return vec3!(-1.0, -1.0, -1.0); // point outside screen
        }
    
        let world_height = (fov.to_radians() / 2.0).tan() * 2.0;
        let pixels_per_world_unit: f32 = (screen_height as f32 / world_height) / vertex_world.z;
    
        let screen_center = vec2!(screen_width as f32 / 2.0, screen_height as f32 / 2.0);
        let pixel_offset = vec2!(vertex_world.x * pixels_per_world_unit, -vertex_world.y * pixels_per_world_unit);
        let vertex_screen = screen_center + pixel_offset;
        return vec3!(vertex_screen.x, vertex_screen.y, vertex_world.z)
    }


    fn draw_triangle(&self, fb: &mut RenderTarget, p1: Vec3, p2: Vec3, p3: Vec3, color: u32) {

        let v1_2d = vec2!(p1.x, p1.y);
        let v2_2d = vec2!(p2.x, p2.y);
        let v3_2d = vec2!(p3.x, p3.y);
          
        // calculate bounding box
        let min_x = v1_2d.x.min(v2_2d.x).min(v3_2d.x).floor().max(0.0) as u32;
        let mut max_x = v1_2d.x.max(v2_2d.x).max(v3_2d.x).ceil() as i32;
        let min_y = v1_2d.y.min(v2_2d.y).min(v3_2d.y).floor().max(0.0) as u32;
        let mut max_y = v1_2d.y.max(v2_2d.y).max(v3_2d.y).ceil() as i32;
        
        // clamp max values to screen dimensions
        max_x = max_x.min(fb.width as i32 - 1);
        max_y = max_y.min(fb.height as i32 - 1);
        
        // if bounding box invalid / entirely off screen
        if max_x < min_x as i32 || max_y < min_y as i32 {
            return;
        }
        
        let max_x = max_x as u32;
        let max_y = max_y as u32;
        
        // optimize for barycentric coordinate calculation
        let dy0 = v3_2d.y - v2_2d.y; 
        let dx0 = v2_2d.x - v3_2d.x; 
        let dy1 = v1_2d.y - v3_2d.y;
        let dx1 = v3_2d.x - v1_2d.x;
        let dy2 = v2_2d.y - v1_2d.y;
        let dx2 = v1_2d.x - v2_2d.x;
        
        // start at top left of bounding box
        let p_start = vec2!(min_x as f32 + 0.5, min_y as f32 + 0.5);
        
        // backface cull; area negative or zero
        let area = self.edge(v1_2d, v2_2d, v3_2d);
        if area <= 0.0 { 
            return;
        }
    
        let inv_z1 = 1.0 / p1.z;
        let inv_z2 = 1.0 / p2.z;
        let inv_z3 = 1.0 / p3.z;
        
        // initial barycentric weights
        let mut w0_row_start = self.edge(v2_2d, v3_2d, p_start);
        let mut w1_row_start = self.edge(v3_2d, v1_2d, p_start);
        let mut w2_row_start = self.edge(v1_2d, v2_2d, p_start);
        
        // raster loop
        for y in min_y..=max_y {
            
            let mut w0 = w0_row_start;
            let mut w1 = w1_row_start;
            let mut w2 = w2_row_start;
    
            for x in min_x..=max_x {
    
                let point_in_tri = w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0;
    
                if point_in_tri {
                    let point_index = (y * fb.width + x) as usize;
    
                    let inv_depth_interp = (w0 * inv_z1 + w1 * inv_z2 + w2 * inv_z3) / area;
                    
                    let depth = 1.0 / inv_depth_interp;
    
                    if depth > fb.depth_buffer[point_index] {
                        fb.depth_buffer[point_index] = depth;
                        fb.set_pixel(x, y, color);
                    }
                }
        
                // incrementally update for next pixel in x-direction
                w0 += dy0;
                w1 += dy1;
                w2 += dy2;
            }
        
            // incrementally update for start of next row in y-direction
            w0_row_start += dx0;
            w1_row_start += dx1;
            w2_row_start += dx2;
        }
    }
    

    // signed area func
    fn edge(&self, a: Vec2, b: Vec2, c: Vec2) -> f32 {
        (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
    }
}











