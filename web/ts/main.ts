import init, { WebLevel } from "../pkg/web.js";

const PX_SIZE = 8;

async function run() {
  await init();

  const res = await fetch("level.bmp");
  const buf = await res.arrayBuffer();
  const view = new Uint8Array(buf);
  const level = WebLevel.new(view);
  const width = level.get_width();
  const height = level.get_height();

  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width * PX_SIZE}px`;
  canvas.style.height = `${height * PX_SIZE}px`;

  document.body.appendChild(canvas);

  const ctx = canvas.getContext('2d');

  if (!ctx) {
    throw new Error("Unable to obtain 2d canvas context");
  }

  const imgData = ctx.createImageData(width, height);
  const imgDataBuf = imgData.data.buffer;
  const uint8Array = new Uint8Array(imgDataBuf);
  level.draw(uint8Array);

  ctx.putImageData(imgData, 0, 0);

  console.log(`level: ${width}x${height}`);

  level.free();
}

run();
