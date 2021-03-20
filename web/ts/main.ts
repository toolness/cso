import init, { WebLevel } from "../pkg/web.js";

async function run() {
  await init();

  const res = await fetch("level.bmp");
  const buf = await res.arrayBuffer();
  const view = new Uint8Array(buf);
  const level = WebLevel.new(view);

  console.log(`level: ${level.get_width()}x${level.get_height()}`);

  level.free();
}

run();
