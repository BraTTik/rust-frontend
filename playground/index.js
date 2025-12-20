import init, { sum, greet, create_post } from "./demo/pkg/demo.js"

async function run() {
  await init();
  console.log(sum(new Int32Array([1, 2, 3, 4, 5])).slice());

  greet("Rust", 2);

  console.log(create_post("Post Test"))
}

run();