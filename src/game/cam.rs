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
    pub pitch: f32,
    pub near: f32,
}

impl Camera {
    pub fn forward_vector(&self) -> Vec3 {
        let pitch_cos = self.pitch.cos();
        let pitch_sin = self.pitch.sin();
        let yaw_cos = self.yaw.cos();
        let yaw_sin = self.yaw.sin();
        Vec3 {
            x: yaw_sin * pitch_cos,
            y: pitch_sin,
            z: yaw_cos * pitch_cos,
        }
    }
    pub fn forward_vector_zero_pitch(&self) -> Vec3 {
        let yaw_cos = self.yaw.cos();
        let yaw_sin = self.yaw.sin();
        Vec3 {
            x: yaw_sin,
            y: 0.0,
            z: yaw_cos,
        }
    }
    pub fn look_matrix(&self) -> GMat4 {
        let forward: GVec3 = self.forward_vector().into();
        let pos: GVec3 = self.pos.into();
        glm::look_at(&pos, &(pos + forward), &(glm::vec3(0.0, 1.0, 0.0)))
    }
}
