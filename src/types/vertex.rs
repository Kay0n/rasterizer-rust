use crate::{Vec2, Vec3};


#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub texcoord: Vec2,
    pub normal: Vec3,
}