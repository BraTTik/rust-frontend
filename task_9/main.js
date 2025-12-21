import init, { scaleImageLanczos as wasmScaleImageLanczos, initSincLUT, scale_x2 } from "./lanczos/pkg/lanczos.js";
import { scaleImageLanczos2Pass } from "./lancoz.js";
(async () => {
  await init();
  initSincLUT();
  /**
   *
   * @param el {HTMLImageElement}
   */
  function getImageData(el) {
    const canvas = document.createElement('canvas');
    canvas.width = el.naturalWidth;
    canvas.height = el.naturalHeight;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(el, 0, 0);
    return ctx.getImageData(0, 0, canvas.width, canvas.height);
  }

  const widthInput = document.getElementById('width');
  const heightInput = document.getElementById('height');

  const jsButton = document.getElementById('scale-js');
  const wasmButton = document.getElementById('scale-wasm');
  const x2Button = document.getElementById('x2');

  /**
   *
   * @param imageData {ImageData}
   */
  function drawImage(imageData) {
    const width = imageData.width;
    const height = imageData.height;
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    canvas.width = width;
    canvas.height = height;

    ctx.putImageData(imageData, 0, 0);
  }

  drawImage(getImageData(document.getElementById('img')));

  widthInput.addEventListener('input', () => {
    heightInput.value = widthInput.value;
  });

  jsButton.addEventListener('click', scaleJs);
  wasmButton.addEventListener('click', scaleWasm);
  x2Button.addEventListener('click', scaleX2Wasm);

  function scaleWasm() {
    const [targetWidth, targetHeight] = getTargetDimensions();
    if (!targetWidth || !targetHeight) return;
    const imageData = getImageData(document.getElementById('img'));
    console.time('wasm');
    const result = wasmScaleImageLanczos(imageData.data, imageData.width, imageData.height, targetWidth, targetHeight, 2);
    console.timeEnd('wasm');
    const resultData = new ImageData(targetWidth, targetHeight);
    resultData.data.set(result);
    drawImage(resultData);
  }

  function scaleJs() {
    const [targetWidth, targetHeight] = getTargetDimensions();
    if (!targetWidth || !targetHeight) return;
    console.time('js');
    const imageData = getImageData(document.getElementById('img'));
    const data = scaleImageLanczos2Pass(imageData, targetWidth, targetHeight, 2);
    console.timeEnd('js');
    drawImage(data);
  }

  function getTargetDimensions() {
    return [
      Number(widthInput.value),
      Number(heightInput.value),
    ]
  }

  function scaleX2Wasm() {
    const canvas = document.getElementById('canvas');
    scale_x2(canvas);
    widthInput.value = canvas.width;
    heightInput.value = canvas.height;
  }
})()
