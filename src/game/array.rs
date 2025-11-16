use super::vecs::Vec2;
use nalgebra_glm as glm;
use glm::DVec2;

pub struct Matrix2 {
    data: [[f64; 2]; 2],
}

pub fn ror_vec2(vec: Vec2, r: f32) -> Vec2 {
    let x = vec.x * r.cos() - vec.y * r.sin();
    let y = vec.x * r.sin() + vec.y * r.cos();
    (x,y).into()
}

pub fn ror_vec3_yaw(vec: Vec3, r: f32) -> Vec3 {
    let x = vec.x * r.cos() - vec.z * r.sin();
    let z = vec.x * r.sin() + vec.z * r.cos();
    (x, vec.y, z).into()
}
