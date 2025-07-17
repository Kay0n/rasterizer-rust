use crate::{graphics::camera::Camera, types::vertex::Vertex, vec2, vec3, Model, RenderTarget, Scene, Transform, Vec2, Vec3, Random};
use std::simd::{f32x8, u32x8, Simd, Mask, prelude::SimdPartialEq, prelude::SimdPartialOrd};



pub struct Rasterizer {
    poly_buffer1: Vec<Vertex>,
    poly_buffer2: Vec<Vertex>,
    final_triangles: Vec<[Vertex; 3]>,

    frustum_planes: [Plane; 6],
    cached_fov: f32,
    cached_aspect: f32,
    cached_near: f32,
    cached_far: f32,
}



impl Rasterizer {
    pub fn new() -> Rasterizer {
        return Self {
            poly_buffer1: Vec::with_capacity(10),
            poly_buffer2: Vec::with_capacity(10),
            final_triangles: Vec::with_capacity(8),

            // invalid values to force recalc TODO: frustum caching
            frustum_planes: [Plane::new(); 6],
            cached_fov: -1.0, 
            cached_aspect: -1.0,
            cached_near: -1.0,
            cached_far: -1.0,
        };
    }


    // TODO: separate into geometry, depth and color pass
    pub fn render(&mut self, render_target: &mut RenderTarget, scene: &Scene){
        // clear buffers
        render_target.color_buffer.fill(0);
        render_target.depth_buffer.fill(f32::NEG_INFINITY);

        let triangles: Vec<RasterTriangle> = self.geometry_setup_pass(scene, render_target);
        self.depth_pass(render_target, &triangles);

        self.color_pass(scene, render_target, &triangles);
        
        // println!("Tri count: {}", num_tris);
    }






    // Pass 1: Prepare all visible triangles for rasterization
    fn geometry_setup_pass(&mut self, scene: &Scene, render_target: &mut RenderTarget) -> Vec<RasterTriangle> {
        
        let mut raster_triangles: Vec<RasterTriangle> = Vec::new();
        let mut clipped_poly_buffer: Vec<Vertex> = Vec::with_capacity(10); // Capacity can be tuned

        for (model_index, model) in scene.models.iter().enumerate() {
            for i in (0..model.vertices.len()).step_by(3) {
                let v0 = &model.vertices[i];
                let v1 = &model.vertices[i + 1];
                let v2 = &model.vertices[i + 2];

                clipped_poly_buffer.clear();

                self.frustum_cull(&scene.camera, model, render_target, v0, v1, v2, &mut clipped_poly_buffer);

                if !clipped_poly_buffer.is_empty() {

                    self.final_triangles.clear();

                    triangulate_convex_polygon(&clipped_poly_buffer, &mut self.final_triangles);

                    for triangle in &self.final_triangles {
                        let tri_data: Option<RasterTriangle> = process_screen_tri(
                            &triangle[0], 
                            &triangle[1], 
                            &triangle[2], 
                            render_target, 
                            &scene.camera, 
                            model_index as u32
                        );
                        if let Some(val) = tri_data {
                            raster_triangles.push(val);
                        }
                        // else backface culled, discard
                    }
                }
                
            }
        }

        return raster_triangles;
    }

