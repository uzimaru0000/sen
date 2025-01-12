import init from "../wasm";
import { App } from "./App";
import { createRoot } from "react-dom/client";

const main2 = async () => {
  await init();

  const rootDom = document.getElementById("root");

  if (!rootDom) {
    return;
  }

  const root = createRoot(rootDom);
  root.render(<App />);
};

main2();
