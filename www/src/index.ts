import { MandelBrot } from "mandelbrot-generator";

const WIDTH = 1000;
const HEIGHT = 750;
// const pixels = MandelBrot.get_pixels_binary(
//   WIDTH,
//   HEIGHT,
//   100,
//   -2.0,
//   2.0,
//   -1.12,
//   1.12
// );

// console.log(pixels.length);
// console.log(countDivergent(pixels));
// const canvas = <HTMLCanvasElement>document.getElementById("mandelbrot-canvas");
// if (canvas) {
//   let ctx = canvas.getContext("2d");
//   if (ctx) {
//     let ImageData = ctx.getImageData(0, 0, WIDTH, HEIGHT);
//     console.log(ImageData.data.length);
//     for (let i = 0; i < ImageData.data.length / 4; i++) {}
//     ctx.putImageData(ImageData, 0, 0);
//   }
// }

const canvas = <HTMLCanvasElement>document.getElementById("mandelbrot-canvas");
let pixels_t = [];

for (let y = 0; y <= 750; y++) {
  for (let x = 0; x <= WIDTH; x++) {
    if (x < WIDTH / 2 && y < HEIGHT / 2) {
      pixels_t.push(0);
    } else if (x > WIDTH / 2 && y > HEIGHT / 2) {
      pixels_t.push(3);
    } else if (x < WIDTH / 2 && y > HEIGHT / 2) {
      pixels_t.push(2);
    } else if (x > WIDTH / 2 && y < HEIGHT / 2) {
      pixels_t.push(1);
    }
  }
}
console.log(pixels_t);
if (canvas) {
  let ctx = canvas.getContext("2d");
  if (ctx) {
    let ImageData = ctx.getImageData(0, 0, WIDTH, HEIGHT);
    for (let i = 0; i < ImageData.data.length / 4; i++) {
      if (pixels_t[i] == 0) {
        ImageData.data[4 * i] = 255;
        ImageData.data[4 * i + 1] = 0;
        ImageData.data[4 * i + 2] = 0;
        ImageData.data[4 * i + 3] = 255;
      } else if (pixels_t[i] == 1) {
        ImageData.data[4 * i] = 0;
        ImageData.data[4 * i + 1] = 255;
        ImageData.data[4 * i + 2] = 0;
        ImageData.data[4 * i + 3] = 255;
      } else if (pixels_t[i] == 2) {
        ImageData.data[4 * i] = 0;
        ImageData.data[4 * i + 1] = 0;
        ImageData.data[4 * i + 2] = 255;
        ImageData.data[4 * i + 3] = 255;
      } else if (pixels_t[i] == 3) {
        ImageData.data[4 * i] = 0;
        ImageData.data[4 * i + 1] = 0;
        ImageData.data[4 * i + 2] = 0;
        ImageData.data[4 * i + 3] = 255;
      }
    }

    ctx.putImageData(ImageData, 0, 0);
  }
}

function mandelbrotToString(pixels: Uint8Array) {
  let mandelbrotString = "";
  for (let pixel of pixels) {
    if (pixel == 1) {
      mandelbrotString = mandelbrotString.concat(".");
    } else {
      mandelbrotString = mandelbrotString.concat(" ");
    }
  }
  return mandelbrotString;
}

function countDivergent(pixels: Uint8Array) {
  let mandelbrotCount = 0;
  for (let pixel of pixels) {
    if (pixel == 1) {
      mandelbrotCount += 1;
    }
  }
  return mandelbrotCount;
}
