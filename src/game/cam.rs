//! defines a camera for 3d rendering
//!
//! also kinda the player position :3
use super::vecs::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub fov: f32,
    pub yaw: f32,
    pub near: f32,
}

impl Camera {
    pub fn forward_vector(&self) -> Vec3 {
        Vec3 {
            x: self.yaw.sin(),
            y: 0.0,
            z: self.yaw.cos(),
        }
        .normalize()
    }
}
