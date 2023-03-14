extern crate ggez;
extern crate rand;

use ggez::event::{self, EventHandler};
use ggez::graphics::{Color, Rect};
use ggez::{conf, graphics};
use ggez::{Context, ContextBuilder, GameResult};

use rand::Rng;

const WIDTH: f32 = 620.0;
const HEIGHT: f32 = 500.0;
const STARS_COUNT: u16 = 150;
const XY_RANGE: i32 = 250;
const MAX_DEPTH: u32 = 220;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("my_game", "Starfield")
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new();

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Star {
    x: f32,
    y: f32,
    z: f32,
    color: Color,
}

impl Star {
    fn new() -> Star {
        Star {
            x: xy_range(),

            y: xy_range(),

            z: rand::thread_rng().gen_range(50, MAX_DEPTH) as f32,
            color: MyGame::get_next_color(
                //inclusive of low and exclusive of high ==> 0-255
                rand::thread_rng().gen_range(0, 256) as u8,
                rand::thread_rng().gen_range(0, 256) as u8,
                rand::thread_rng().gen_range(0, 256) as u8,
            ),
        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas) -> () {
        let half_width = WIDTH as f32 / 2.0;
        let half_height = HEIGHT as f32 / 2.0;
        let k = 128.0 / self.z;
        let px: f32 = self.x * k + half_width;
        let py: f32 = self.y * k + half_height;
        if px >= 0.0 && px <= WIDTH as f32 && py >= 0.0 && py <= HEIGHT as f32 {
            let size = 12.0;
            let color = self.color;

            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::default()
                    .dest_rect(Rect::new(px, py, size, size))
                    .color(color),
            );
        }
    }
}

fn xy_range() -> f32 {
    rand::thread_rng().gen_range(XY_RANGE * -1, XY_RANGE) as f32
}

struct MyGame {
    stars: Vec<Star>,
}

impl MyGame {
    fn new() -> Self {
        let mut stars: Vec<Star> = vec![];
        for _ in 0..STARS_COUNT {
            stars.push(Star::new());
        }
        MyGame { stars }
    }

    fn get_next_color(r: u8, g: u8, b: u8) -> Color {
        Color::from_rgba(r, g, b, 255)
    }
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(120) {
            for star in &mut self.stars {
                star.z -= 0.3;
                if star.z <= 0.0 {
                    star.x = xy_range();
                    star.y = xy_range();
                    star.z = MAX_DEPTH as f32;
                }
            }
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Color::from_rgb_u32(0x000000) clear
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb_u32(0x010400));
        for star in &mut self.stars {
            star.draw(&mut canvas);
        }
        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}
