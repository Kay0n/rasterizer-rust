use crate::{Transform, Vec3};


pub struct Model {
    pub points: Vec<Vec3>,
    pub colors: Vec<u32>,
    pub transform: Transform
}
