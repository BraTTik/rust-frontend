const BYTE_LENGTH = 1;
/**
 * @param str {string}
 * @param instance {WebAssembly.Instance}
 * @returns {*}
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
  const arr = new Uint8Array(instance.exports.memory.buffer, ...header);
  const str = String.fromCharCode(...arr);
  instance.exports.free(ptr);

  return str;
}

/**
 *
 * @param arr {string[]}
 * @param instance {WebAssembly.Instance}
 */
export function writeStringArray(arr, instance) {

}
