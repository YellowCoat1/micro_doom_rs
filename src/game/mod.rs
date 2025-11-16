//Screen size is 800x600
mod vecs;
mod a3d_to_2d;
mod lines;
mod array;
mod polygons;
use ggez::GameError;
use lines::LineSegment;
use ggez::{Context, GameResult, event, graphics, graphics::Color};
use ggez::graphics::{DrawMode, Mesh};
use vecs::{Vec2, Vec3};
use rand::Rng;

use polygons::Polygon;

pub struct GameState {
    camera3d: Vec3,
    walls:Vec<(Polygon, Color)>
}

fn wall_floor_to_3d(wall_left: &Vec2, wall_right: &Vec2) -> Vec<Vec3> {
    let mut three_d_point = vec![];
    let base = -5.0;
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
            (Vec2::new(10.0, 5.0), Vec2::new(10.0, 10.0)),
            (Vec2::new(10.0, 10.0), Vec2::new(0.0, 10.0)),
            (Vec2::new(0.0, 10.0), Vec2::new(-20.0, 10.0)),
        ];

        let mut camera3d: vecs::Vec3 = Default::default();
        let camera_distance: f32 = 100.0;

        let mut parsed_walls = vec![];
        let wall_points = floor_plan.iter().map(|(l, r)| wall_floor_to_3d(l, r));
        for wall_point_set in wall_points {
            let mut wall: Vec<Vec2> = vec![];
            for wall_point in wall_point_set {
                match a3d_to_2d::a3d_to_2d(wall_point, camera3d, camera_distance) {
                    Some(s) => wall.push(Vec2 {
                        x: s.x,
                        y: s.y, 
                    }),
                    None => continue,
                }
            };
            parsed_walls.push((Polygon::new(wall), random_color()));
        }
        GameState {
            // Initialize game state here
            camera3d,
            walls: parsed_walls,
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


  /*   let line_seg = LineSegment {
        start: Vec2 {
            x: 0.0,
            y: 0.0,
        },
        end: Vec2 {
            x: 0.0,
            y: 600.0,
        }
    };
        let line_seg2 = LineSegment {
        start: Vec2 {
            x: 0.0,
            y: 0.0,
        },
        end: Vec2 {
            x: 800.0,
            y: 0.0,
        }
    };*/
    let scale: Vec2 = ((800.0/2.0), (600.0/2.0)).into();
    for (wall, color) in game_state.walls.iter(){
        println!("poly_len {:?}", wall.points.len());
        for point in wall.points.iter(){
            println!("Points: {},{}", point.x, point.y);
        }
        (wall.clone()+scale).draw_filled(ctx, canvas, *color);
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