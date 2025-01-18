import { RenderBuffer } from "../bindings/RenderBuffer";

const WIDTH = 256;
const HEIGHT = 240;

export class Renderer {
  constructor(private canvas: HTMLCanvasElement) {}

  render(buf: RenderBuffer) {
    const ctx = this.canvas.getContext("2d");
    if (!ctx) {
      return;
    }

    const imageData = ctx.createImageData(WIDTH, HEIGHT);
    for (let y = 0; y < HEIGHT; y++) {
      for (let x = 0; x < WIDTH; x++) {
        const base = (y * WIDTH + x) * 3;
        const r = buf.data[base];
        const g = buf.data[base + 1];
        const b = buf.data[base + 2];

        const offset = (y * WIDTH + x) * 4;
        imageData.data[offset] = r;
        imageData.data[offset + 1] = g;
        imageData.data[offset + 2] = b;
        imageData.data[offset + 3] = 255;
      }
    }

    ctx.putImageData(imageData, 0, 0);
  }

  setCanvas(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
  }

  reset() {
    const ctx = this.canvas.getContext("2d");
    if (!ctx) {
      return;
    }

    ctx.clearRect(0, 0, WIDTH, HEIGHT);
  }
}
