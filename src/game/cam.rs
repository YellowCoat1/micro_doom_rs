use super::vecs::{Vec2, Vec3};
use super::lines::LineSegment;
pub struct Camera {
    pub pos: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn frustum_cam_rays(&self, aspect: f32) -> (LineSegment, LineSegment) {
        let near = 0.1;

        let forward = Vec3::new(0.0, 0.0, 1.0);
        let right = Vec3::new(1.0, 0.0, 0.0);
        // Compute near plane half extents
        let h_half = (self.fov * 0.5).tan() * near;
        let w_half = h_half * aspect;
    
        // Camera-space points on the near-plane edges
        let left_cam = Vec3::new(-w_half, 0.0, near);
        let right_cam = Vec3::new(w_half, 0.0, near);


        // Convert to world space
        let left_world = self.pos + right * left_cam.x + forward * left_cam.z;
        let right_world = self.pos + right * right_cam.x + forward * right_cam.z;

        // Compute 3D ray directions
        let left_dir3 = left_world - self.pos;
        let right_dir3 = right_world - self.pos;

        // Project to 2D (top-down) by dropping the y component
        let left_dir2 = Vec3::new(left_dir3.x, 0.0, left_dir3.z);
        let right_dir2 = Vec3::new(right_dir3.x, 0.0, right_dir3.z);

        // Ray origins in 2D: just camera's ground position (x, z)
        let origin2: Vec2 = (self.pos.x, self.pos.z).into();

        let end1: Vec2 = (left_dir2.x, left_dir2.z).into();
        let end2: Vec2 = (right_dir2.x, right_dir2.z).into();

        (
            LineSegment { start: origin2, end: end1 + origin2},
            LineSegment { start: origin2, end: end2 + origin2 },
        )
    }
}
