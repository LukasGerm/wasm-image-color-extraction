import init, { extract_color } from "./src/wasm_image_color_extraction.js";

init();

onmessage = async (e) => {
  const url = e.data;
  const color = await extract_color(url);
  postMessage(color);
};
