import init, { WebLevel } from "../pkg/web.js";
const PX_SIZE = 8;
const INITIAL_FPS = 15;
async function fetchBytes(url) {
    const res = await fetch(url);
    if (!res.ok) {
        throw new Error(`Fetching "${url}" failed with HTTP ${res.status}`);
    }
    const buf = await res.arrayBuffer();
    return new Uint8Array(buf);
}
function getElement(selector, classObj) {
    const thing = document.querySelector(selector);
    if (!thing) {
        throw new Error(`Nothing matches selector "${selector}"`);
    }
    if (!(thing instanceof classObj)) {
        throw new Error(`Expected selector "${selector}" to match a ${classObj.name}`);
    }
    return thing;
}
function toPositiveFloat(value) {
    const result = parseFloat(value);
    if (isNaN(result)) {
        throw new Error(`Expected "${value}" to be convertible to a float`);
    }
    if (result <= 0) {
        throw new Error(`Expected "${value}" to be greater than zero`);
    }
    return result;
}
function setCanvasSize(canvas, level) {
    const width = level.get_width();
    const height = level.get_height();
    canvas.width = width;
    canvas.height = height;
    canvas.style.width = `${width * PX_SIZE}px`;
    canvas.style.height = `${height * PX_SIZE}px`;
}
function createCanvasImageData(ctx) {
    const imgData = ctx.createImageData(ctx.canvas.width, ctx.canvas.height);
    const imgDataBuf = imgData.data.buffer;
    const uint8Array = new Uint8Array(imgDataBuf);
    return { imgData, uint8Array };
}
function getCanvasCtx2d(canvas) {
    const ctx = canvas.getContext('2d');
    if (!ctx) {
        throw new Error("Unable to obtain 2d canvas context");
    }
    return ctx;
}
async function run() {
    await init();
    const level = WebLevel.new(await fetchBytes("level.bmp"));
    const fpsRange = getElement('#fps', HTMLInputElement);
    const rainCheckbox = getElement('#rain', HTMLInputElement);
    const canvas = getElement('#canvas', HTMLCanvasElement);
    setCanvasSize(canvas, level);
    const ctx = getCanvasCtx2d(canvas);
    const { imgData, uint8Array } = createCanvasImageData(ctx);
    let timeout = 0;
    const syncRain = () => level.set_enable_water_factories(rainCheckbox.checked);
    rainCheckbox.onchange = syncRain;
    syncRain();
    const drawFrame = () => {
        level.draw(uint8Array);
        ctx.putImageData(imgData, 0, 0);
        level.tick();
        timeout = window.setTimeout(drawFrame, 1000 / toPositiveFloat(fpsRange.value));
    };
    drawFrame();
    return () => {
        level.free();
        window.clearTimeout(timeout);
    };
}
run();
//# sourceMappingURL=main.js.map