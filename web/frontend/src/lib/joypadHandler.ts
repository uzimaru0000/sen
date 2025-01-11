import { InputState } from "../bindings/InputState";

export class JoypadHandler {
  private inputState: InputState;

  constructor() {
    this.inputState = {
      a: false,
      b: false,
      select: false,
      start: false,
      up: false,
      down: false,
      left: false,
      right: false,
    };

    document.addEventListener("keydown", (e) => {
      switch (e.key) {
        case "a":
          this.inputState.a = true;
          break;
        case "s":
          this.inputState.b = true;
          break;
        case "z":
          this.inputState.select = true;
          break;
        case "x":
          this.inputState.start = true;
          break;
        case "ArrowUp":
          this.inputState.up = true;
          break;
        case "ArrowDown":
          this.inputState.down = true;
          break;
        case "ArrowLeft":
          this.inputState.left = true;
          break;
        case "ArrowRight":
          this.inputState.right = true;
          break;
      }
    });
    document.addEventListener("keyup", (e) => {
      switch (e.key) {
        case "a":
          this.inputState.a = false;
          break;
        case "s":
          this.inputState.b = false;
          break;
        case "z":
          this.inputState.select = false;
          break;
        case "x":
          this.inputState.start = false;
          break;
        case "ArrowUp":
          this.inputState.up = false;
          break;
        case "ArrowDown":
          this.inputState.down = false;
          break;
        case "ArrowLeft":
          this.inputState.left = false;
          break;
        case "ArrowRight":
          this.inputState.right = false;
          break;
      }
    });
  }

  handle() {
    return this.inputState;
  }
}
