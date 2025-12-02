import * as fs from 'node:fs/promises';

const file = await fs.readFile('./wasm/target/wasm32-unknown-unknown/debug/wasm.wasm');

const module = await WebAssembly.compile(file);
const instance = await WebAssembly.instantiate(module, {
  env: {
    console_log(ptr, len) {
      const mem = instance.exports.memory.buffer;
      console.log(new TextDecoder().decode(mem.slice(ptr, ptr + len)));
    }
  }
});

new Int32Array(instance.exports.memory.buffer).set([1, 2, 3]);

console.log(instance.exports.sum(0, 3));