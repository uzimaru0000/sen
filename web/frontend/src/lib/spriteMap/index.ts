export class SpriteMap {
  private image: HTMLImageElement;
  private isLoaded: Promise<void>;
  private context: CanvasRenderingContext2D;

  constructor(private canvas: HTMLCanvasElement, src: string) {
    this.context = this.canvas.getContext("2d")!;

    this.image = new Image();
    this.image.src = src;

    this.isLoaded = new Promise<void>((res, rej) => {
      this.image.onload = () => {
        res();
      };

      this.image.onerror = (e) => {
        rej(e);
      };
    });
  }

  clear() {
    this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }

  async draw(tile: number, x: number, y: number, rotate: number = 0) {
    await this.isLoaded;

    const centerX = x * 16 + 8;
    const centerY = y * 16 + 8;

    this.context.save();

    this.context.translate(centerX, centerY);
    this.context.rotate((rotate * Math.PI) / 180);

    const sx = Math.floor(tile % 8) * 16;
    const sy = Math.floor(tile / 8) * 16;
    this.context.drawImage(this.image, sx, sy, 16, 16, -8, -8, 16, 16);

    this.context.restore();
  }
}
