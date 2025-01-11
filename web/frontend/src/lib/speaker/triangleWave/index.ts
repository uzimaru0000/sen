import workletUrl from "./worklet?worker&url";

class InnerTriangleWave extends AudioWorkletNode {
  constructor(context: AudioContext) {
    super(context, "triangleWaveProcessor", {
      parameterData: {
        frequency: 440,
        volume: 0,
      },
    });
  }

  get frequency() {
    return this.parameters.get("frequency")!;
  }

  get volume() {
    return this.parameters.get("volume")!;
  }
}

export type TriangleWave = InnerTriangleWave;

export const createTriangleWave = async (context: AudioContext) => {
  await context.audioWorklet.addModule(workletUrl);

  const node = new InnerTriangleWave(context);
  return node;
};
