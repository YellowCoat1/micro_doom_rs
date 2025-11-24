use nalgebra_glm as glm;
use crate::game::{cam::Camera, vecs::Vec2};

pub fn draw_skybox(
    cam: &Camera,
    drawer: &mut impl super::drawing::Drawer,
    proj: glm::Mat4,
)  {

    let width = drawer.screen_width();
    let height = drawer.screen_height();

    let middlepoint = cam.pos + cam.forward_vector_zero_pitch();
    let screen_middlepoint = super::a3d_to_2d::project_point(middlepoint.into(), &cam, proj, width, height)
        .expect("screen middlepoint was off screen, somehow.");
    let y_val = screen_middlepoint.y;

    let points = vec![
        Vec2 { x: 0.0, y: 0.0},
        Vec2 { x: width, y: 0.0},
        Vec2 { x: width, y: y_val},
        Vec2 { x: 0.0, y: y_val},
    ];

    const BLUE: (u8, u8, u8, u8) = (0, 0, 255, 255);

    drawer.draw_polygon(&points, BLUE);
}
