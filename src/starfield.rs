extern crate piston_window;
extern crate rand;

use self::piston_window::*;
use self::rand::Rng;
use game;

const STARS_COUNT: u16 = 200;
const XY_RANGE: i32 = 25;
const MAX_DEPTH: u32 = 32;

struct Star {
    x: f64,
    y: f64,
    z: f64,
}

impl Star {
    fn new() -> Star {
        Star {
            x: xy_range(),
            y: xy_range(),
            z: rand::thread_rng().gen_range(1, MAX_DEPTH) as f64,
        }
    }
}

fn xy_range() -> f64 {
    rand::thread_rng().gen_range(XY_RANGE * -1, XY_RANGE) as f64
}


pub struct Starfield {
    width: u32,
    height: u32,
    hwidth: u32,
    hheight: u32,
    stars: Vec<Star>
}

impl Starfield {
    pub fn new(res: &game::Resolution) -> Starfield {
        let mut stars: Vec<Star> = vec![];
        for _ in 0..STARS_COUNT {
            stars.push(Star::new());
        }
        Starfield {
            width: res.width,
            hwidth: res.width/2,
            height: res.height,
            hheight: res.height/2,
            stars: stars
        }
    }

    pub fn play(&mut self, t: types::Matrix2d, g: &mut G2d) {
        for star in &mut self.stars {
            star.z -= 0.4;
            if star.z <= 0.0 {
                star.x = xy_range();
                star.y = xy_range();
                star.z = MAX_DEPTH as f64;
            }
            // Perspective projection of stars
            let k  = 128.0 / star.z;
            let px: f64 = star.x * k + self.hwidth as f64;
            let py: f64 = star.y * k + self.hheight as f64;

            if px >= 0.0 && px <= self.width as f64 && py >= 0.0 && py <= self.height as f64  {
                let size = (1.0 - star.z / 32.0) * 5.0;
                let shade = (1.0 - star.z / 64.0) as f32;
                rectangle([shade, shade, shade, 1.0],
                    [px , py, size, size],
                    t, g);
            }
        }
    }
}