use piston_window::*;
use rand::*;

const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 1000.0;

const FPS: f64 = 60.0;

const BIRD_COUNT: usize = 50;

const BIRD_SHAPE: [[f64; 2]; 4] = [
    [  0.0,  0.0],
    [-10.0, -5.0],
    [  0.0, 15.0],
    [ 10.0, -5.0]
];

const BIRD_COLOR: &str = "1e508c";
const BG_COLOR: &str = "000000";

#[derive(Debug)]
struct Bird {
    pub x: f64,
    pub y: f64,
    pub heading: f64
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            x: random::<f64>() * WIDTH,
            y: random::<f64>() * HEIGHT,
            heading: random::<f64>() * (std::f64::consts::PI * 2.0)
        }
    }
}

fn main() {
    let mut birds = Vec::new();
    for _ in 0..BIRD_COUNT {
        birds.push(Bird::new());
    }

    let mut window: PistonWindow = WindowSettings::new("Boids", [WIDTH, HEIGHT])
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut timer: f64 = 0.0;
    let mut events = window.events;
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear(color::hex(BG_COLOR), g);
                for bird in birds.iter() {
                    let cos = bird.heading.cos();
                    let sin = bird.heading.sin();
                    let mut shape: [[f64; 2]; BIRD_SHAPE.len()] = Default::default();
                    for i in 0..BIRD_SHAPE.len() {
                        shape[i][0] = BIRD_SHAPE[i][0] * cos - BIRD_SHAPE[i][1] * sin + bird.x;
                        shape[i][1] = BIRD_SHAPE[i][0] * sin + BIRD_SHAPE[i][1] * cos + bird.y;
                    }
                    polygon(color::hex(BIRD_COLOR), &shape, c.transform, g);
                }
            });
        }
        if let Some(u) = e.update_args() {
            if timer >= 1.0 / FPS{
                timer = u.dt;
                for bird in birds.iter_mut() {
                    bird.heading = (bird.heading + std::f64::consts::PI / 360.0) % (std::f64::consts::PI * 2.0);
                }
            } else {
                timer += u.dt;
            }
        }   
    }
}
