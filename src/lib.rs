mod utils;
use wasm_bindgen::prelude::*;

extern crate png_encode_mini;

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
    pub fn get_pixels_binary(
        width: u16,
        height: u16,
        max_iterations: u32,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
    ) -> Vec<u8> {
        let mut pixels: Vec<u8> = vec![];
        for x in 0..(width) {
            for y in 0..(height) {
                let x_0 = MandelBrot::scale_x_value(width as f64, x, x_min, x_max);
                let y_0 = MandelBrot::scale_y_value(height as f64, y, y_min, y_max);
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
    pub fn get_pixels(
        width: u16,
        height: u16,
        max_iterations: u32,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
    ) -> Vec<u8> {
        let mut pixels: Vec<u8> = vec![];
        for x in 0..(width) {
            for y in 0..(height) {
                let x_0 = MandelBrot::scale_x_value(width as f64, x, x_min, x_max);
                let y_0 = MandelBrot::scale_y_value(height as f64, y, y_min, y_max);
                let is_divergent = !(MandelBrot::is_divergent(x_0, y_0, max_iterations));
                if (is_divergent) {
                    pixels.extend(&[0, 0, 0, 255])
                } else {
                    pixels.extend(&[255, 255, 255, 255])
                }
            }
        }
        return pixels;
    }
    fn is_divergent(x: f64, y: f64, max_iterations: u32) -> bool {
        let mut x_curr: f64 = 0.0;
        let mut y_curr: f64 = 0.0;
        let mut num_iterations = 0;
        for i in 0..max_iterations {
            let x_temp = (x_curr.powf(2.0) - y_curr.powf(2.0)) + x;
            let y_temp = (2.0 * x_curr * y_curr) + y;
            x_curr = x_temp;
            y_curr = y_temp;
            if (x_curr.powf(2.0) + y_curr.powf(2.0)) > 4.0 {
                return true;
            }
        }
        return false;
    }

    fn scale_x_value(range: f64, value: u16, x_min: f64, x_max: f64) -> f64 {
        let MAX_X: f64 = x_max;
        let MIN_X: f64 = x_min;
        let div = (MAX_X - MIN_X) / range;
        return MIN_X + (div * value as f64);
        // let normalized = value as f64 / range;

        // return (MAX_X - MIN_X) * normalized;
    }

    fn scale_y_value(range: f64, value: u16, y_min: f64, y_max: f64) -> f64 {
        let MAX_Y: f64 = y_max;
        let MIN_Y: f64 = y_min;

        let div = (MAX_Y - MIN_Y) / range;
        return MIN_Y + (div * value as f64);
        // let normalized = value as f64 / range;
        // return (MAX_Y - MIN_Y) * normalized;
    }
}
#[cfg(test)]
mod tests {
    use core::panic;

    use crate::MandelBrot;

    #[test]
    fn check_escape() {
        assert_eq!(MandelBrot::is_divergent(0.0, 1.0, 100), false);
        assert_eq!(MandelBrot::is_divergent(-0.5, 0.5, 100), false);
        assert_eq!(MandelBrot::is_divergent(1.0, 0.0, 100), true);
        assert_eq!(MandelBrot::is_divergent(-1.0, 0.0, 100), false);
        assert_eq!(MandelBrot::is_divergent(-1.0, 0.1, 100), false);
    }

    #[test]
    fn test_generate_image() {
        let pixels = MandelBrot::get_pixels(1000, 750, 100, -2.0, 2.0, -1.12, 1.12);
        let mut file = std::fs::File::create("outputrust.png").unwrap();
        match png_encode_mini::write_rgba_from_u8(&mut file, &pixels[..], 750, 1000) {
            Ok(_) => {}
            Err(err) => {
                panic!("error writing to png file");
            }
        };
    }

    #[test]
    fn test_count_divergent() {
        let pixels = MandelBrot::get_pixels_binary(1000, 750, 100, -2.0, 2.0, -1.12, 1.12);
        let mut mandelbrotCount = 0;
        for pixel in pixels.into_iter() {
            if (pixel == 1 as u8) {
                mandelbrotCount += 1;
            }
        }
        println!(" mandelbrotcount - {}", mandelbrotCount);
    }
}
