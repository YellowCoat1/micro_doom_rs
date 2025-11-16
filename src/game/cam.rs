use super::vecs::{Vec2, Vec3};
use crate::game::array::ror_vec3_yaw;
use super::lines::{self, LineSegment};
pub struct Camera {
    pub pos: Vec3,
    pub fov: f32,
    pub yaw: f32,
}

impl Camera {
    pub fn frustum_cam_rays(&self, aspect: f32) -> (LineSegment, LineSegment) {
        let near = 1.0;

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

    pub fn rotate_seg(&self, seg: LineSegment) -> LineSegment {
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();

        let rotate_point = |point: Vec2| -> Vec2 {
            Vec2 {
                x: point.x * cos_yaw - point.y * sin_yaw,
                y: point.x * sin_yaw + point.y * cos_yaw,
            }
        };

        LineSegment {
            start: rotate_point(seg.start),
            end: rotate_point(seg.end),
        }
    }

}

pub fn wall_camera_intersect(fulcs: (LineSegment, LineSegment), wall_seg: LineSegment, _cam: &Camera) -> Vec<LineSegment> {
    let left = wall_camera_intersect_one(fulcs.0, wall_seg);
    if let Some(s) = left {
        return s.into();
    }
    let right = wall_camera_intersect_one(fulcs.1, wall_seg);
    if let Some(s) = right {
        return s.into();
    }
    vec![wall_seg]
}

fn wall_camera_intersect_one(fulc: LineSegment, wall_seg: LineSegment) -> Option<[LineSegment; 2]> {
    match lines::intersection_point_segment(&fulc, &wall_seg) {
        Some(a) => Some([LineSegment { start: wall_seg.start, end: a }, LineSegment { start: a, end: wall_seg.end }]),
        _ => None,
    }
}

pub fn rotate_seg(seg: LineSegment, cam: &Camera) -> LineSegment {
    // world → camera-relative
    let start_rel = Vec3 {
        x: seg.start.x,
        y: 0.0,
        z: seg.start.y,
    } - cam.pos;

    let end_rel = Vec3 {
        x: seg.end.x,
        y: 0.0,
        z: seg.end.y,
    } - cam.pos;

    // rotate into camera view space
    let start_view = ror_vec3_yaw(start_rel, cam.yaw);
    let end_view   = ror_vec3_yaw(end_rel, cam.yaw);

    // back to 2D ground
    let final_start = Vec2 { x: start_view.x, y: start_view.z };
    let final_end   = Vec2 { x: end_view.x,   y: end_view.z };

    (final_start, final_end).into()
}   
