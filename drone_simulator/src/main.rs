use ggez::{conf, event, graphics, timer, Context, GameResult, mint};
use rand::Rng;
use nalgebra as na;

const GRID_SIZE: f32 = 500.0;
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

    fn update(&mut self, ctx: &Context) -> GameResult<()>{
        let mut rng = rand::thread_rng();

        let delta_time = timer::delta(ctx).as_secs_f32();
        let displacement = na::Vector2::new(
            rng.gen_range(-5.0..10.0),
            rng.gen_range(-5.0..10.0),
        ) * delta_time;

        self.position = mint::Point2 {
            x: self.position.x + displacement.x,
            y: self.position.y + displacement.y,
        };

        Ok(())
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

struct MainState {
    drones: Vec<Drone>,
}

impl MainState {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let drones: Vec<Drone> = (0..NUM_DRONES)
            .map(|_| Drone::new(rng.gen_range(0.0..GRID_SIZE), rng.gen_range(0.0..GRID_SIZE)))
            .collect();
        MainState{drones}
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for drone in &mut self.drones {
            drone.update(ctx)?
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::BLACK);

        for drone in &self.drones {
            drone.draw(ctx)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}
fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("drone_simulator", "your_username")
        .window_setup(conf::WindowSetup::default().title("Drone Simulator"))
        .window_mode(conf::WindowMode::default().dimensions(GRID_SIZE, GRID_SIZE).resizable(true));

    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
