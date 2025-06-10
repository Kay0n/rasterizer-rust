
use core::str;

use crate::{Texture, Vec2, Vec3};


pub trait Shader {


    fn pixel_color(&self, texcoord: Vec2, normal: Vec3) -> u32;
}


pub struct TextureShader {
    texture: Texture,
}

impl TextureShader {
    pub fn new(texture: Texture) -> Self {
        return Self { texture }
    }
}

impl Shader for TextureShader{

    fn pixel_color(&self, texcoord: Vec2, normal: Vec3) -> u32 {
        return self.texture.sample(texcoord.x, texcoord.y);
    }
}


pub struct SolidShader;

impl SolidShader {
    pub fn new() -> Self {
        return Self;
    }
}

impl Shader for SolidShader{
    fn pixel_color(&self, texcoord: Vec2, normal: Vec3) -> u32 {
        return 0xFFFFFFFF;
    }
}