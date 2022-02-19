use piston_window::*;

const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 1000.0;

const FPS: f64 = 5.0;

const BIRD_COLOR: &str = "1e508c";
const BG_COLOR: &str = "000000";

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Boids", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    let mut timer: f64 = 0.0;
    let mut events = window.events;
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear(color::hex(BG_COLOR), g);
                let bird = &[
                    [20.0, 20.0],
                    [30.0, 40.0],
                    [20.0, 35.0],
                    [10.0, 40.0]];
                polygon(color::hex(BIRD_COLOR), bird, c.transform, g);
            });
        }
        if let Some(u) = e.update_args() {
            if timer >= 1.0 / FPS{
                timer = u.dt;
            } else {
                timer += u.dt;
            }
        }   
    }
}
