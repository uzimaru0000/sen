import { JsNoiseMode } from "../../../bindings/JsNoiseMode";

export class NoiseGenerator {
  private bit: number;
  private register = 1;

  constructor(mode: JsNoiseMode) {
    switch (mode) {
      case "Long":
        this.bit = 1;
        break;
      case "Short":
        this.bit = 6;
        break;
    }
  }

  next(): boolean {
    const feedback =
      (this.register & 0x01) ^ ((this.register >> this.bit) & 0x01);
    this.register = this.register >> 1;
    this.register = (this.register & 0x3fff) | (feedback << 14);

    return (this.register & 0x01) === 0;
  }
}
