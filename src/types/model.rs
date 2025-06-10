use crate::{Transform, Vec3};


pub struct Model {
    pub vertices: Vec<Vec3>,
    pub colors: Vec<u32>,
    pub transform: Transform
}
