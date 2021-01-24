#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

#[derive(Copy, Clone)]
struct Complex {
    x: f64,
    y: f64,
}

impl Complex {
    fn new(x: f64, y: f64) -> Complex {
        Complex{x, y}
    }

    fn add(&self, b: &Complex) -> Complex {
        Complex{x: self.x+b.x, y: self.y+b.y}
    }

    fn square(&self) -> Complex {
        Complex{x: self.x*self.x-self.y*self.y, y: 2.0*self.x*self.y}
    }

    fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}

fn color(iterations: u32, max_iterations: u32) -> Pixel {
    let color = 255 - (255.0 * (iterations as f64) / (max_iterations as f64)) as u8;
    px!(color, color, color)
}

fn main() {
    let min_x = -2.0;
    let max_x = 1.0;
    let min_y = -1.0;
    let max_y = 1.0;
    let sample_x = 3000;
    let sample_y = 2000;
    let max_iterations = 1000;

    let mut img = Image::new(sample_x, sample_y);

    for i in 0..sample_x {
        let c_x = min_x + (max_x - min_x) * ((i as f64) / ((sample_x - 1) as f64));

        for j in 0..sample_y {
            let c_y = min_y + (max_y - min_y) * ((j as f64) / ((sample_y - 1) as f64));
            let c = Complex::new(c_x, c_y);

            let mut z = c.clone();
            let mut iterations = 0;
            while z.norm_squared() <= 4.0 && iterations < max_iterations {
                z = z.square().add(&c);
                iterations += 1;
            }

            let color = color(iterations, max_iterations);
            img.set_pixel(i, j, color);
        }
    }

    let _ = img.save("img.bmp");
}
