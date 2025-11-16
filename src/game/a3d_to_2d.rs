use super::vecs::{Vec3, Vec2};

/// Projects a 3D point `p` into 2D space given a camera position and a vertical FOV (in radians).
///
/// - `cam_pos` is the camera’s position in world space.  
/// - `fov_y` is the vertical field of view in **radians**.  
/// - `screen_size` is (width, height) of your target 2D plane (in pixels); optional.
///
/// Returns a 2D point in pixel coordinates if `screen_size` is provided,
/// otherwise returns normalized device coordinates in [-1, 1].
fn project_point(point: Vec3, cam_pos: Vec3, fov_y: f32, screen_size: Option<(f32, f32)>) -> Option<Vec2> {
    // Translate point relative to camera
    let dx = point.x - cam_pos.x;
    let dy = point.y - cam_pos.y;
    let dz = point.z - cam_pos.z;

    // If point is behind the camera (or exactly at camera), no projection
    if dz <= 0.0 {
        return None;
    }

    // Compute focal length from FOV: f = 1 / tan(FOV/2)
    let f = 1.0 / (fov_y * 0.5).tan();

    // Project to NDC (normalized device coordinates)
    let ndc_x = dx * f / dz;
    let ndc_y = dy * f / dz;

    if let Some((w, h)) = screen_size {
        // Map NDC (-1..1) to pixel coordinates
        let px = (ndc_x + 1.0) * 0.5 * w;
        // Flip Y-axis if your screen origin is top-left
        let py = (1.0 - (ndc_y + 1.0) * 0.5) * h;
        Some(Vec2 { x: px, y: py })
    } else {
        Some(Vec2 { x: ndc_x, y: ndc_y })
    }
}
