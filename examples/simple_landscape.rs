extern crate nml;
extern crate piston_window;
extern crate rand;


use nml::game::Resolution;
use self::piston_window::*;
use self::rand::Rng;

fn main() {
    let res = Resolution {width: 640, height: 480};

    let mut window: PistonWindow =
        WindowSettings::new("Landscape", [res.width, res.height])
        .exit_on_esc(true).build().unwrap();

    let step_max = 2.5;
    let step_change = 1.0;
    let height_max = res.height;

    let mut height =  rand::thread_rng().gen_range(0.0, height_max as f64);
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
    
        if height > height_max as f64 { 
            height = height_max as f64;
            slope = slope * -1.0;
        }
        if height < 0.0 { 
            height = 0.0;
            slope = slope * -1.0;
        }
        points.push(height);
    }

    // println!("{:?}", points);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let mut i = 0.0;
            for &point in points.iter() {
                line([1.0, 1.0, 1.0, 0.2], 1.0, [i, point, i, height_max as f64], c.transform, g);
                i = i + 1.0;
            }
        });
    }
}