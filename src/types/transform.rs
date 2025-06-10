
use crate::{
    vec3,Vec3
};



pub struct Transform {
    pub yaw: f32,
    pub pitch: f32,
    pub position: Vec3,
}



impl Transform {
    pub fn new() -> Transform{
        return Transform { 
            yaw: 0.0, 
            pitch: 0.0, 
            position: vec3!(0,0,0) 
        }
    }


    pub fn get_basis_vectors(&self) -> (Vec3, Vec3, Vec3) {

        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        let ihat_yaw = vec3!(yaw_rad.cos(), 0.0, -yaw_rad.sin());
        let jhat_yaw = vec3!(0.0, 1.0, 0.0); 
        let khat_yaw = vec3!(yaw_rad.sin(), 0.0, yaw_rad.cos());

        let ihat_pitch = vec3!(1, 0, 0);
        let jhat_pitch = vec3!(0, pitch_rad.cos(), -pitch_rad.sin());
        let khat_pitch = vec3!(0, pitch_rad.sin(), pitch_rad.cos());

        let ihat = Self::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, ihat_pitch);
        let jhat = Self::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, jhat_pitch);
        let khat = Self::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, khat_pitch);

        return (ihat, jhat, khat);
    }


    fn get_inverse_basis_vectors(&self) -> (Vec3, Vec3, Vec3) {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        return (
            vec3!(ihat.x, jhat.x, khat.x),
            vec3!(ihat.y, jhat.y, khat.y),
            vec3!(ihat.z, jhat.z, khat.z),
        );
    }


    pub fn to_world_point(&self, point: Vec3) -> Vec3 {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        return Self::transform_vector(ihat, jhat, khat, point) + self.position;
    }


    pub fn to_local_point(&self, world_point: Vec3) -> Vec3{
        let (ihat, jhat, khat) = self.get_inverse_basis_vectors();
        return Self::transform_vector(ihat, jhat, khat, world_point - self.position);
    }

    
    fn transform_vector(ihat: Vec3, jhat: Vec3, khat: Vec3, vector: Vec3) -> Vec3 {
        ihat * vector.x + jhat * vector.y + khat * vector.z
    }
}

