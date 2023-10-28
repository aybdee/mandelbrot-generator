import { MandelBrot } from "mandelbrot-generator";

const WIDTH = 100;
const HEIGHT = 100;
const pixels = MandelBrot.get_pixels(WIDTH, HEIGHT, 1000);
const canvas = <HTMLCanvasElement>document.getElementById("mandelbrot-canvas");
if (canvas) {
  let ctx = canvas.getContext("2d");
  if (ctx) {
    let ImageData = ctx.createImageData(WIDTH, HEIGHT);
    for (let i = 0; i < ImageData.data.length; i += 4) {
      ImageData.data[i] = 255;
      ImageData.data[i + 1] = 255;
      ImageData.data[i + 2] = 255;
    }
    ctx.putImageData(ImageData, 0, 0);
    ctx.putImageData(ImageData, 0, 0);
    console.log(ctx.getImageData(0, 0, 100, 100));
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
