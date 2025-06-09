use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Model;
use crate::{Vec3, vec3};
use crate::Random;
use crate::Transform;



pub fn parse_obj(obj_file_path: &str) -> Model {
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
    let mut rng = Random::new(4676319);
    let num_triangles = triangulated_points.len() / 3;
    let mut tri_colors: Vec<u32> = Vec::with_capacity(num_triangles);
    for _ in 0..num_triangles {
        tri_colors.push(rng.random_argb());
    }

    Model {
        points: triangulated_points,
        colors: tri_colors, 
        transform: Transform::new()
    }
}


// fn parse_faces(){

// }

// fn parse_points()


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

    return triangles;
}



