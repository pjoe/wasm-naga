const naga = require("../pkg/wasm_naga");
const fs = require("fs");

if (process.argv.length < 3) {
  console.log(`usage: node ${process.argv[1]} <input_file>`);
  process.exit(-1);
}

const input = fs.readFileSync(process.argv[2], "utf8");
const moduleIdx = naga.glsl_front(input);
console.log("glsl_front:", moduleIdx);
console.log("msl_back:", naga.msl_back(moduleIdx));