    fn frustum_cull(
        &mut self, 
        camera: &Camera, 
        model: &Model, 
        render_target: 
        &RenderTarget, 
        v0: &Vertex, 
        v1: &Vertex, 
        v2: &Vertex,
        output_polygon: &mut Vec<Vertex>,
    ) {

        let aspect = render_target.width as f32 / render_target.height as f32;
        let near_clip = 0.001;
        let far_clip = 40.0;
        let frustum_planes = build_frustum_planes(camera.fov, aspect, near_clip, far_clip);


        self.poly_buffer1.clear();
        self.poly_buffer1.push(Vertex {
            position: camera.transform.to_local_point(model.transform.to_world_point(v0.position)),
            texcoord: v0.texcoord,
            normal: camera.transform.to_local_vector(model.transform.to_world_vector(v0.normal)),
        });
        self.poly_buffer1.push(Vertex {
            position: camera.transform.to_local_point(model.transform.to_world_point(v1.position)),
            texcoord: v1.texcoord,
            normal: camera.transform.to_local_vector(model.transform.to_world_vector(v1.normal)),
        });
        self.poly_buffer1.push(Vertex {
            position: camera.transform.to_local_point(model.transform.to_world_point(v2.position)),
            texcoord: v2.texcoord,
            normal: camera.transform.to_local_vector(model.transform.to_world_vector(v2.normal)),
        });

        let mut input_poly = &mut self.poly_buffer1;
        let mut output_poly = &mut self.poly_buffer2;

        for plane in &frustum_planes {
            clip_polygon_against_plane(input_poly, output_poly, plane);
            
            std::mem::swap(&mut input_poly, &mut output_poly);

            if input_poly.is_empty() {
                break;
            }
        }
        
        if !input_poly.is_empty() {
            output_polygon.extend_from_slice(input_poly);
        }
    }



    fn depth_pass(&self, fb: &mut RenderTarget, triangles: &[RasterTriangle]) {
        let mut tri_count: i32 = 0;
        for tri in triangles {


            let mut tri_active = false;


            // optimize for barycentric coordinate calculation
            let simd_one = f32x8::splat(1.0);
            
            // start at top left of bounding box
            // let p_start = vec2!(tri.min_x as f32 + 0.5, tri.min_y as f32 + 0.5);

            let step_w0 = tri.simd_dy0 * f32x8::splat(8.0);
            let step_w1 = tri.simd_dy1 * f32x8::splat(8.0);
            let step_w2 = tri.simd_dy2 * f32x8::splat(8.0);

        

            // raster loop
            for y in tri.min_y..=tri.max_y {

                let simd_y: Simd<f32, 8> = f32x8::splat(y as f32 + 0.5);
        
                // simd weights
                let mut simd_w0 = edge_simd(f32x8::splat(tri.v2_2d.x), f32x8::splat(tri.v2_2d.y), f32x8::splat(tri.v3_2d.x), f32x8::splat(tri.v3_2d.y), tri.simd_x, simd_y);
                let mut simd_w1 = edge_simd(f32x8::splat(tri.v3_2d.x), f32x8::splat(tri.v3_2d.y), f32x8::splat(tri.v1_2d.x), f32x8::splat(tri.v1_2d.y), tri.simd_x, simd_y);
                let mut simd_w2 = edge_simd(f32x8::splat(tri.v1_2d.x), f32x8::splat(tri.v1_2d.y), f32x8::splat(tri.v2_2d.x), f32x8::splat(tri.v2_2d.y), tri.simd_x, simd_y);
                
                // 8 pixels at a time
                for x in (tri.min_x..=tri.max_x).step_by(8) {
        
                    // posibly have in y loop?
                    let mask = 
                        (simd_w0.simd_ge(f32x8::splat(0.0))) &
                        (simd_w1.simd_ge(f32x8::splat(0.0))) &
                        (simd_w2.simd_ge(f32x8::splat(0.0)));
        
                    // if any pixels in triangle
                    if mask.any() {
                        let index_start = (y * fb.width + x) as usize;
        
                        // let inv_depth_interp = (simd_w0 * tri.simd_inv_z1 + simd_w1 * tri.simd_inv_z2 + simd_w2 * tri.simd_inv_z3) / tri.simd_area;
                        let inv_depth_interp = (simd_w0 * tri.simd_inv_z1 + simd_w1 * tri.simd_inv_z2 + simd_w2 * tri.simd_inv_z3) * tri.simd_inv_area;
                        let depth = simd_one / inv_depth_interp;
                        
                        // load existing depth values, handles edge of screen
                        let mut current_depth = [0.0; 8];
                        let end_index = (index_start + 8).min(fb.depth_buffer.len());
                        current_depth[..end_index - index_start].copy_from_slice(&fb.depth_buffer[index_start..end_index]);
                        let simd_current_depth = f32x8::from_slice(&current_depth);
                        
                        let new_depth_mask = mask & depth.simd_gt(simd_current_depth);

                        if new_depth_mask.any() {
                            tri_active = true;
                            
                            for i in 0..8 {
                                if new_depth_mask.test(i) {
                                    let current_x = x + i as u32;
                                    if current_x <= tri.max_x {
                                        let index = index_start + i;

                                        fb.depth_buffer[index] = depth[i];

                                    }
                                }
                            }
                        }
                    }
                    // update barycentric coords
                    simd_w0 += step_w0;
                    simd_w1 += step_w1;
                    simd_w2 += step_w2;
                }
            }

            if tri_active == true {
                tri_count += 1;
            }
        }
        println!("total_tris: {tri_count}")

    }


