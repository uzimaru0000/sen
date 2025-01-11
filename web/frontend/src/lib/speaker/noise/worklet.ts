import { NoiseGenerator } from "./generator";

class NoiseAudioProcessor extends AudioWorkletProcessor {
  private phase = 0;
  private value = false;
  private generator = new NoiseGenerator("Long");

  static get parameterDescriptors() {
    return [
      {
        name: "volume",
        defaultValue: 1.0,
      },
      {
        name: "mode",
        defaultValue: 1,
      },
      {
        name: "frequency",
        defaultValue: 440,
      },
    ];
  }

  constructor() {
    super();
  }

  process(
    _: Float32Array[][],
    outputs: Float32Array[][],
    parameters: Record<string, Float32Array>
  ) {
    const output = outputs[0];
    const volume = parameters.volume;
    const mode = parameters.mode;
    const frequency = parameters.frequency;

    if (mode[0] === 1) {
      this.generator = new NoiseGenerator("Long");
    } else {
      this.generator = new NoiseGenerator("Short");
    }

    for (let channel = 0; channel < output.length; channel++) {
      const outputChannel = output[channel];

      for (let i = 0; i < outputChannel.length; i++) {
        const currentFrequency =
          frequency.length > 1 ? frequency[i] : frequency[0];
        const currentVolume = volume.length > 1 ? volume[i] : volume[0];

        if (this.value) {
          outputChannel[i] = currentVolume;
        } else {
          outputChannel[i] = 0;
        }

        const currentPhase = this.phase;
        this.phase =
          (this.phase + currentFrequency / globalThis.sampleRate) % 1.0;
        if (currentPhase > this.phase) {
          this.value = this.generator.next();
        }
      }
    }

    return true;
  }
}

registerProcessor("noiseProcessor", NoiseAudioProcessor);
