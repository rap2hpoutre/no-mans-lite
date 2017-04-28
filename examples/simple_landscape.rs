extern crate nml;
extern crate piston_window;
extern crate rand;


use nml::landscape::Mountain;
use nml::game::Resolution;
use self::piston_window::*;

fn main() {
    let res = Resolution {width: 640, height: 480};
    let mut m = Mountain::new(&res);

    let mut window: PistonWindow =
        WindowSettings::new("Landscape", [res.width, res.height])
        .exit_on_esc(true).build().unwrap();

    
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            m.play(c.transform, g);
        });
    }
}