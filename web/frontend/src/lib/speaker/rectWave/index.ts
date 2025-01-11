import workletUrl from "./worklet?worker&url";

class InnerRectWave extends AudioWorkletNode {
  constructor(context: AudioContext) {
    super(context, "rectWaveProcessor", {
      parameterData: {
        frequency: 440,
        duty: 0.5,
        volume: 0,
      },
    });
  }

  get frequency() {
    return this.parameters.get("frequency")!;
  }

  get duty() {
    return this.parameters.get("duty")!;
  }

  get volume() {
    return this.parameters.get("volume")!;
  }
}

export type RectWave = InnerRectWave;

export const createRectWave = async (context: AudioContext) => {
  await context.audioWorklet.addModule(workletUrl);

  const node = new InnerRectWave(context);
  return node;
};
