import { RectWave, createRectWave } from "./rectWave";
import { JsSpeakerEvent } from "../../bindings/JsSpeakerEvent";
import { TriangleWave, createTriangleWave } from "./triangleWave";
import { createNoiseNode, LONG, NoiseNode, SHORT } from "./noise";

export class Speaker {
  private gain: GainNode;
  private ch1: RectWave | undefined;
  private ch2: RectWave | undefined;
  private ch3: TriangleWave | undefined;
  private ch4: NoiseNode | undefined;

  constructor(private context: AudioContext) {
    this.gain = context.createGain();

    createRectWave(context).then((node) => {
      this.ch1 = node;
      this.ch1.connect(this.gain);
    });
    createRectWave(context).then((node) => {
      this.ch2 = node;
      this.ch2.connect(this.gain);
    });
    createTriangleWave(context).then((node) => {
      this.ch3 = node;
      this.ch3.connect(this.gain);
    });
    createNoiseNode(context).then((node) => {
      this.ch4 = node;
      this.ch4.connect(this.gain);
    });

    this.gain.connect(context.destination);
  }

  setVolume(volume: number) {
    this.gain.gain.setValueAtTime(volume, this.context.currentTime);
  }

  send(ch: number, event: JsSpeakerEvent) {
    switch (ch) {
      case 1:
        if (event.type === "SquareNote" && this.ch1) {
          this.ch1.frequency.setValueAtTime(event.hz, this.context.currentTime);
          this.ch1.duty.setValueAtTime(event.duty, this.context.currentTime);
          this.ch1.volume.setValueAtTime(
            event.volume,
            this.context.currentTime
          );
        }
        return;
      case 2:
        if (event.type === "SquareNote" && this.ch2) {
          this.ch2.frequency.setValueAtTime(event.hz, this.context.currentTime);
          this.ch2.duty.setValueAtTime(event.duty, this.context.currentTime);
          this.ch2.volume.setValueAtTime(
            event.volume,
            this.context.currentTime
          );
        }
        return;
      case 3:
        if (event.type === "TriangleNote" && this.ch3) {
          this.ch3.frequency.setValueAtTime(event.hz, this.context.currentTime);
        }
        return;
      case 4:
        if (event.type === "NoiseNote" && this.ch4) {
          this.ch4.frequency.setValueAtTime(event.hz, this.context.currentTime);
          this.ch4.volume.setValueAtTime(
            event.volume,
            this.context.currentTime
          );

          switch (event.mode) {
            case "Long":
              this.ch4.mode.setValueAtTime(LONG, this.context.currentTime);
              break;
            case "Short":
              this.ch4.mode.setValueAtTime(SHORT, this.context.currentTime);
              break;
          }
        }
        return;
    }
  }
}
