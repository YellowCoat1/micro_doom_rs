//Screen size is 800x600
mod vecs;
mod a3d_to_2d;
mod lines;
mod array;
mod polygons;
mod cam;
mod skybox;
use lines::LineSegment;
use ggez::{Context, GameResult, event, graphics, graphics::Color};
use ggez::input::keyboard::KeyCode;
use vecs::{Vec2, Vec3};
use rand::Rng;
use cam::Camera;

use polygons::Polygon;




pub struct GameState {
    cam: Camera,
    walls:Vec<(LineSegment, Color)>
}

fn wall_floor_to_3d(wall_left: &Vec2, wall_right: &Vec2) -> Vec<Vec3> {
    let mut three_d_point = vec![];
    let base = -3.0;
    let offset_up = 10.0;
    three_d_point.push(Vec3 {
        x: wall_left.x,
        y: base,
        z: wall_left.y
    });
    three_d_point.push(Vec3 {
        x: wall_left.x,
        y: base+offset_up,
        z: wall_left.y
    });
    three_d_point.push(Vec3 {
        x: wall_right.x,
        y: base+offset_up,
        z: wall_right.y
    });
    three_d_point.push(Vec3 {
        x: wall_right.x,
        y: base,
        z: wall_right.y
    });
    three_d_point
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        //let mut apoint3d: vecs::Vec3 = (10.0, 10.0, 10.0).into();
        //let mut another_point3d: vecs::Vec3 = (10.0, 20.0, 10.0).into();
        let floor_plan: Vec<(Vec2, Vec2)> = vec![
            (Vec2::new(2.0, 1.0), Vec2::new(2.0, 20.0)),
            (Vec2::new(2.0, 20.0), Vec2::new(30.0, 30.2)),
            (Vec2::new(30.0, 30.2), Vec2::new(30.0, 1.0))
        ];

        let camera3d: vecs::Vec3 = Default::default();
        let fov: f32 = 75.0_f32.to_radians();
        let cooler_floor_plan = floor_plan.into_iter()
            .map(|v| (v.into(), random_color()))
            .collect::<Vec<_>>();

        GameState {
            // Initialize game state here
            cam: Camera {
                pos: camera3d,
                fov,
                yaw: 0.0,
            },
            walls: cooler_floor_plan,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.cam.pos.z += 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.cam.pos.z -= 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.cam.pos.x -= 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.cam.pos.x += 0.01;
        }
        if (ctx.keyboard.is_key_pressed(KeyCode::J)) {
            self.cam.yaw -= 0.01;
        }
        if (ctx.keyboard.is_key_pressed(KeyCode::K)) {
            self.cam.yaw += 0.01;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        draw_screen(self, ctx, &mut canvas)?;

        canvas.finish(ctx)
    }
}

fn draw_screen(game_state: &mut GameState, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult<()> {
    // Drawing logic here
    let (width, height) = ctx.gfx.size();

    skybox::draw_skybox(game_state, canvas, width as f32, height as f32)?;


    let aspect = width/height;

    let (frustum_left_ray, frustum_right_ray) = game_state.cam.frustum_cam_rays(aspect); 

    let mut wall_segs = Vec::with_capacity(game_state.walls.len());
    for (wall_seg, color) in game_state.walls.iter() {
        let rotated_wall_seg = cam::rotate_seg(*wall_seg, &game_state.cam);
    
        let wall_segment = cam::wall_camera_intersect((frustum_left_ray, frustum_right_ray), rotated_wall_seg, &game_state.cam)
            .into_iter()
            .map(|v| (v, color))
            .collect::<Vec<_>>();
        wall_segs.push(wall_segment);
    }

    let mut parsed_walls: Vec<(Polygon, &Color)> = vec![];
    for (wall_segment, color) in wall_segs.into_iter().flat_map(|v| v) {

        // wall_seg: LineSegment

        let wall_point_set = wall_floor_to_3d(&wall_segment.start, &wall_segment.end);
        let mut wall: Vec<Vec2> = vec![];
        for wall_point in wall_point_set {
            match a3d_to_2d::project_point(wall_point, &game_state.cam, width/height){
                Some(s) => wall.push(Vec2 {
                    x: s.x,
                    y: s.y, 
                }*500.0),
                None => continue,
            }
        };
        parsed_walls.push((Polygon::new(wall), color));
    }

    let scale: Vec2 = ((width/2.0), (height/2.0)).into();
    for (wall, color) in parsed_walls.iter(){
        for point in wall.points.iter(){
            println!("Points: {},{}", point.x, point.y);
        }
        (wall.clone()+scale).draw_filled(ctx, canvas, **color);
    }
    //line_seg.draw(ctx, canvas, Color::BLACK);


    Ok(())
}
fn random_color() -> Color{
    let mut rng = rand::rng();
    let r: u8 = rng.random();
    let g: u8 = rng.random();
    let b: u8 = rng.random();
    Color::from_rgb(r, g, b)
}

