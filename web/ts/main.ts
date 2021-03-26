import init, { WebLevel } from "../pkg/web.js";
import {
  getElement,
  fetchBytes,
  getCanvasCtx2d,
  createCanvasImageData,
  toPositiveFloat,
  TimeoutInfo,
  CANCEL_TIMEOUT
} from "./util.js";

const PX_SIZE = 8;

function setCanvasSize(canvas: HTMLCanvasElement, level: WebLevel) {
  const width = level.get_width();
  const height = level.get_height();

  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width * PX_SIZE}px`;
  canvas.style.height = `${height * PX_SIZE}px`;
}

async function run() {
  await init();

  const level = WebLevel.new(await fetchBytes("level.bmp"));
  const fpsRange = getElement('#fps', HTMLInputElement);
  const rainRange = getElement('#rain', HTMLInputElement);
  const canvas = getElement('#canvas', HTMLCanvasElement);

  setCanvasSize(canvas, level);

  const ctx = getCanvasCtx2d(canvas);
  const {imgData, uint8Array} = createCanvasImageData(ctx);
  let timeoutInfo: TimeoutInfo|null = null;

  const scheduleNextFrame = () => {
    const fps = toPositiveFloat(fpsRange.value);
    if (fps < 60) {
      timeoutInfo = {
        kind: "timeout",
        timeout: window.setTimeout(drawFrame, 1000 / fps),
      };
    } else {
      timeoutInfo = {
        kind: "raf",
        timeout: window.requestAnimationFrame(drawFrame),
      };
    }
  };

  const configureRain = () => {
    const rain = parseInt(rainRange.value);
    if (rain === 0) {
      level.set_enable_water_factories(false);
    } else if (rain > 0) {
      level.set_override_water_factory_count(rain);
      level.set_enable_water_factories(true);
    }
  };

  const drawFrame = () => {
    configureRain();
    level.draw(uint8Array);
    ctx.putImageData(imgData, 0, 0);
    level.tick();
    scheduleNextFrame();
  };

  const shutdown = () => {
    level.free();
    if (timeoutInfo) {
      CANCEL_TIMEOUT[timeoutInfo.kind](timeoutInfo.timeout);
    }
  };

  drawFrame();

  return shutdown;
}

run();
