import {readString,  writeStringArray} from "./string";
import {readI32Array, writeFloat32Array, writeI32Array} from './number';
import {readBoolean, readBooleanArray, writeBooleanArray} from "./boolean";

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

  // const str = writeString("Hello, world", instance);
  // instance.exports.log_string(str.ptr, str.size);
  //
  // readString(instance.exports.get_string(), instance);

  const { ptr: s_ptr, size: s_size } = writeI32Array([1, 2, 3], instance)
  console.log(instance.exports.sum_i32_array(s_ptr, s_size));
  const { ptr: r_ptr, size: r_size } = writeI32Array([2, 4, 10], instance);
  const r = instance.exports.pow_i32_array(r_ptr, r_size);
  console.log(readI32Array(r, instance));


  const { ptr: f_ptr, size: f_size } = writeFloat32Array([1.1, 2.2, 3.3], instance);
  console.log(instance.exports.sum_f32_array(f_ptr, f_size))

  const helloWorld = writeStringArray(["Hello, ", "world!", " From", " ", "Rust", "!"], instance);
  const joined_str = instance.exports.join_str(helloWorld.ptr);

  console.log(readString(joined_str, instance));

  console.log("2 = 2", readBoolean(instance.exports.compare_two_num(2, 2)));
  console.log("0 = 2", readBoolean(instance.exports.compare_two_num(0, 2)));

  instance.exports.print_bool_arr(writeBooleanArray([false, true, false], instance).ptr);

  console.log(readBooleanArray(instance.exports.get_bool_arr(), instance));
}

init();