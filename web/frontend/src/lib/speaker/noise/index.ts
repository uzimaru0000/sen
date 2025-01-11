import workletUrl from "./worklet?worker&url";

export const LONG = 1;
export const SHORT = 0;

class InnerNoiseNode extends AudioWorkletNode {
  constructor(context: AudioContext) {
    super(context, "noiseProcessor", {
      parameterData: {
        volume: 1.0,
        mode: LONG,
        frequency: 440,
      },
    });
  }

  get volume() {
    return this.parameters.get("volume")!;
  }

  get mode() {
    return this.parameters.get("mode")!;
  }

  get frequency() {
    return this.parameters.get("frequency")!;
  }
}

export type NoiseNode = InnerNoiseNode;

export const createNoiseNode = async (context: AudioContext) => {
  await context.audioWorklet.addModule(workletUrl);

  const node = new InnerNoiseNode(context);
  return node;
};
