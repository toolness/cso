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

function toPositiveFloat(value: string): number {
  const result = parseFloat(value);

  if (isNaN(result)) {
    throw new Error(`Expected "${value}" to be convertible to a float`);
  }

  if (result <= 0) {
    throw new Error(`Expected "${value}" to be greater than zero`);
  }

  return result;
}

function setCanvasSize(canvas: HTMLCanvasElement, level: WebLevel) {
  const width = level.get_width();
  const height = level.get_height();

  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width * PX_SIZE}px`;
  canvas.style.height = `${height * PX_SIZE}px`;
}

function createCanvasImageData(ctx: CanvasRenderingContext2D) {
  const imgData = ctx.createImageData(ctx.canvas.width, ctx.canvas.height);
  const imgDataBuf = imgData.data.buffer;
  const uint8Array = new Uint8Array(imgDataBuf);
  return { imgData, uint8Array };
}

function getCanvasCtx2d(canvas: HTMLCanvasElement): CanvasRenderingContext2D {
  const ctx = canvas.getContext('2d');

  if (!ctx) {
    throw new Error("Unable to obtain 2d canvas context");
  }

  return ctx;
}

type TimeoutKind = "raf"|"timeout";

const CANCEL_TIMEOUT: { [key in TimeoutKind]: (timeout: number) => void } = {
  "raf": timeout => window.cancelAnimationFrame(timeout),
  "timeout": timeout => window.clearTimeout(timeout),
};

async function run() {
  await init();

  const level = WebLevel.new(await fetchBytes("level.bmp"));
  const fpsRange = getElement('#fps', HTMLInputElement);
  const rainRange = getElement('#rain', HTMLInputElement);
  const canvas = getElement('#canvas', HTMLCanvasElement);

  setCanvasSize(canvas, level);

  const ctx = getCanvasCtx2d(canvas);
  const {imgData, uint8Array} = createCanvasImageData(ctx);
  let timeout = 0;
  let timeoutKind: TimeoutKind = "timeout";

  const scheduleNextFrame = () => {
    const fps = toPositiveFloat(fpsRange.value);
    if (fps < 60) {
      timeout = window.setTimeout(drawFrame, 1000 / fps);
      timeoutKind = "timeout";
    } else {
      window.requestAnimationFrame(drawFrame);
      timeoutKind = "raf";
    }
  };

  const drawFrame = () => {
    const rain = parseInt(rainRange.value);
    if (rain === 0) {
      level.set_enable_water_factories(false);
    } else if (rain > 0) {
      level.set_override_water_factory_count(rain);
      level.set_enable_water_factories(true);
    }
    level.draw(uint8Array);
    ctx.putImageData(imgData, 0, 0);
    level.tick();
    scheduleNextFrame();
  }

  drawFrame();

  return () => {
    level.free();
    CANCEL_TIMEOUT[timeoutKind](timeout);
  };
}

run();
