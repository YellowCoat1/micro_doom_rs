use mint::Point2;
use nalgebra_glm as glm;
use crate::{Drawer, GraphicsContext, game::cam::Camera};

pub fn draw_skybox<T: Drawer>(
    cam: &Camera,
    gctx: &mut GraphicsContext<'_, T>,
    proj: glm::Mat4,
)  {
    let width = gctx.width as f32;
    let height = gctx.height as f32;

    let middlepoint = cam.pos + cam.forward_vector_zero_pitch();
    let screen_middlepoint = super::a3d_to_2d::project_point(middlepoint.into(), &cam, proj, width, height)
        .expect("screen middlepoint was off screen, somehow.");
    let y_val = screen_middlepoint.y;

    let points = vec![
        Point2 { x: 0.0, y: 0.0},
        Point2 { x: width, y: 0.0},
        Point2 { x: width, y: y_val},
        Point2 { x: 0.0, y: y_val},
    ];

    const BLUE: (u8, u8, u8, u8) = (0, 0, 255, 255);

    gctx.drawer.draw_polygon(&points, BLUE);
}
