const LUT_SIZE = 4096;
const sincLUT = new Float32Array(LUT_SIZE + 1);

// sinc(x) для x ∈ [0..1]
for (let i = 0; i <= LUT_SIZE; i++) {
  const x = i / LUT_SIZE;
  sincLUT[i] = x === 0 ? 1 : Math.sin(Math.PI * x) / (Math.PI * x);
}

function sinc(x) {
  const ax = Math.abs(x);
  if (ax >= 1) return 0;
  return sincLUT[(ax * LUT_SIZE) | 0];
}

function lanczos(x, a) {
  const ax = Math.abs(x);
  if (ax >= a) return 0;
  return sinc(x) * sinc(x / a);
}

function clamp(v) {
  return v < 0 ? 0 : v > 255 ? 255 : v;
}

export function scaleImageLanczos2Pass(src, targetWidth, targetHeight, a = 3) {

  const srcW = src.width;
  const srcH = src.height;
  const srcData = src.data;

  const scaleX = srcW / targetWidth;
  const scaleY = srcH / targetHeight;

  // ─────────────────────────────────────────────
  // 1 PASS — горизонтальный
  // ─────────────────────────────────────────────

  const tmp = new Float32Array(targetWidth * srcH * 4);

  for (let y = 0; y < srcH; y++) {

    const rowOffsetSrc = y * srcW * 4;
    const rowOffsetTmp = y * targetWidth * 4;

    for (let x = 0; x < targetWidth; x++) {

      const srcX = (x + 0.5) * scaleX - 0.5;

      const xStart = Math.floor(srcX - a + 1);
      const xEnd   = Math.floor(srcX + a);

      let r = 0, g = 0, b = 0, aSum = 0;
      let weightSum = 0;

      for (let ix = xStart; ix <= xEnd; ix++) {
        if (ix < 0 || ix >= srcW) continue;

        const w = lanczos(srcX - ix, a);
        if (w === 0) continue;

        const si = rowOffsetSrc + ix * 4;

        r += srcData[si]     * w;
        g += srcData[si + 1] * w;
        b += srcData[si + 2] * w;
        aSum += srcData[si + 3] * w;

        weightSum += w;
      }

      const ti = rowOffsetTmp + x * 4;
      const inv = 1 / weightSum;

      tmp[ti]     = r * inv;
      tmp[ti + 1] = g * inv;
      tmp[ti + 2] = b * inv;
      tmp[ti + 3] = aSum * inv;
    }
  }

  // ─────────────────────────────────────────────
  // 2 PASS — вертикальный
  // ─────────────────────────────────────────────

  const dst = new ImageData(targetWidth, targetHeight);
  const dstData = dst.data;

  for (let y = 0; y < targetHeight; y++) {

    const srcY = (y + 0.5) * scaleY - 0.5;

    const yStart = Math.floor(srcY - a + 1);
    const yEnd   = Math.floor(srcY + a);

    for (let x = 0; x < targetWidth; x++) {

      let r = 0, g = 0, b = 0, aSum = 0;
      let weightSum = 0;

      for (let iy = yStart; iy <= yEnd; iy++) {
        if (iy < 0 || iy >= srcH) continue;

        const w = lanczos(srcY - iy, a);
        if (w === 0) continue;

        const ti = (iy * targetWidth + x) * 4;

        r += tmp[ti]     * w;
        g += tmp[ti + 1] * w;
        b += tmp[ti + 2] * w;
        aSum += tmp[ti + 3] * w;

        weightSum += w;
      }

      const di = (y * targetWidth + x) * 4;
      const inv = 1 / weightSum;

      dstData[di]     = clamp(r * inv);
      dstData[di + 1] = clamp(g * inv);
      dstData[di + 2] = clamp(b * inv);
      dstData[di + 3] = clamp(aSum * inv);
    }
  }

  return dst;
}


