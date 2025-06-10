use crate::{Vertex, Shader, Texture, Transform, Vec2, Vec3};


pub struct Model {
    pub vertices: Vec<Vertex>,
    pub colors: Vec<u32>,
    pub transform: Transform,
    pub shader: Box<dyn Shader>    
}
