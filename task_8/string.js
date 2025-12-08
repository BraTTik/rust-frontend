import { writeI32Array } from './number';

const BYTE_LENGTH = 1;
/**
 * @param str {string}
 * @param instance {WebAssembly.Instance}
 * @returns {{ ptr: number, size: number }}
 */
export function writeString(str, instance) {
  const ptr = instance.exports.malloc(str.length * BYTE_LENGTH);

  let arr = new Uint8Array(instance.exports.memory.buffer, ptr);

  for (let i = 0; i < str.length; i++) {
    arr[i] = str.charCodeAt(i);
  }

  return {
    ptr,
    size: str.length,
  };
}

export function readString(ptr, instance) {
  let header = new Uint32Array(instance.exports.memory.buffer, ptr, 2);
  let [arr_ptr, len] = header;
  const arr = new Uint8Array(instance.exports.memory.buffer, ...header);
  const str = String.fromCharCode(...arr);

  instance.exports.free(ptr, 2 * 4);
  instance.exports.free(arr_ptr, len);

  return str;
}

/**
 * @param array {string[]}
 * @param instance {WebAssembly.Instance}
 * @returns {{ ptr: number, size: number }}
 */
export function writeStringArray(array, instance) {
  const stringHeaders =  array.map(str => {
    const { ptr, size } = writeString(str, instance);
    return [ptr, size];
  });

  const headersStringH = stringHeaders.map(([ptr, size]) => writeI32Array([ptr, size], instance));
  const header = writeI32Array(headersStringH.map(h => h.ptr), instance);

  return header;
}