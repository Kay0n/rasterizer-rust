use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::types::shader::SolidShader;
use crate::types::vertex::Vertex;
use crate::Model;
use crate::{Vec3, vec3, Vec2, vec2};
use crate::Random;
use crate::Transform;
use crate::Texture;
use std::io::{Read, Seek, SeekFrom, Result, Error, ErrorKind};



pub fn parse_obj(obj_file_path: &str) -> Model {
    let file = File::open(obj_file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut texcoords: Vec<Vec2> = Vec::new();     
    let mut normals: Vec<Vec3> = Vec::new();

    let mut triangulated_vertices: Vec<Vertex> = Vec::new();

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
            "vt" => {
                let u: f32 = parts[1].parse().expect("Failed to parse texcoord");
                let v: f32 = parts[2].parse().expect("Failed to parse texcoord");
                texcoords.push(Vec2 { x: u, y: v });
            }
            "vn" => {
                let x: f32 = parts[1].parse().expect("Failed to parse normal");
                let y: f32 = parts[2].parse().expect("Failed to parse normal");
                let z: f32 = parts[3].parse().expect("Failed to parse normal");
                normals.push(Vec3 { x, y, z });
            }
            "f" => {
                let face_vertices = parse_face(parts, &vertices, &texcoords, &normals);
                triangulated_vertices.extend(triangulate_face(&face_vertices));
            }
            
            _ => {}
        }
    }

    // random tri colors
    let mut rng = Random::new(4676319);
    let num_triangles = triangulated_vertices.len() / 3;
    let mut tri_colors: Vec<u32> = Vec::with_capacity(num_triangles);
    for _ in 0..num_triangles {
        tri_colors.push(rng.random_argb());
    }

    Model {
        vertices: triangulated_vertices,
        colors: tri_colors,
        transform: Transform::new(),
        shader: Box::new(SolidShader::new()),
    }
}



fn parse_face(parts: Vec<&str>, vertices: &[Vec3], texture_coords: &[Vec2], normals: &[Vec3]) -> Vec<Vertex> {
    let mut face_vertices: Vec<Vertex> = Vec::new();

    for part in &parts[1..] {
        let indices: Vec<&str> = part.split('/').collect();

        let vertex_index: usize = indices[0].parse().expect("Missing vertex index");
        let position = vertices[vertex_index - 1]; // 1-based index

        let mut texcoord = vec2!(0.0, 0.0 );
        if indices.len() > 1 && !indices[1].is_empty() {
            let texture_index: usize = indices[1].parse().expect("Failed to parse texture index");
            if texture_index > 0 && texture_index <= texture_coords.len() {
                texcoord = texture_coords[texture_index - 1]; // 1-based index
            } else {
                eprintln!(
                    "Warning: Texture index {} out of bounds (max {}). Using default UV (0,0).",
                    texture_index,
                    texture_coords.len()
                );
            }
        }

        let mut normal = vec3!(0.0, 0.0, 0.0); 
        if indices.len() > 2 && !indices[2].is_empty() {
            let normal_index: usize = indices[2].parse().expect("Failed to parse normal index");
            if normal_index > 0 && normal_index <= normals.len() {
                normal = normals[normal_index - 1]; // 1-based index
            } else {
                eprintln!(
                    "Warning: Normal index {} out of bounds (max {}). Using default normal (0,0,0).",
                    normal_index,
                    normals.len()
                );
            }
        }
        
        face_vertices.push(Vertex { position, texcoord, normal });
    }
    return face_vertices;
}



fn triangulate_face(polygon: &[Vertex]) -> Vec<Vertex> {
    if polygon.len() < 3 {
        return vec![];
    }

    let mut triangles = Vec::with_capacity((polygon.len() - 2) * 3);
    let anchor = polygon[0];

    for i in 1..(polygon.len() - 1) {
        triangles.push(anchor);
        triangles.push(polygon[i]);
        triangles.push(polygon[i + 1]);
    }

    return triangles;
}



pub fn read_bitmap(path: &str) -> Result<Texture> {
    let mut file = File::open(path)?;

    let mut header_bytes = [0; 14];
    file.read_exact(&mut header_bytes)?;

    if &header_bytes[0..2] != b"BM" {
        return Err(Error::new(ErrorKind::InvalidData, "Not a valid BMP file: Missing 'BM' signature"));
    }

    let pixel_data_offset = u32::from_le_bytes(header_bytes[10..14].try_into().unwrap());

    let mut dib_header_bytes = [0; 40];
    file.read_exact(&mut dib_header_bytes)?;

    let dib_header_size = u32::from_le_bytes(dib_header_bytes[0..4].try_into().unwrap());
    if dib_header_size != 40 {
        return Err(Error::new(ErrorKind::InvalidData, "Unsupported DIB header size. Only BITMAPINFOHEADER (40 bytes) is supported."));
    }

    let width = i32::from_le_bytes(dib_header_bytes[4..8].try_into().unwrap()) as u32;
    let height = i32::from_le_bytes(dib_header_bytes[8..12].try_into().unwrap()) as u32;
    let bits_per_pixel = u16::from_le_bytes(dib_header_bytes[14..16].try_into().unwrap());
    let compression_method = u32::from_le_bytes(dib_header_bytes[16..20].try_into().unwrap());

    if bits_per_pixel != 24 {
        return Err(Error::new(ErrorKind::InvalidData, format!("Unsupported bits per pixel: {}. Only 24-bit BMPs are supported.", bits_per_pixel)));
    }
    if compression_method != 0 {
        return Err(Error::new(ErrorKind::InvalidData, "Unsupported compression method. Only uncompressed (BI_RGB) BMPs are supported."));
    }

    let row_padding = (4 - (width * 3) % 4) % 4;
    let row_size_bytes = (width * 3 + row_padding) as usize;

    file.seek(SeekFrom::Start(pixel_data_offset as u64))?;

    let mut texture_data = vec![0u32; (width * height) as usize];
    let mut row_buffer = vec![0u8; row_size_bytes];

    // bottom to top
    for y_bmp in 0..height { 
        file.read_exact(&mut row_buffer)?;

        let y_tex = height - 1 - y_bmp;
        let base_index_tex_row = (y_tex * width) as usize;

        for x in 0..width {
            let bgr_idx = (x * 3) as usize;
            let b = row_buffer[bgr_idx];
            let g = row_buffer[bgr_idx + 1];
            let r = row_buffer[bgr_idx + 2];

            let pixel_color = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            texture_data[base_index_tex_row + (x as usize)] = pixel_color;
        }
    }

    Ok(Texture {
        data: texture_data,
        width,
        height,
    })
}