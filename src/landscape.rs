extern crate piston_window;
extern crate rand;

use self::piston_window::*;
use self::rand::Rng;
use game;


struct Mountain {
    color: (f32, f32, f32)
}

impl Mountain {
    fn new() -> Mountain {
        Mountain {
            color (1.0 as f32, 1.0 as f32, 1.0 as f32)
        }
    }
}