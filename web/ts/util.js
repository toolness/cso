export async function fetchBytes(url) {
    const res = await fetch(url);
    if (!res.ok) {
        throw new Error(`Fetching "${url}" failed with HTTP ${res.status}`);
    }
    const buf = await res.arrayBuffer();
    return new Uint8Array(buf);
}
export function getElement(selector, classObj) {
    const thing = document.querySelector(selector);
    if (!thing) {
        throw new Error(`Nothing matches selector "${selector}"`);
    }
    if (!(thing instanceof classObj)) {
        throw new Error(`Expected selector "${selector}" to match a ${classObj.name}`);
    }
    return thing;
}
export function toPositiveFloat(value) {
    const result = parseFloat(value);
    if (isNaN(result)) {
        throw new Error(`Expected "${value}" to be convertible to a float`);
    }
    if (result <= 0) {
        throw new Error(`Expected "${value}" to be greater than zero`);
    }
    return result;
}
export function createCanvasImageData(ctx) {
    const imgData = ctx.createImageData(ctx.canvas.width, ctx.canvas.height);
    const imgDataBuf = imgData.data.buffer;
    const uint8Array = new Uint8Array(imgDataBuf);
    return { imgData, uint8Array };
}
export function getCanvasCtx2d(canvas) {
    const ctx = canvas.getContext('2d');
    if (!ctx) {
        throw new Error("Unable to obtain 2d canvas context");
    }
    return ctx;
}
export const CANCEL_TIMEOUT = {
    "raf": timeout => window.cancelAnimationFrame(timeout),
    "timeout": timeout => window.clearTimeout(timeout),
};
//# sourceMappingURL=util.js.map