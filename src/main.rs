extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::clear;
use ggez::graphics::{present, Color, Rect};
use ggez::timer::check_update_time;
use ggez::{Context, ContextBuilder, GameResult};

use rand::Rng;

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 200.0;
const STARS_COUNT: u16 = 200;
const XY_RANGE: i32 = 25;
const MAX_DEPTH: u32 = 32;

fn main() {
    let ctx = &mut ContextBuilder::new("my_game", "Starfield")
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    let mut my_game = MyGame::new(&mut ctx.0);

    // Run!
    match event::run(&mut ctx.0, &mut ctx.1, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Star {
    x: f32,
    y: f32,
    z: f32,
}

impl Star {
    fn new() -> Star {
        Star {
            x: xy_range(),
            y: xy_range(),
            z: rand::thread_rng().gen_range(1, MAX_DEPTH) as f32,
        }
    }
}

fn xy_range() -> f32 {
    rand::thread_rng().gen_range(XY_RANGE * -1, XY_RANGE) as f32
}

struct MyGame {
    half_width: f32,
    half_height: f32,
    stars: Vec<Star>,
}

impl MyGame {
    fn new(_ctx: &mut ggez::Context) -> MyGame {
        let mut stars: Vec<Star> = vec![];
        for _ in 0..STARS_COUNT {
            stars.push(Star::new());
        }
        MyGame {
            half_width: WIDTH as f32 / 2.0,
            half_height: HEIGHT as f32 / 2.0,
            stars: stars,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while check_update_time(ctx, 30) {
            for star in &mut self.stars {
                star.z -= 0.2;
                if star.z <= 0.0 {
                    star.x = xy_range();
                    star.y = xy_range();
                    star.z = MAX_DEPTH as f32;
                }
            }
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb_u32(0x000000));
        for star in &mut self.stars {
            let k = 128.0 / star.z;
            let px: f32 = star.x * k + self.half_width;
            let py: f32 = star.y * k + self.half_height;
            if px >= 0.0 && px <= WIDTH as f32 && py >= 0.0 && py <= HEIGHT as f32 {
                let size = (1.0 - star.z / 32.0) * 5.0;
                let color = [0.0, 0.0, 1.0, 1.0].into();
                let rectangle = ggez::graphics::Mesh::new_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::fill(),
                    Rect::new(px, py, size, size),
                    color,
                )?;
                ggez::graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
                    .ok();
            }
        }
        present(ctx).ok();
        Ok(())
    }
}
