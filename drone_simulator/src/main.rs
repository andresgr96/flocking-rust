use ggez::{conf, event, graphics, timer, Context, GameResult, mint};
use rand::Rng;
use nalgebra as na;

const GRID_SIZE: f32 = 20.0;
const DRONE_RADIUS: f32 = 10.0;
const NUM_DRONES: usize = 10;

#[derive(Debug, Copy, Clone)]
struct Drone {
    position: mint::Point2<f32>,
    velocity: mint::Vector2<f32>,
}

impl Drone {
    fn new(x: f32, y: f32) -> Self {
        Drone {
            position: mint::Point2 { x, y },
            velocity: mint::Vector2{x: 0.0, y: 0.0},
        }
    }

    fn update(&mut self, ctx: &Context) {
        let mut rng = rand::thread_rng();

        let delta_time = timer::delta(ctx).as_secs_f32();
        let displacement = na::Vector2::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ) * delta_time;

        self.position = mint::Point2 {
            x: self.position.x + displacement.x,
            y: self.position.y + displacement.y,
        };
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position,
            DRONE_RADIUS,
            0.1,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, (mint::Point2{x: 0.0, y: 0.0},))
    }
}

fn main() {
    println!("Hello, world!");
}
