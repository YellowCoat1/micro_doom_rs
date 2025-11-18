//! defines a camera for 3d rendering
//!
//! also kinda the player position :3
use super::vecs::Vec3;
use nalgebra_glm as glm;
use nalgebra_glm::Mat4 as GMat4;
use nalgebra_glm::Vec3 as GVec3;

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
    pub fn look_matrix(&self) -> GMat4 {
        let forward: GVec3 = self.forward_vector().into();
        let pos: GVec3 = self.pos.into();
        glm::look_at(&pos, &(pos + forward), &(glm::vec3(0.0, 1.0, 0.0)))
    }
}
