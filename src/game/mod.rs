//Screen size is 800x600
mod vecs;
mod a3d_to_2d;
mod lines;
mod array;
mod polygons;
mod cam;
use ggez::GameError;
use ggez::timer::time_since_start;
use lines::LineSegment;
use ggez::{Context, GameResult, event, graphics, graphics::Color};
use ggez::graphics::{DrawMode, Mesh};
use nalgebra_glm::project;
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
    let base = -0.5;
    let offset_up = 1.0;
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
            (Vec2::new(1.0, 1.0), Vec2::new(1.0, -3.0)),
        ];

        let mut camera3d: vecs::Vec3 = Default::default();
        let fov: f32 = 120.0_f32.to_radians();
        let cooler_floor_plan = floor_plan.into_iter()
            .map(|v| (v.into(), random_color()))
            .collect::<Vec<_>>();

        GameState {
            // Initialize game state here
            cam: Camera {
                pos: camera3d,
                fov,
            },
            walls: cooler_floor_plan,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update game logic here
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


    let aspect = width/height;

    let (frustum_left_ray, frustum_right_ray) = game_state.cam.frustum_cam_rays(aspect); 

    let mut parsed_walls = vec![];
    for (wall_seg, color) in game_state.walls.iter() {

        let wall_segment = wall_camera_intersect((frustum_left_ray, frustum_right_ray), *wall_seg);
        println!("wall seg {:?}", wall_segment);

        let wall_point_set = wall_floor_to_3d(&wall_segment.start, &wall_segment.end);
        let mut wall: Vec<Vec2> = vec![];
        for wall_point in wall_point_set {
            match a3d_to_2d::project_point(wall_point, &game_state.cam, None){
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

fn wall_camera_intersect(fulcs: (LineSegment, LineSegment), wall_seg: LineSegment) -> LineSegment {
    let left = wall_camera_intersect_one(fulcs.0, wall_seg);
    if left != wall_seg {
        return left;
    }
    let right = wall_camera_intersect_one(fulcs.1, wall_seg);
    if right != wall_seg {
        return right;
    }
    wall_seg
}

fn wall_camera_intersect_one(fulc: LineSegment, mut wall_seg: LineSegment) -> LineSegment {
        let intersection: Vec2 = match lines::intersection_point_segment(&fulc, &wall_seg) {
            Some(mut a) => {
                a.y += 0.1;
                a
            },
            _ => {
                return wall_seg
            }
        };
        println!("start: {} end: {} inter: {}", wall_seg.start.y, wall_seg.end.y, intersection.y);
        if wall_seg.start.y < intersection.y{
            wall_seg.start = intersection;
        } else if wall_seg.end.y < intersection.y {
            wall_seg.end = intersection;
        } 

        wall_seg
}

fn random_color() -> Color{
    let mut rng = rand::rng();
    let r: u8 = rng.random();
    let g: u8 = rng.random();
    let b: u8 = rng.random();
    Color::from_rgb(r, g, b)
}