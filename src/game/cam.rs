//! defines a camera for 3d rendering
//!
//! also kinda the player position :3
use super::vecs::{Vec2, Vec3};
use crate::game::{array::ror_vec3_yaw, lines::LineSegment3};
use super::lines::{self, LineSegment};

pub struct Camera {
    pub pos: Vec3,
    pub fov: f32,
    pub yaw: f32,
    pub near: f32,
}

impl Camera {
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

    pub fn clip_to_near_plane(&self, start: Vec3, end: Vec3) -> Option<(Vec3, Vec3)> {
        let mut p1 = start - self.pos;
        let mut p2 = end - self.pos;
        let near = self.near;
        let z1 = p1.z;
        let z2 = p2.z;

        if z1 < near && z2 < near {
            return None; // Both behind
        }

        if z1 < near {
            let t = (near - z1) / (z2 - z1);
            p1.x += t * (p2.x - p1.x);
            p1.y += t * (p2.y - p1.y);
            p1.z = near;
        } else if z2 < near {
            let t = (near - z2) / (z1 - z2);
            p2.x += t * (p1.x - p2.x);
            p2.y += t * (p1.y - p2.y);
            p2.z = near;
        }

        Some((p1+self.pos, p2+self.pos))
    }

    pub fn forward_vector(&self) -> Vec3 {
        Vec3 {
            x: self.yaw.sin(),
            y: 0.0,
            z: self.yaw.cos(),
        }.normalize()
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
