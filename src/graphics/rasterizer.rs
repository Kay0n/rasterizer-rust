use crate::{graphics::camera::Camera, vec2, vec3, Model, RenderTarget, Scene, Transform, Vec2, Vec3};
use std::simd::{f32x8, u32x8, Simd, Mask, prelude::SimdPartialEq, prelude::SimdPartialOrd};


pub struct Rasterizer {
    poly_buffer1: Vec<Vec3>,
    poly_buffer2: Vec<Vec3>,
    final_triangles: Vec<[Vec3; 3]>,

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


    pub fn render(&mut self, render_target: &mut RenderTarget, scene: &Scene){
        // clear buffers
        render_target.color_buffer.fill(0);
        render_target.depth_buffer.fill(f32::NEG_INFINITY);

        let mut num_tris = 0;
        let aspect = render_target.width as f32 / render_target.height as f32;
        let near_clip = 0.001;
        let far_clip = 40.0;
        let frustum_planes = build_frustum_planes(scene.camera.fov, aspect, near_clip, far_clip);

        for model in &scene.models {
            for i in (0..model.points.len()).step_by(3) {

                self.poly_buffer1.clear();
                self.poly_buffer1.push(scene.camera.transform.to_local_point(model.transform.to_world_point(model.points[i])));
                self.poly_buffer1.push(scene.camera.transform.to_local_point(model.transform.to_world_point(model.points[i + 1])));
                self.poly_buffer1.push(scene.camera.transform.to_local_point(model.transform.to_world_point(model.points[i + 2])));

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
                    self.final_triangles.clear();
                    triangulate_convex_polygon(input_poly, &mut self.final_triangles);
    
                    for triangle in &self.final_triangles {
                        let a = self.vertex_to_screen(triangle[0], render_target, &scene.camera);
                        let b = self.vertex_to_screen(triangle[1], render_target, &scene.camera);
                        let c = self.vertex_to_screen(triangle[2], render_target, &scene.camera);

                        self.draw_triangle(render_target, a, b, c, model.colors[i / 3]);
                        num_tris += 1
                    }
                }
            }
        }
        println!("Tri count: {}", num_tris);
    }


    fn vertex_to_screen(&self, vertex_view: Vec3, target: &mut RenderTarget, cam: &Camera) -> Vec3 {

        // let vertex_view = cam.transform.to_local_point(vertex_world);

        let world_height = (cam.fov.to_radians() / 2.0).tan() * 2.0; // TODO: not run every time?
        let pixels_per_world_unit: f32 = (target.height as f32 / world_height) / -vertex_view.z;
    
        let screen_center = vec2!(target.width as f32 / 2.0, target.height as f32 / 2.0);
        let pixel_offset = vec2!(vertex_view.x * pixels_per_world_unit, -vertex_view.y * pixels_per_world_unit);
        let vertex_screen = screen_center + pixel_offset;
        return vec3!(vertex_screen.x, vertex_screen.y, vertex_view.z)
    }


    fn draw_triangle(&self, fb: &mut RenderTarget, p1: Vec3, p2: Vec3, p3: Vec3, color: u32) {

        let v1_2d = vec2!(p1.x, p1.y);
        let v2_2d = vec2!(p2.x, p2.y);
        let v3_2d = vec2!(p3.x, p3.y);
          
        // calculate bounding box
        let min_x = v1_2d.x.min(v2_2d.x).min(v3_2d.x).floor().max(0.0) as u32;
        let mut max_x = v1_2d.x.max(v2_2d.x).max(v3_2d.x).ceil() as i32;
        let min_y = v1_2d.y.min(v2_2d.y).min(v3_2d.y).floor().max(0.0) as u32;
        let mut max_y = v1_2d.y.max(v2_2d.y).max(v3_2d.y).ceil() as i32;
        
        // clamp max values to screen dimensions
        max_x = max_x.min(fb.width as i32 - 1);
        max_y = max_y.min(fb.height as i32 - 1);
        
        // if bounding box invalid / entirely off screen
        if max_x < min_x as i32 || max_y < min_y as i32 {
            return;
        }
        
        let max_x = max_x as u32;
        let max_y = max_y as u32;
        
        // optimize for barycentric coordinate calculation
        let dy0 = v3_2d.y - v2_2d.y; 
        let dx0 = v2_2d.x - v3_2d.x; 
        let dy1 = v1_2d.y - v3_2d.y;
        let dx1 = v3_2d.x - v1_2d.x;
        let dy2 = v2_2d.y - v1_2d.y;
        let dx2 = v1_2d.x - v2_2d.x;

        let simd_dy0 = f32x8::splat(dy0);
        let simd_dy1 = f32x8::splat(dy1);
        let simd_dy2 = f32x8::splat(dy2);
        
        // start at top left of bounding box
        let p_start = vec2!(min_x as f32 + 0.5, min_y as f32 + 0.5);
        
        // backface cull; area negative or zero
        let area = self.edge(v1_2d, v2_2d, v3_2d);
        if area <= 0.0 { 
            return;
        }
    
        let inv_z1 = 1.0 / p1.z;
        let inv_z2 = 1.0 / p2.z;
        let inv_z3 = 1.0 / p3.z;

        let simd_inv_z1 = f32x8::splat(inv_z1);
        let simd_inv_z2 = f32x8::splat(inv_z2);
        let simd_inv_z3 = f32x8::splat(inv_z3);
        let simd_area = f32x8::splat(area);
        let simd_one = f32x8::splat(1.0);
        
        // initial barycentric weights
        let mut w0_row_start = self.edge(v2_2d, v3_2d, p_start);
        let mut w1_row_start = self.edge(v3_2d, v1_2d, p_start);
        let mut w2_row_start = self.edge(v1_2d, v2_2d, p_start);

        // raster loop
        for y in min_y..=max_y {

            let simd_x = f32x8::from_array([
                min_x as f32 + 0.5, min_x as f32 + 1.5, min_x as f32 + 2.5, min_x as f32 + 3.5,
                min_x as f32 + 4.5, min_x as f32 + 5.5, min_x as f32 + 6.5, min_x as f32 + 7.5
            ]);
            let simd_y = f32x8::splat(y as f32 + 0.5);
    
            // simd weights
            let mut simd_w0 = self.edge_simd(f32x8::splat(v2_2d.x), f32x8::splat(v2_2d.y), f32x8::splat(v3_2d.x), f32x8::splat(v3_2d.y), simd_x, simd_y);
            let mut simd_w1 = self.edge_simd(f32x8::splat(v3_2d.x), f32x8::splat(v3_2d.y), f32x8::splat(v1_2d.x), f32x8::splat(v1_2d.y), simd_x, simd_y);
            let mut simd_w2 = self.edge_simd(f32x8::splat(v1_2d.x), f32x8::splat(v1_2d.y), f32x8::splat(v2_2d.x), f32x8::splat(v2_2d.y), simd_x, simd_y);
            
            // 8 pixels at a time
            for x in (min_x..=max_x).step_by(8) {
    
                let mask = (simd_w0.simd_ge(f32x8::splat(0.0))) &
                           (simd_w1.simd_ge(f32x8::splat(0.0))) &
                           (simd_w2.simd_ge(f32x8::splat(0.0)));
    
                // if any pixels in triangle
                if mask.any() {
                    let index_start = (y * fb.width + x) as usize;
    
                    let inv_depth_interp = (simd_w0 * simd_inv_z1 + simd_w1 * simd_inv_z2 + simd_w2 * simd_inv_z3) / simd_area;
                    let depth = simd_one / inv_depth_interp;
                    
                    // load existing depth values, handles edge of screen
                    let mut current_depth = [0.0; 8];
                    let end_index = (index_start + 8).min(fb.depth_buffer.len());
                    current_depth[..end_index - index_start].copy_from_slice(&fb.depth_buffer[index_start..end_index]);
                    let simd_current_depth = f32x8::from_slice(&current_depth);
                    
                    let new_depth_mask = mask & depth.simd_gt(simd_current_depth);
                    
                    if new_depth_mask.any() {
                        for i in 0..8 {
                            if new_depth_mask.test(i) {
                                let current_x = x + i as u32;
                                if current_x <= max_x {
                                    let index = index_start + i;
                                    fb.depth_buffer[index] = depth[i];
                                    fb.set_pixel(current_x, y, color);
                                }
                            }
                        }
                    }
                }
    
                // update barycentric coords
                simd_w0 += simd_dy0 * f32x8::splat(8.0);
                simd_w1 += simd_dy1 * f32x8::splat(8.0);
                simd_w2 += simd_dy2 * f32x8::splat(8.0);
            }
        }
    }
    

    // signed area func
    #[inline]
    fn edge(&self, a: Vec2, b: Vec2, c: Vec2) -> f32 {
        (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
    }


    #[inline]
    fn edge_simd(&self, ax: f32x8, ay: f32x8, bx: f32x8, by: f32x8, cx: f32x8, cy: f32x8) -> f32x8 {
        (cx - ax) * (by - ay) - (cy - ay) * (bx - ax)
    }
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
    /// Calculates the signed distance from a point `p` to this plane.
    fn distance(&self, p: Vec3) -> f32 {
        self.normal.dot(p) + self.d
    }
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



fn clip_polygon_against_plane(input_poly: &[Vec3], output_poly: &mut Vec<Vec3>, plane: &Plane) {
    output_poly.clear();

    if input_poly.is_empty() {
        return;
    }

    let mut prev_v = *input_poly.last().unwrap();
    let mut prev_dist = plane.distance(prev_v);

    for &curr_v in input_poly {
        let curr_dist = plane.distance(curr_v);

        let prev_inside = prev_dist >= 0.0;
        let curr_inside = curr_dist >= 0.0;

        // if edge crosses plane, add intersection point
        if prev_inside != curr_inside {
            let t = prev_dist / (prev_dist - curr_dist);
            let intersection = prev_v + (curr_v - prev_v) * t;
            output_poly.push(intersection);
        }

        // if vertex inside plane
        if curr_inside {
            output_poly.push(curr_v);
        }

        prev_v = curr_v;
        prev_dist = curr_dist;
    }
}



fn triangulate_convex_polygon(polygon: &[Vec3], triangles: &mut Vec<[Vec3; 3]>) {
    if polygon.len() < 3 {
        return;
    }
    
    let v0 = polygon[0];
    for i in 1..(polygon.len() - 1) {
        triangles.push([v0, polygon[i], polygon[i + 1]]);
    }
}


