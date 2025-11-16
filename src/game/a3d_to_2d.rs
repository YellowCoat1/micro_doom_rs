use super::vecs::{Vec3, Vec2};
use super::cam::Camera;

/// Projects a 3D point `p` into 2D space given a camera position and a vertical FOV (in radians).
///
/// - `cam_pos` is the camera’s position in world space.  
/// - `fov_y` is the vertical field of view in **radians**.  
/// - `screen_size` is (width, height) of your target 2D plane (in pixels); optional.
///
/// Returns a 2D point in pixel coordinates if `screen_size` is provided,
/// otherwise returns normalized device coordinates in [-1, 1].
const SCALE: f32 = 0.5;
pub fn project_point(point: Vec3, cam: &Camera, aspect_ratio: f32) -> Option<Vec2> {
    // Translate point relative to camera
    let dx = point.x - cam.pos.x;
    let dy = point.y - cam.pos.y;
    let dz = point.z - cam.pos.z;

    // If point is behind the camera (or exactly at camera), no projection
    if dz <= 0.0 {
        return None;
    }

    // Compute focal length from FOV: f = 1 / tan(FOV/2)
    let f = 1.0 / (cam.fov * 0.5).tan();

    let f_y = 1.0 / (cam.fov * 0.5).tan();
    let f_x = f_y * aspect_ratio;

    // Project to NDC (normalized device coordinates)
    let ndc_x = dx * f_x * SCALE / dz;
    let ndc_y = dy * f_y * SCALE / dz;

    Some(Vec2 { x: ndc_x, y: ndc_y })
}
