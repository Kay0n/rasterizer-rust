
use crate::{Transform, vec3};



pub struct Camera {
    pub fov: f32,
    pub transform: Transform,
}



impl Camera {
    pub fn new(fov: f32) -> Camera {
        Camera {
            fov, 
            transform: Transform::new(),
        }
    }
}




