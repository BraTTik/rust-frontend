import { readI32Array, writeI32Array } from "./number";

/**
 * @param num {number}
 * @returns {boolean}
 */
export function readBoolean(num) {
  return Boolean(num)
}

/**
 * @param bool {boolean}
 * @returns {number}
 */
export function writeBoolean(bool) {
  return Number(bool);
}

export function readBooleanArray(ptr, instance) {
  let h = readI32Array(ptr, instance);
  return readI32Array(h[0], instance).map(readBoolean);
}

export function writeBooleanArray(array, instance) {
  let bools = writeI32Array(array.map(writeBoolean), instance);

  return writeI32Array([bools.ptr, bools.size], instance);
}