use super::vecs::{Vec3, Vec2};

/// Takes in a 3D point, an eye position, and a distance to the projection plane,
/// and returns a pointer to a 2D point representing the projected coordinates.
/// If the point is behind the eye, returns a null pointer.
pub fn a3d_to_2d(point: Vec3, eye: Vec3, distance: f32) -> Option<Vec2> {
    let relative = point - eye;

    if relative.z <= f32::EPSILON {
        return None;
    }


    let screen_x = (relative.x * distance) / relative.z;
    let screen_y = (relative.y * distance) / relative.z;
    Some(Vec2::new(screen_x, screen_y))
}

