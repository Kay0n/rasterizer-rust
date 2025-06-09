#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![feature(portable_simd)]

use std::io::{Result};
use std::time::{Duration, Instant};

use config::*;
use graphics::render_target::*;
use graphics::renderer::*;
use graphics::camera::*;
use graphics::scene::*;
use types::model::*;
use types::transform::*;
use types::vec2::*;
use types::vec3::*;
use utils::file_parser::*;
use utils::random::*;
use utils::view::*;

mod graphics;
mod types;
mod utils;
mod config;







fn main() -> Result<()> {


    let mut cube = parse_obj("./models/Tori.obj");
    let mut monkey = parse_obj("./models/monkey.obj");
    
    
    let mut scene = Scene::new();
    let mut renderer = Renderer::new();

    let mut render_target: RenderTarget = RenderTarget::new(WIDTH, HEIGHT);
    let mut view = View::new(1920, 1080);

    let mut last_time = Instant::now();


    cube.transform.position.z = -20.0;
    monkey.transform.position.z = -5.0;
    scene.load_model(cube);
    scene.load_model(monkey);
    scene.camera.fov = 70.0;



    view.run(move |view| {

        let frame_start = Instant::now();
        let delta_time = (frame_start - last_time).as_secs_f32();
        last_time = frame_start;
            
        scene.update(delta_time, view);

        renderer.render(&mut render_target, &scene);
        
        view.draw(&render_target);

        let elapsed = frame_start.elapsed();
        println!("Frame time: {} ms", elapsed.as_millis());

    });


    return Ok(());


}








