use crate::Model;
use crate::Camera;

pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Model>,
    //pub models: Vec<&'a Model>,??
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

    pub fn update(&mut self, delta_time: f32){

            // 10 deg per sec
            self.models[0].transform.yaw += 10.0 * delta_time;
            self.models[0].transform.pitch += 10.0 * delta_time;
            // model.transform.position.x += 1.0 * delta_time;
    }

}