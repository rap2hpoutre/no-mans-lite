extern crate piston_window;
extern crate rand;

use self::piston_window::*;
use self::rand::Rng;
use game;


pub struct Mountain {
    color: (f32, f32, f32),
    points: Vec<f64>,
    height_max: f64
}

impl Mountain {
    pub fn new(res: &game::Resolution) -> Mountain {
        let step_max = 2.5;
        let step_change = 1.0;
        let height_max = res.height as f64;

        let mut height =  rand::thread_rng().gen_range(0.0, height_max);
        let mut slope = rand::thread_rng().gen_range(0.0, step_max) * 2.0 - step_max;

        let mut points: Vec<f64> = Vec::new();

        for _ in 0..res.width {
            height = height + slope;
            slope = slope + (rand::thread_rng().gen_range(0.0, step_change) as f64 * 2.0 - step_change);

            if slope > step_max { 
                slope = step_max;
            }
            if slope < -step_max {
                slope = -step_max;
            } 
        
            if height > height_max { 
                height = height_max;
                slope = slope * -1.0;
            }
            if height < 0.0 { 
                height = 0.0;
                slope = slope * -1.0;
            }
            points.push(height);
        }
        Mountain {
            color: (1.0 as f32, 1.0 as f32, 1.0 as f32),
            points: points,
            height_max: height_max
        }
    }

    pub fn play(&mut self, t: types::Matrix2d, g: &mut G2d) { 
        let mut i = 0.0;
            for &point in self.points.iter() {
                line([1.0, 1.0, 1.0, 0.2], 1.0, [i, point, i, self.height_max as f64], t, g);
                i = i + 1.0;
            }
    }
}