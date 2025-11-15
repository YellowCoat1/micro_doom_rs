use nalgebra_glm as glm;
use nalgebra_glm::{Vec2 as GlmVec2, Vec3 as GlmVec3};

pub struct Vec2 {
    x: f32,
    y: f32,
}

pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl From<GlmVec2> for Vec2 {
    fn from(v: GlmVec2) -> Self {
        Vec2 { x: v.x, y: v.y }
    }
}
impl From<GlmVec3> for Vec3 {
    fn from(v: GlmVec3) -> Self {
        Vec3 { x: v.x, y: v.y, z: v.z }
    }
}
impl From<Vec2> for GlmVec2 {
    fn from(v: Vec2) -> Self {
        GlmVec2::new(v.x, v.y)
    }
}

impl From<Vec3> for GlmVec3 {
    fn from(v: Vec3) -> Self {
        GlmVec3::new(v.x, v.y, v.z)
    }
}
