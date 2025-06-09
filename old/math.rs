// use crate::math::vectors::*;


// pub fn perpendicular(vector: Vec2) -> Vec2 {
//     return vec2!(vector.y, -vector.x)
// }

// // positive when p is on left, negative on right (relative to a -> b)
// fn cross_product_z(pa: Vec2, pb: Vec2) -> f32 {
//     pa.x * pb.y - pa.y * pb.x
// }

// pub fn signed_tri_area(a: Vec2, b: Vec2, c: Vec2) -> f32 {
//     let ac: Vec2 = a - c;
//     let ab_perp = perpendicular(b - a);
//     return dot(ac, ab_perp) / 2.0;
// }



// pub fn point_in_tri(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> bool {

//     // edges of tri
//     let ab = b - a;
//     let bc = c - b;
//     let ca = a - c;

//     // vectors from corner to point
//     let ap = p - a;
//     let bp = p - b;
//     let cp = p - c;

    
//     let cross1 = cross_product_z(ab, ap);
//     let cross2 = cross_product_z(bc, bp);
//     let cross3 = cross_product_z(ca, cp);

//     // if all cross products have same sign (or zero)
//     // point is inside or on edge
//     // handles both CCW and CW winding orders 
//     let all_left = cross1 >= 0.0 && cross2 >= 0.0 && cross3 >= 0.0;
//     let all_right = cross1 <= 0.0 && cross2 <= 0.0 && cross3 <= 0.0;
//     return all_left || all_right;
// }