    fn color_pass(&self, scene: &Scene, fb: &mut RenderTarget, triangles: &[RasterTriangle]) {

        let mut rng = Random::new(654);
        let simd_one = f32x8::splat(1.0);

        let mut tri_count: i32 = 0;
        for tri in triangles {

            let color = rng.random_argb();
            let mut tri_active = false;

            let model = &scene.models[tri.model_index as usize]; // Get the corresponding model

            let step_w0 = tri.simd_dy0 * f32x8::splat(8.0);
            let step_w1 = tri.simd_dy1 * f32x8::splat(8.0);
            let step_w2 = tri.simd_dy2 * f32x8::splat(8.0);

            for y in tri.min_y..=tri.max_y {
                let simd_y = f32x8::splat(y as f32 + 0.5);

                let mut simd_w0 = edge_simd(f32x8::splat(tri.v2_2d.x), f32x8::splat(tri.v2_2d.y), f32x8::splat(tri.v3_2d.x), f32x8::splat(tri.v3_2d.y), tri.simd_x, simd_y);
                let mut simd_w1 = edge_simd(f32x8::splat(tri.v3_2d.x), f32x8::splat(tri.v3_2d.y), f32x8::splat(tri.v1_2d.x), f32x8::splat(tri.v1_2d.y), tri.simd_x, simd_y);
                let mut simd_w2 = edge_simd(f32x8::splat(tri.v1_2d.x), f32x8::splat(tri.v1_2d.y), f32x8::splat(tri.v2_2d.x), f32x8::splat(tri.v2_2d.y), tri.simd_x, simd_y);

                for x in (tri.min_x..=tri.max_x).step_by(8) {
                    let inside_mask =
                        simd_w0.simd_ge(f32x8::splat(0.0)) &
                        simd_w1.simd_ge(f32x8::splat(0.0)) &
                        simd_w2.simd_ge(f32x8::splat(0.0));

                    if inside_mask.any() {
                        let index_start = (y * fb.width + x) as usize;

                        let inv_depth_interp = (simd_w0 * tri.simd_inv_z1 + simd_w1 * tri.simd_inv_z2 + simd_w2 * tri.simd_inv_z3) * tri.simd_inv_area;
                        let depth = simd_one / inv_depth_interp;

                        let mut current_depth = [f32::NEG_INFINITY; 8];
                        let end_index = (index_start + 8).min(fb.depth_buffer.len());
                        current_depth[..end_index - index_start].copy_from_slice(&fb.depth_buffer[index_start..end_index]);
                        let simd_current_depth = f32x8::from_slice(&current_depth);

                        let color_mask = inside_mask & depth.simd_ge(simd_current_depth);

                        if color_mask.any() {

                            tri_active = true;
                            let u_over_z = (simd_w0 * tri.simd_u1_over_z + simd_w1 * tri.simd_u2_over_z + simd_w2 * tri.simd_u3_over_z) * tri.simd_inv_area;
                            let v_over_z = (simd_w0 * tri.simd_v1_over_z + simd_w1 * tri.simd_v2_over_z + simd_w2 * tri.simd_v3_over_z) * tri.simd_inv_area;

                            let nx_over_z = (simd_w0 * tri.simd_nx1_over_z + simd_w1 * tri.simd_nx2_over_z + simd_w2 * tri.simd_nx3_over_z) * tri.simd_inv_area;
                            let ny_over_z = (simd_w0 * tri.simd_ny1_over_z + simd_w1 * tri.simd_ny2_over_z + simd_w2 * tri.simd_ny3_over_z) * tri.simd_inv_area;
                            let nz_over_z = (simd_w0 * tri.simd_nz1_over_z + simd_w1 * tri.simd_nz2_over_z + simd_w2 * tri.simd_nz3_over_z) * tri.simd_inv_area;

                            let tex_u = u_over_z * depth;
                            let tex_v = v_over_z * depth;

                            let normal_x = nx_over_z * depth;
                            let normal_y = ny_over_z * depth;
                            let normal_z = nz_over_z * depth;

                            for i in 0..8 {
                                if color_mask.test(i) {
                                    let current_x = x + i as u32;
                                    if current_x <= tri.max_x {
                                        let index = index_start + i;

                                       

                                        // --- DEBUG: visualize normals as colors ---
                                        let normal = vec3!(normal_x[i], normal_y[i], normal_z[i]).normalize();
                                        let r = ((normal.x * 0.5 + 0.5) * 255.0).round() as u32;
                                        let g = ((normal.y * 0.5 + 0.5) * 255.0).round() as u32;
                                        let b = ((normal.z * 0.5 + 0.5) * 255.0).round() as u32;
                                        let normal_color = (0xFF << 24) | (r << 16) | (g << 8) | b;

                                        fb.color_buffer[index] = color;
                                    }
                                }
                            }
                        }
                    }
                    simd_w0 += step_w0;
                    simd_w1 += step_w1;
                    simd_w2 += step_w2;
                }
            }
            if tri_active == true {
                tri_count += 1;
            }
        }
        println!("color_tris: {tri_count}")
    }

}


