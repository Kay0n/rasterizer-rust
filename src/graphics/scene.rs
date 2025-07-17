use crate::config::MOUSE_SENSITIVITY;
use crate::config::WIDTH;
use crate::utils::view::View;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use crate::Model;
use crate::Camera;
use crate::CAMERA_SPEED;
use crate::{vec2, vec3, Vec3};



pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Model>,
}



impl Scene {
    pub fn new() -> Scene {
        return Scene { 
            camera: Camera::new(90.0),
            models: Vec::new(),
        }
    }


    pub fn load_model(&mut self, model: Model){
        self.models.push(model);
    }


    pub fn update(&mut self, delta_time: f32, view: &mut View){


        self.handle_input(view, delta_time);

        // 10 deg per sec
        self.models[0].transform.yaw += 10.0 * delta_time;
        self.models[0].transform.pitch += 10.0 * delta_time;
        // model.transform.position.x += 1.0 * delta_time;
    }


    fn handle_input(&mut self, view: &mut View, delta_time: f32){
        if view.mouse_pressed(MouseButton::Left) { view.set_mouse_grab(true);}
        if view.key_held(KeyCode::Escape) { view.set_mouse_grab(false);}
    
        if view.is_focused {
            let mouse_delta = (view.mouse_delta() / WIDTH as f32) * MOUSE_SENSITIVITY;
            let pitch = (self.camera.transform.pitch + mouse_delta.y)
                .clamp(-85.0, 85.0);
            let yaw = self.camera.transform.yaw - mouse_delta.x;
            self.camera.transform.set_rotation(pitch, yaw);
        }   
    
        let mut camera_delta = vec3!(0,0,0);
        let (cam_right, cam_up, cam_fwd) = self.camera.transform.basis_vectors;
    
        if view.key_held(KeyCode::KeyW) {camera_delta -= cam_fwd}
        if view.key_held(KeyCode::KeyA) {camera_delta -= cam_right}
        if view.key_held(KeyCode::KeyS) {camera_delta += cam_fwd}
        if view.key_held(KeyCode::KeyD) {camera_delta+= cam_right}

        let mut sprint = 1.0;
        if view.key_held(KeyCode::ShiftLeft) {sprint *= 2.8}
    
        self.camera.transform.position += camera_delta.normalize() * CAMERA_SPEED * delta_time * sprint;
    }
}



