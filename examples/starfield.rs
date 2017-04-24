extern crate nml;
extern crate piston_window;

use nml::starfield::Starfield;
use nml::game::Resolution;
use self::piston_window::*;

fn main() {
    let res = Resolution {width: 320, height: 200};
    let mut s = Starfield::new(&res);

    let mut window: PistonWindow =
        WindowSettings::new("Star example", [res.width, res.height])
        .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            s.play(c.transform, g);
        });
    }
}