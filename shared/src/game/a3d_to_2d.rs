//! RANDOM FUCKY STUFF THATS THE CORE ABOUT DRAWING
//! HOW DOES IT WORK??? FUCK IF I KNOW
//! CLIP SPACE OR SMTH IG

use super::vecs::Vec2;
use nalgebra_glm as glm;

/// Represents a vertex in clip space
#[derive(Clone, Debug)]
struct Vertex {
    pos: glm::Vec4, // clip-space position
}

/// Clip polygon against a single frustum plane
fn clip_against_plane(
    vertices: &[Vertex],
    component: usize,
    compare: f32,
    less_than: bool,
) -> Vec<Vertex> {
    let mut output = Vec::new();

    for i in 0..vertices.len() {
        let current = &vertices[i];
        let prev = &vertices[(i + vertices.len() - 1) % vertices.len()];

        let current_inside = if less_than {
            current.pos[component] <= compare * current.pos.w
        } else {
            current.pos[component] >= compare * current.pos.w
        };

        let prev_inside = if less_than {
            prev.pos[component] <= compare * prev.pos.w
        } else {
            prev.pos[component] >= compare * prev.pos.w
        };

        if current_inside {
            if !prev_inside {
                // Intersection
                let t = (compare * prev.pos.w - prev.pos[component])
                    / ((current.pos[component] - prev.pos[component])
                        - compare * (current.pos.w - prev.pos.w));
                let intersection = Vertex {
                    pos: prev.pos + (current.pos - prev.pos) * t,
                };
                output.push(intersection);
            }
            output.push(current.clone());
        } else if prev_inside {
            let t = (compare * prev.pos.w - prev.pos[component])
                / ((current.pos[component] - prev.pos[component])
                    - compare * (current.pos.w - prev.pos.w));
            let intersection = Vertex {
                pos: prev.pos + (current.pos - prev.pos) * t,
            };
            output.push(intersection);
        }
    }

    output
}

/// Clip polygon against all six frustum planes
fn clip_polygon(vertices: &[Vertex]) -> Vec<Vertex> {
    let mut clipped = vertices.to_vec();

    let planes = [
        (0, -1.0, false), // x >= -w
        (0, 1.0, true),   // x <= w
        (1, -1.0, false), // y >= -w
        (1, 1.0, true),   // y <= w
        (2, 0.0, false),  // z >= 0
        (2, 1.0, true),   // z <= w
    ];

    for &(component, compare, less_than) in &planes {
        clipped = clip_against_plane(&clipped, component, compare, less_than);
        if clipped.is_empty() {
            break;
        }
    }

    clipped
}

/// Project a point without clipping
pub fn project_point(point: glm::Vec3, cam: &super::cam::Camera, proj: glm::Mat4, screen_width: f32, screen_height: f32) -> Option<Vec2> {
    let view = cam.look_matrix();
    let clip = proj * view * glm::vec4(point.x, point.y, point.z, 1.0);
    if clip.z < 0.0 {
        return None
    }

    let ndc_x = clip.x / clip.w;
    let ndc_y = clip.y / clip.w;
    Some(ndc_to_screen(ndc_x, ndc_y, screen_width, screen_height))
}

/// Clip and project polygon to screen space
///
/// Essentially takes a 3d polygon n gives u the 2d coords for drawing
///
/// the CORE of the core
pub fn clip_and_project_polygon(
    world_vertices: &[glm::Vec3],
    cam: &super::cam::Camera,
    proj: glm::Mat4,
    screen_width: f32,
    screen_height: f32,
) -> Vec<Vec2> {
    
    // Transform to clip space
    let mut clip_vertices: Vec<Vertex> = world_vertices
        .iter()
        .map(|p| {
            let view = cam.look_matrix();
            let clip = proj * view * glm::vec4(p.x, p.y, p.z, 1.0);
            Vertex { pos: clip }
        })
        .collect();
    
    // if all are behind the camera, return empty
    if clip_vertices.iter().all(|v| v.pos.z < 0.0) {
        return Vec::new();
    }

    // Clip polygon
    clip_vertices = clip_polygon(&clip_vertices);

    // Perspective divide and map to screen
    clip_vertices
        .iter()
        .map(|v| {
            let ndc_x = v.pos.x / v.pos.w;
            let ndc_y = v.pos.y / v.pos.w;
            ndc_to_screen(ndc_x, ndc_y, screen_width, screen_height)
        })
        .collect()
}

/// Convert NDC to screen space
/// i actually understand this one! :D
pub fn ndc_to_screen(ndc_x: f32, ndc_y: f32, width: f32, height: f32) -> Vec2 {
    Vec2 {
        x: (-ndc_x + 1.0) * 0.5 * width,
        y: (ndc_y + 1.0) * 0.5 * height,
    }
}
