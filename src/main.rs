extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;
const STARS_COUNT: u16 = 200;
const XY_RANGE: i32 = 25;
const MAX_DEPTH: u32 = 32;

struct Star {
    x: f32,
    y: f32,
    z: f32,
}

impl Star {
    fn new() -> Star {
        Star {
            x: rand::thread_rng().gen_range(XY_RANGE * -1, XY_RANGE) as f32,
            y: rand::thread_rng().gen_range(XY_RANGE * -1, XY_RANGE) as f32,
            z: rand::thread_rng().gen_range(1, MAX_DEPTH) as f32,
        }
    }
}

fn main() {
    let half_width = WIDTH / 2;
    let half_height = HEIGHT / 2;
    let mut stars: Vec<Star> = vec![];

    for i in 0..STARS_COUNT {
        stars.push(Star::new());
        println!("{} {} {}", stars[i as usize].x, stars[i as usize].y, stars[i as usize].z);
    }

    let mut window: PistonWindow =
        WindowSettings::new("Hello star.", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([0.0, 0.0, 0.0, 1.0],
                      [0.0, 0.0, WIDTH as f64, HEIGHT as f64],
                      c.transform, g);
            for star in &mut stars {
                star.z -= 0.2;
            }
        });
    }
}