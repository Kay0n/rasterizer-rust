#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::io::{Result};
use minifb::Scale;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

use config::*;
use graphics::render_target::*;
use graphics::renderer::*;
use graphics::camera::*;
use types::model::*;
use types::transform::*;
use types::vec2::*;
use types::vec3::*;
use utils::file_parser::*;
use utils::random::*;
use utils::view::*;

use crate::graphics::renderer;
use crate::graphics::scene::Scene;

mod graphics;
mod types;
mod utils;
mod config;







fn main() -> Result<()> {


    let mut cube = parse_obj("./models/Tori.obj");
    
    
    let mut scene = Scene::new();
    let renderer = Renderer::new();

    let mut render_target: RenderTarget = RenderTarget::new(WIDTH, HEIGHT);
    let mut view = View::new(WIDTH, HEIGHT);

    let mut last_time = Instant::now();


    cube.transform.position.z = -20.0;
    scene.load_model(cube);



    while view.is_active() {

        let frame_start = Instant::now();
        let delta_time = (frame_start - last_time).as_secs_f32();
        last_time = frame_start;
        
        scene.update(delta_time);

        renderer.render(&mut render_target, &scene, FOV);

        view.draw(&render_target);

        let elapsed = frame_start.elapsed();
        println!("Frame time: {} ms", elapsed.as_millis());

        // if elapsed < FRAME_DURATION {
        //     std::thread::sleep(FRAME_DURATION - elapsed);
        // }
    }
    return Ok(());


}












// fn argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
//     ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
// }








