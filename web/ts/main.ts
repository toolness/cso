import init, { WebLevel } from "../pkg/web.js";

const PX_SIZE = 8;
const INITIAL_FPS = 15;

async function fetchBytes(url: string): Promise<Uint8Array> {
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`Fetching "${url}" failed with HTTP ${res.status}`)
  }
  const buf = await res.arrayBuffer();
  return new Uint8Array(buf);
}

// https://2ality.com/2020/04/classes-as-values-typescript.html
type Class<T> = new (...args: any[]) => T;

function getElement<T extends Element>(selector: string, classObj: Class<T>): T {
  const thing = document.querySelector(selector);
  if (!thing) {
    throw new Error(`Nothing matches selector "${selector}"`);
  }
  if (!(thing instanceof classObj)) {
    throw new Error(`Expected selector "${selector}" to match a ${classObj.name}`);
  }
  return thing;
}

async function run() {
  await init();

  const level = WebLevel.new(await fetchBytes("level.bmp"));
  const width = level.get_width();
  const height = level.get_height();

  const canvas = getElement('#canvas', HTMLCanvasElement);
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width * PX_SIZE}px`;
  canvas.style.height = `${height * PX_SIZE}px`;

  const ctx = canvas.getContext('2d');

  if (!ctx) {
    throw new Error("Unable to obtain 2d canvas context");
  }

  const imgData = ctx.createImageData(width, height);
  const imgDataBuf = imgData.data.buffer;
  const uint8Array = new Uint8Array(imgDataBuf);

  let timeout = 0;

  const drawFrame = () => {
    level.draw(uint8Array);
    ctx.putImageData(imgData, 0, 0);
    level.tick();
    timeout = setTimeout(drawFrame, 1000 / INITIAL_FPS);
  }

  drawFrame();

  return () => {
    level.free();
    clearTimeout(timeout);
  };
}

run();
