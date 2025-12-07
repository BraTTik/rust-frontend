const I32_BYTE_LENGTH = 4;
const I64_BYTE_LENGTH = 8;

function readHeader(ptr, instance) {
  const header = new Int32Array(instance.exports.memory.buffer, ptr, 2);
  const [arr_ptr, len] = header;
  instance.exports.free(ptr, 2 * I32_BYTE_LENGTH);

  return [arr_ptr, len];
}


/**
 *
 * @param array {number[]}
 * @param instance {WebAssembly.Instance}
 * @returns {{ ptr: number, size: number }}
 */
export function writeI32Array(array, instance) {
  const ptr = instance.exports.malloc(array.length * I32_BYTE_LENGTH);
  const a = new Int32Array(instance.exports.memory.buffer, ptr, array.length)
  a.set(array)
  return { ptr, size: array.length };
}

/**
 *
 * @param ptr {number}
 * @param instance {WebAssembly.Instance}
 * @returns {number[]}
 */
export function readI32Array(ptr, instance) {
  const [arr_ptr, len] = readHeader(ptr, instance);
  const arr = new Int32Array(instance.exports.memory.buffer, arr_ptr, len);
  const r = [];
  for (let i = 0; i < arr.length; i++) {
    r.push(arr[i]);
  }
  instance.exports.free(arr_ptr, len * I32_BYTE_LENGTH);
  return r;
}

export function readI64Array(ptr, instance) {
  const [arr_ptr, len] = readHeader(ptr, instance);
  instance.exports.free(ptr, 2 * I32_BYTE_LENGTH);
  const arr = new BigInt64Array(instance.exports.memory.buffer, arr_ptr, len);
  const r = [];
  for (let i = 0; i < arr.length; i++) {
    r.push(arr[i]);
  }
  instance.exports.free(arr_ptr, len * I32_BYTE_LENGTH);

  return r;
}

export function writeI64Array(array, instance) {
  const ptr = instance.exports.malloc(array.length * I64_BYTE_LENGTH);
  const a = new BigInt64Array(instance.exports.memory.buffer, ptr, array.length)
  a.set(array)
  return { ptr, size: array.length };
}

export function readFloat32Array(ptr, instance) {
  const [arr_ptr, len] = readHeader(ptr, instance);
  const a = new Float32Array(instance.exports.memory.buffer, arr_ptr, len);

  const r = [];
  for (let i = 0; i < a.length; i++) {
    r.push(a[i]);
  }
  instance.exports.free(arr_ptr, len * I32_BYTE_LENGTH);

  return r;
}

export function writeFloat32Array(array, instance) {
  const ptr = instance.exports.malloc(array.length * I32_BYTE_LENGTH);
  const a = new Float32Array(instance.exports.memory.buffer, ptr, array.length)
  a.set(array)
  return { ptr, size: array.length };
}

export function readFloat64Array(ptr, instance) {
  const [arr_ptr, len] = readHeader(ptr, instance);
  const a = new Float64Array(instance.exports.memory.buffer, arr_ptr, len);
  const r = [];
  for (let i = 0; i < a.length; i++) {
    r.push(a[i]);
  }
  return r;
}