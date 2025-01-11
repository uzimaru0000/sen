class TriangleWaveProcessor extends AudioWorkletProcessor {
  private phase = 0;

  static get parameterDescriptors() {
    return [
      { name: "frequency", defaultValue: 440, automationRate: "a-rate" },
      {
        name: "volume",
        defaultValue: 1.0,
        minValue: 0.0,
        maxValue: 1.0,
        automationRate: "a-rate",
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
    const sampleRate = globalThis.sampleRate;
    const frequency = parameters.frequency;
    const volume = parameters.volume;

    for (let channel = 0; channel < output.length; channel++) {
      const outputChannel = output[channel];

      for (let i = 0; i < outputChannel.length; i++) {
        const currentFrequency =
          frequency.length > 1 ? frequency[i] : frequency[0];
        const currentVolume = volume.length > 1 ? volume[i] : volume[0];

        const base = this.phase <= 0.5 ? this.phase : 1.0 - this.phase;
        outputChannel[i] = (base * 4.0 - 1.0) * currentVolume;

        this.phase = (this.phase + currentFrequency / sampleRate) % 1.0;
      }
    }

    return true; // プロセッサを続行
  }
}

registerProcessor("triangleWaveProcessor", TriangleWaveProcessor);
