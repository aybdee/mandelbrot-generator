mod utils;
use wasm_bindgen::prelude::*;

// View area size --> constant visible part of mandelbrot 500x500
// pixels --> the correct colour of all the pixels in the current view area(rgb)

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
struct MandelBrot {
    // viewarea: [u16; 4],
    // pixels: Vec<u8>,
    // width: u16,
    // height: u16,
}

#[wasm_bindgen]
impl MandelBrot {
    pub fn get_pixels(width: u16, height: u16, max_iterations: u8) -> Vec<u8> {
        let mut pixels: Vec<u8> = vec![];
        for x in 0..(width) {
            for y in 0..(height) {
                let x_0 = MandelBrot::scale_x_value(width as f32, x);
                let y_0 = MandelBrot::scale_y_value(height as f32, y);
                let is_divergent = !(MandelBrot::is_divergent(x_0, y_0, max_iterations));
                if (is_divergent) {
                    pixels.push(0);
                } else {
                    pixels.push(1);
                }
            }
        }
        return pixels;
    }
    fn is_divergent(x: f32, y: f32, max_iterations: u8) -> bool {
        let mut x_curr = x;
        let mut y_curr = y;
        let mut num_iterations = 0;
        while (x_curr.powf(2.0) + y_curr.powf(2.0) <= 4.0) && (num_iterations < max_iterations) {
            let x_temp = (x_curr.powf(2.0) - y_curr.powf(2.0)) + x;
            y_curr = (2.0 * x_curr * y_curr) + y;
            x_curr = x_temp;
            num_iterations += 1;
        }
        if num_iterations == max_iterations {
            return false;
        } else {
            return true;
        }
    }

    fn scale_x_value(range: f32, value: u16) -> f32 {
        const MAX_X: f32 = 0.47;
        const MIN_X: f32 = -2.00;
        let div = (MAX_X - MIN_X) / range;
        return MIN_X + (div * (value as f32));
    }

    fn scale_y_value(range: f32, value: u16) -> f32 {
        const MAX_X: f32 = 1.12;
        const MIN_X: f32 = -1.12;
        let div = (MAX_X - MIN_X) / range;
        return MIN_X + (div * (value as f32));
    }
}
