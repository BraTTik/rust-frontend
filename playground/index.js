
async function init() {
  const wasm = await WebAssembly.instantiateStreaming(await fetch('./wasm/target/wasm32-unknown-unknown/debug/wasm.wasm'), {
    env: {}
  });

  const instance = wasm.instance;

  const ptr = instance.exports.malloc(3 * 4);

  const arr = new Int32Array(instance.exports.memory.buffer, ptr)
  arr.set([1, 2, 3]);

  console.log(instance.exports.sum(ptr, 3))
}

init();