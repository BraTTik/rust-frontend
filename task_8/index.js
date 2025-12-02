import {readString, writeString} from "./string";

const I32_BYTE_LENGTH = 4

async function init() {
  const wasm = await WebAssembly.instantiateStreaming(await fetch('./wasm/target/wasm32-unknown-unknown/debug/wasm.wasm'), {
    env: {
      console_log(ptr, size) {
        console.log(new TextDecoder().decode(mem().buffer.slice(ptr, ptr + size)));
      }
    }
  });

  const mem = () => wasm.instance.exports.memory;

  const instance = wasm.instance;

  const arr = [5, 2, 3, 4];
  const ptr = instance.exports.malloc(arr.length * I32_BYTE_LENGTH);

  new Int32Array(mem().buffer, ptr).set(arr);

  instance.exports.free(ptr);

  const str = writeString("Hello, world", instance);
  instance.exports.log_string(str.ptr, str.size);

  readString(instance.exports.get_string(), instance);

}

init();