#[inline]
fn edge(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

#[inline]
fn edge_simd(ax: f32x8, ay: f32x8, bx: f32x8, by: f32x8, cx: f32x8, cy: f32x8) -> f32x8 {
    (cx - ax) * (by - ay) - (cy - ay) * (bx - ax)
}


#[derive(Debug, Clone, Copy)]
struct Plane {
    normal: Vec3,
    d: f32,
}



impl Plane {
    pub fn new() -> Plane{
        return Plane {
            normal: vec3!(0,0,0),
            d: 0.0
        };
    }

    fn distance(&self, p: Vec3) -> f32 {
        self.normal.dot(p) + self.d
    }
}

/// returns `Option<RasterTriangle>` to handle backface culling.
fn process_screen_tri(
    v1: &Vertex,
    v2: &Vertex,
    v3: &Vertex,
    fb: &mut RenderTarget,
    cam: &Camera,
    model_index:u32,
    // self might be needed if vertex_to_screen or edge are methods on it
) -> Option<RasterTriangle> {
    // project vertices to screen space
    let p1_screen = vertex_to_screen(v1.position, fb, cam);
    let p2_screen = vertex_to_screen(v2.position, fb, cam);
    let p3_screen = vertex_to_screen(v3.position, fb, cam);

    let v1_2d = vec2!(p1_screen.x, p1_screen.y);
    let v2_2d = vec2!(p2_screen.x, p2_screen.y);
    let v3_2d = vec2!(p3_screen.x, p3_screen.y);

    // backface cull for clockwise? winding
    let area = edge(v1_2d, v2_2d, v3_2d);
    if area <= 0.0 {
        return None; 
    }

    let inv_area = 1.0 / area;

    // calc bounding box
    let min_x = v1_2d.x.min(v2_2d.x).min(v3_2d.x).floor().max(0.0) as u32;
    let mut max_x = v1_2d.x.max(v2_2d.x).max(v3_2d.x).ceil() as i32;
    let min_y = v1_2d.y.min(v2_2d.y).min(v3_2d.y).floor().max(0.0) as u32;
    let mut max_y = v1_2d.y.max(v2_2d.y).max(v3_2d.y).ceil() as i32;

    max_x = max_x.min(fb.width as i32 - 1);
    max_y = max_y.min(fb.height as i32 - 1);

    if max_x < min_x as i32 || max_y < min_y as i32 {
        return None; // tri out of screen bounds, covered by frustum cull?
    }

    // precompute interpolation data
    let inv_z1 = 1.0 / v1.position.z; 
    let inv_z2 = 1.0 / v2.position.z;
    let inv_z3 = 1.0 / v3.position.z;

    let uv1_over_z = v1.texcoord * inv_z1;
    let uv2_over_z = v2.texcoord * inv_z2;
    let uv3_over_z = v3.texcoord * inv_z3;

    let n1_over_z = v1.normal * inv_z1;
    let n2_over_z = v2.normal * inv_z2;
    let n3_over_z = v3.normal * inv_z3;

    let simd_x = f32x8::from_array([
        min_x as f32 + 0.5, min_x as f32 + 1.5, min_x as f32 + 2.5, min_x as f32 + 3.5,
        min_x as f32 + 4.5, min_x as f32 + 5.5, min_x as f32 + 6.5, min_x as f32 + 7.5
    ]);

    // 5. Pre-splat all values for SIMD
    Some(RasterTriangle {
        min_x: min_x as u32,
        max_x: max_x as u32,
        min_y: min_y as u32,
        max_y: max_y as u32,
        v1_2d,
        v2_2d,
        v3_2d,
        simd_x,
        simd_dy0: f32x8::splat(v3_2d.y - v2_2d.y),
        simd_dy1: f32x8::splat(v1_2d.y - v3_2d.y),
        simd_dy2: f32x8::splat(v2_2d.y - v1_2d.y),
        simd_area: f32x8::splat(area),
        simd_inv_area: f32x8::splat(inv_area),
        simd_inv_z1: f32x8::splat(inv_z1),
        simd_inv_z2: f32x8::splat(inv_z2),
        simd_inv_z3: f32x8::splat(inv_z3),
        simd_u1_over_z: f32x8::splat(uv1_over_z.x),
        simd_v1_over_z: f32x8::splat(uv1_over_z.y),
        simd_u2_over_z: f32x8::splat(uv2_over_z.x),
        simd_v2_over_z: f32x8::splat(uv2_over_z.y),
        simd_u3_over_z: f32x8::splat(uv3_over_z.x),
        simd_v3_over_z: f32x8::splat(uv3_over_z.y),
        simd_nx1_over_z: f32x8::splat(n1_over_z.x),
        simd_ny1_over_z: f32x8::splat(n1_over_z.y),
        simd_nz1_over_z: f32x8::splat(n1_over_z.z),
        simd_nx2_over_z: f32x8::splat(n2_over_z.x),
        simd_ny2_over_z: f32x8::splat(n2_over_z.y),
        simd_nz2_over_z: f32x8::splat(n2_over_z.z),
        simd_nx3_over_z: f32x8::splat(n3_over_z.x),
        simd_ny3_over_z: f32x8::splat(n3_over_z.y),
        simd_nz3_over_z: f32x8::splat(n3_over_z.z),
        model_index
    })
}



#[derive(Debug, Clone, Copy)]
pub struct RasterTriangle {
    // Bounding box for the rasterizer
    pub min_x: u32,
    pub max_x: u32,
    pub min_y: u32,
    pub max_y: u32,

    // For SIMD barycentric calculation
    pub v1_2d: Vec2, // We need the original 2D vectors
    pub v2_2d: Vec2,
    pub v3_2d: Vec2,

    // Pre-splatted deltas for updating barycentric coordinates
    pub simd_dy0: f32x8,
    pub simd_dy1: f32x8,
    pub simd_dy2: f32x8,

    pub simd_x: f32x8,

    // Pre-splatted inverse area for barycentric normalization
    pub simd_inv_area: f32x8,
    pub simd_area: f32x8,

    // Pre-splatted data for perspective-correct interpolation
    pub simd_inv_z1: f32x8,
    pub simd_inv_z2: f32x8,
    pub simd_inv_z3: f32x8,

    pub simd_u1_over_z: f32x8,
    pub simd_v1_over_z: f32x8,
    pub simd_u2_over_z: f32x8,
    pub simd_v2_over_z: f32x8,
    pub simd_u3_over_z: f32x8,
    pub simd_v3_over_z: f32x8,

    pub simd_nx1_over_z: f32x8,
    pub simd_ny1_over_z: f32x8,
    pub simd_nz1_over_z: f32x8,
    pub simd_nx2_over_z: f32x8,
    pub simd_ny2_over_z: f32x8,
    pub simd_nz2_over_z: f32x8,
    pub simd_nx3_over_z: f32x8,
    pub simd_ny3_over_z: f32x8,
    pub simd_nz3_over_z: f32x8,

    // Optional: Triangle ID for debugging or advanced techniques
    // pub triangle_id: u32,
    pub model_index: u32,
    // You could add material_index, etc., as needed
}


fn vertex_to_screen(vertex_view: Vec3, target: &mut RenderTarget, cam: &Camera) -> Vec3 {

    let world_height = (cam.fov.to_radians() / 2.0).tan() * 2.0; // TODO: not run every time?
    let pixels_per_world_unit: f32 = (target.height as f32 / world_height) / -vertex_view.z;

    let screen_center = vec2!(target.width as f32 / 2.0, target.height as f32 / 2.0);
    let pixel_offset = vec2!(vertex_view.x * pixels_per_world_unit, -vertex_view.y * pixels_per_world_unit);
    let vertex_screen = screen_center + pixel_offset;
    return vec3!(vertex_screen.x, vertex_screen.y, vertex_view.z)
}




fn build_frustum_planes(fov: f32, aspect: f32, near: f32, far: f32) -> [Plane; 6] {
    let tan_half_fov = (fov.to_radians() / 2.0).tan();
    let slope_y = tan_half_fov;
    let slope_x = tan_half_fov * aspect;

    [
        Plane { normal: vec3!(0.0, 0.0, -1.0), d: -near },                 // NEAR
        Plane { normal: vec3!(0.0, 0.0, 1.0), d: far },                    // FAR
        Plane { normal: vec3!(-1.0, 0.0, -slope_x).normalize(), d: 0.0 },  // RIGHT
        Plane { normal: vec3!(1.0, 0.0, -slope_x).normalize(), d: 0.0 },  // LEFT
        Plane { normal: vec3!(0.0, -1.0, -slope_y).normalize(), d: 0.0 }, // TOP
        Plane { normal: vec3!(0.0, 1.0, -slope_y).normalize(), d: 0.0 },  // BOTTOM
    ]
}



fn clip_polygon_against_plane(input_poly: &[Vertex], output_poly: &mut Vec<Vertex>, plane: &Plane) {
    output_poly.clear();
    if input_poly.is_empty() {
        return;
    }

    let mut prev_v = input_poly.last().unwrap();
    let mut prev_dist = plane.distance(prev_v.position);

    for curr_v in input_poly {
        let curr_dist = plane.distance(curr_v.position);

        let prev_inside = prev_dist >= 0.0;
        let curr_inside = curr_dist >= 0.0;

        if prev_inside != curr_inside {
            let t = prev_dist / (prev_dist - curr_dist);

            let intersection = Vertex {
                position: prev_v.position + (curr_v.position - prev_v.position) * t,
                texcoord: prev_v.texcoord + (curr_v.texcoord - prev_v.texcoord) * t,
                normal: prev_v.normal + (curr_v.normal - prev_v.normal) * t,
            };
            output_poly.push(intersection);
        }

        if curr_inside {
            output_poly.push(Vertex { ..*curr_v });
        }

        prev_v = curr_v;
        prev_dist = curr_dist;
    }
}



fn triangulate_convex_polygon(polygon: &Vec<Vertex>, triangles: &mut Vec<[Vertex; 3]>) {
    if polygon.len() < 3 {
        return;
    }
    
    let v0 = polygon[0];
    for i in 1..(polygon.len() - 1) {
        triangles.push([v0, polygon[i], polygon[i + 1]]);
    }
}

