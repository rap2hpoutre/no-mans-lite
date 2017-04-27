extern crate noise;

use self::noise::Fbm;
use self::noise::NoiseModule;
use planet::noise::MultiFractal;

pub struct Planet {
    radius: usize,
    tiles: Vec<usize>,
}


impl Planet {
    pub fn new() -> Planet {
        let radius = 64usize;
        let diameter = radius * 2;

        let mut tiles: Vec<usize> = Vec::new();
        tiles.resize(diameter.pow(3), 0);

        let fbm = Fbm::new(); // .set_frequency((radius / 2) as f32)

        let center = (radius as f32, radius as f32, radius as f32);

        for x in 0..diameter {
            for y in 0..diameter {
                for z in 0..diameter {
                    let position = (x as f32, y as f32, z as f32);
                    let xs = (center.0 - position.0).abs();
                    let ys = (center.1 - position.1).abs();
                    let zs = (center.2 - position.2).abs();

                    let length = (xs * xs + ys * ys + zs * zs).sqrt();
                    let solid = length < (radius as f32);

                    if solid {
                        let idx = (x + (y * diameter) + (z * diameter * diameter)) as usize;
                        // Adjust noise from -1.0..1.0 to 0.0..1.0
                        let val = (fbm.get([x as f32, y as f32, z as f32]) + 1.0) / 2.0;
                        println!("{}", val);
                        tiles[idx] = (val * 10f32) as usize;

                    }
                }
            }
        }

        Planet {
            radius: radius,
            tiles: tiles,
        }

    }
    pub fn tiles(&self) -> &Vec<usize> {
        &self.tiles
    }
}