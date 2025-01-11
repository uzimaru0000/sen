import init, { WebEmulator } from "../wasm";
import { JoypadHandler } from "./lib/joypadHandler";
import { Renderer } from "./lib/renderer";
import { Speaker } from "./lib/speaker";

const main = async () => {
  await init();

  const state = {
    emulator: null as WebEmulator | null,
  };

  const fileInput = document.getElementById("file") as HTMLInputElement;
  const startBtn = document.getElementById("start") as HTMLButtonElement;
  const canvas = document.getElementById("canvas") as HTMLCanvasElement;

  startBtn.disabled = true;

  fileInput.addEventListener("change", async () => {
    if (!fileInput.files) {
      return;
    }

    const file = fileInput.files[0];
    const buffer = await file.arrayBuffer();

    const rom = new Uint8Array(buffer);
    const speaker = new Speaker(new AudioContext());
    const joypadHandler = new JoypadHandler();
    const renderer = new Renderer(canvas);

    speaker.setVolume(0.1);

    state.emulator = new WebEmulator(rom, speaker, joypadHandler, renderer);
    state.emulator.reset();

    startBtn.disabled = false;
  });

  startBtn.addEventListener("click", () => {
    startBtn.disabled = true;

    const run = () => {
      if (!state.emulator) {
        return;
      }

      for (let i = 0; i < 10000; i++) {
        state.emulator.step();
      }

      requestAnimationFrame(run);
    };

    run();
  });
};

main();
