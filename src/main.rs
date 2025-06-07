#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

mod render_target;
mod math;
mod model;

use std::io::{Result};
use std::time::{Instant, Duration};
use minifb::{Key, Window, WindowOptions};

use render_target::RenderTarget;
use math::math::*;
use math::rng::*;
use math::vectors::*;
use model::*;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;
const TARGET_FPS: u64 = 60;
const FOV: f32 = 70.0;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / TARGET_FPS); // ~16 ms
      
const NEAR_CLIP_PLANE: f32 = -0.001; // z axis

    


fn main() -> Result<()> {

    let mut cube = get_model("./models/Tori.obj");
        cube.transform.position.z = -20.0;
        // cube.transform.pitch = 42.0;

    let mut render_target: RenderTarget = RenderTarget::new(WIDTH, HEIGHT);
    let mut window = init_window(WIDTH, HEIGHT);

    let mut last_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let frame_start = Instant::now();
        let now = Instant::now();
        let delta_time = (now - last_time).as_secs_f32();
        last_time = now;
        
        update(&mut cube, delta_time);

        render(&mut render_target, &cube, FOV);

        window
            .update_with_buffer(&render_target.buffer, WIDTH as usize, HEIGHT as usize)
            .unwrap();

        let elapsed = frame_start.elapsed();
        println!("Frame time: {} ms", elapsed.as_millis());

        // if elapsed < FRAME_DURATION {
        //     std::thread::sleep(FRAME_DURATION - elapsed);
        // }
    }
    return Ok(());


}


fn update(model: &mut Model, delta_time: f32){
    // 10 deg per sec
    model.transform.yaw += 10.0 * delta_time;
    model.transform.pitch += 10.0 * delta_time;
    // model.transform.position.x += 1.0 * delta_time;
}


fn render(render_target: &mut RenderTarget, model: &Model, fov: f32){

    // clear bg
    render_target.fill(0);

    for i in (0..model.points.len()).step_by(3) {
        let a = vertex_to_screen(model.points[i], &model.transform, WIDTH, HEIGHT, fov);
        let b = vertex_to_screen(model.points[i + 1], &model.transform, WIDTH, HEIGHT, fov);
        let c = vertex_to_screen(model.points[i + 2], &model.transform, WIDTH, HEIGHT, fov);

        let d = vec2!(a.x, a.y);
        let e = vec2!(b.x, b.y);
        let f = vec2!(c.x, c.y);

        draw_triangle(render_target, d, e,f, model.colors[i / 3]);
    }

}



fn vertex_to_screen(vertex: Vec3, transform: &Transform, screen_width: u32, screen_height: u32, fov: f32) -> Vec3 {

    let vertex_world = transform.to_world_point(vertex);
    // println!("{}", vertex_world.z);
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



// fn argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
//     ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
// }



struct Triangle {
    a: Vec2,
    b: Vec2,
    c: Vec2,
    color: u32,
    velocity: Vec2
}



fn draw_triangle(fb: &mut RenderTarget, p1: Vec2, p2: Vec2, p3: Vec2, color: u32) {
      
    // calculate bounding box
    let min_x = p1.x.min(p2.x).min(p3.x).floor().max(0.0) as u32;
    let mut max_x = p1.x.max(p2.x).max(p3.x).ceil() as i32;
    let min_y = p1.y.min(p2.y).min(p3.y).floor().max(0.0) as u32;
    let mut max_y = p1.y.max(p2.y).max(p3.y).ceil() as i32;
    
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
    let dy0 = p3.y - p2.y; 
    let dx0 = p2.x - p3.x; 
    let dy1 = p1.y - p3.y;
    let dx1 = p3.x - p1.x;
    let dy2 = p2.y - p1.y;
    let dx2 = p1.x - p2.x;
    
    // start at top left of bounding box
    let p_start = vec2!(min_x as f32 + 0.5, min_y as f32 + 0.5);
    
    // backface cull; area negative or zero
    let area = edge(p1, p2, p3);
    if area <= 0.0 { 
        return;
    }
    
    // initial barycentric weights
    let mut w0_row_start = edge(p2, p3, p_start);
    let mut w1_row_start = edge(p3, p1, p_start);
    let mut w2_row_start = edge(p1, p2, p_start);



    
    // raster loop
    for y in min_y..=max_y {
        
        let mut w0 = w0_row_start;
        let mut w1 = w1_row_start;
        let mut w2 = w2_row_start;
    

        for x in min_x..=max_x {

            // check if pixel inside triangle
            // assumes CCW winding
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                fb.set_pixel(x, y, color);
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
fn edge(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}



fn init_window(width: u32, height: u32) -> Window {
    let mut window = Window::new(
        "Rust Rasterizer",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.set_target_fps(500);
    return window;
}



