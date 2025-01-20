import { WebEmulator } from "../../wasm/web";
import { JoypadHandler } from "./joypadHandler";
import { Renderer } from "./renderer";
import { Speaker } from "./speaker";

export class Emulator {
  private speaker: Speaker;
  private joypadHandler: JoypadHandler;
  private renderer: Renderer;
  private emulator: WebEmulator;
  private handler: number | undefined;

  constructor(
    rom: Uint8Array,
    context: AudioContext,
    canvas: HTMLCanvasElement
  ) {
    this.speaker = new Speaker(context);
    this.joypadHandler = new JoypadHandler();
    this.renderer = new Renderer(canvas);
    this.emulator = new WebEmulator(
      rom,
      this.speaker,
      this.joypadHandler,
      this.renderer
    );
    this.speaker.setVolume(0.5);
  }

  setCanvas(canvas: HTMLCanvasElement) {
    this.renderer.setCanvas(canvas);
  }

  setMasterVolume(volume: number) {
    this.speaker.setVolume(volume);
  }

  start() {
    this.reset();

    const run = () => {
      for (let i = 0; i < 10000; i++) {
        this.emulator.step();
      }

      this.handler = requestAnimationFrame(run);
    };

    run();
  }

  stop() {
    if (!this.handler) {
      return;
    }

    cancelAnimationFrame(this.handler);
    this.reset();
  }

  reset() {
    this.emulator.reset();
    this.speaker.reset();
    this.renderer.reset();
  }
}
