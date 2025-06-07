use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::math::vectors::*;
use crate::math::rng::*;



pub struct Model {
    pub points: Vec<Vec3>,
    pub colors: Vec<u32>,
    pub transform: Transform
}



pub struct Transform {
    pub yaw: f32,
    pub pitch: f32,
    pub position: Vec3,
}



impl Transform {
    pub fn new(yaw: f32, pitch:f32, position:Vec3) -> Transform{
        return Transform { yaw, pitch, position }
    }


    fn get_basis_vectors(&self) -> (Vec3, Vec3, Vec3) {

        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        let ihat_yaw = vec3!(yaw_rad.cos(), 0.0, -yaw_rad.sin());
        let jhat_yaw = vec3!(0.0, 1.0, 0.0); // No change needed here
        let khat_yaw = vec3!(yaw_rad.sin(), 0.0, yaw_rad.cos());

        let ihat_pitch = vec3!(1, 0, 0);
        let jhat_pitch = vec3!(0, pitch_rad.cos(), -pitch_rad.sin());
        let khat_pitch = vec3!(0, pitch_rad.sin(), pitch_rad.cos());

        let ihat = transform_vector(ihat_yaw, jhat_yaw, khat_yaw, ihat_pitch);
        let jhat = transform_vector(ihat_yaw, jhat_yaw, khat_yaw, jhat_pitch);
        let khat = transform_vector(ihat_yaw, jhat_yaw, khat_yaw, khat_pitch);

        return (ihat, jhat, khat);
    }


    pub fn to_world_point(&self, point: Vec3) -> Vec3 {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        return transform_vector(ihat, jhat, khat, point) + self.position;
    }

    
}



fn transform_vector(ihat: Vec3, jhat: Vec3, khat: Vec3, vector: Vec3) -> Vec3 {
    return ihat * vector.x  + jhat * vector.y + khat * vector.z;
}



// loads .obj files into models
pub fn get_model(obj_file_path: &str) -> Model{
    let file = File::open(obj_file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    
    let mut vertices = Vec::new();
    let mut triangulated_points = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                let x: f32 = parts[1].parse().expect("Failed to parse vertex coordinate");
                let y: f32 = parts[2].parse().expect("Failed to parse vertex coordinate");
                let z: f32 = parts[3].parse().expect("Failed to parse vertex coordinate");
                vertices.push(Vec3 { x, y, z });
            }
            "f" => {
                // collect all face vertices 
                let mut face_vertices = Vec::new();
                for part in &parts[1..] {
                    let vertex_indices: Vec<&str> = part.split('/').collect();
                    let vertex_index: usize = vertex_indices[0].parse().expect("Failed to parse vertex index");
                    face_vertices.push(vertices[vertex_index - 1]);
                }

                // triangulate face
                if face_vertices.len() >= 3 {
                    let triangles = triangulate_face(&face_vertices);
                    triangulated_points.extend(triangles);
                }
                else {
                    panic!("Face should have at least 3 vertices")
                }
            }
            _ => {}
        }
    }

    // random tri colors
    let mut rng = SimpleRng::new(423435);
    let num_triangles = triangulated_points.len() / 3;
    let mut tri_colors: Vec<u32> = Vec::with_capacity(num_triangles);
    for _ in 0..num_triangles {
        tri_colors.push(rng.random_argb());
    }

    Model {
        points: triangulated_points,
        colors: tri_colors, 
        transform: Transform::new(0.0, 0.0, vec3!(0,0,0))
    }
}



// fan algorithm, TODO: earclipping
fn triangulate_face(polygon: &[Vec3]) -> Vec<Vec3> {
    if polygon.len() < 3 {
        return vec![];
    }
    
    let mut triangles = Vec::with_capacity((polygon.len() - 2) * 3);
    let anchor_vertex = polygon[0];
    
    for i in 1..(polygon.len() - 1) {
        triangles.push(anchor_vertex);
        triangles.push(polygon[i]);
        triangles.push(polygon[i + 1]);
    }
    
    return triangles
}



