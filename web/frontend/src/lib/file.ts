export const readFile = async (file: File): Promise<Uint8Array> => {
  return new Promise((res, rej) => {
    const reader = new FileReader();
    reader.onload = () => {
      const buffer = reader.result as ArrayBuffer;
      res(new Uint8Array(buffer));
    };
    reader.onerror = rej;

    reader.readAsArrayBuffer(file);
  });
};
