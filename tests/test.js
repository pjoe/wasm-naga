const naga = require("../pkg/wasm_naga");
const fs = require("fs");
const path = require("path");

if (process.argv.length < 3) {
  console.log(`usage: node ${process.argv[1]} <input_file>`);
  process.exit(-1);
}

const fname = process.argv[2];
const inPath = path.parse(fname);
const input = fs.readFileSync(fname, "utf8");

let moduleIdx = 0;
switch (inPath.ext) {
  case ".vert":
    moduleIdx = naga.glsl_in(input, "vertex");
    break;
  case ".frag":
    moduleIdx = naga.glsl_in(input, "fragment");
    break;
  case ".comp":
    moduleIdx = naga.glsl_in(input, "compute");
    break;
  case ".wgsl":
    moduleIdx = naga.wgsl_in(input);
    break;
}

console.log("parsed module:", moduleIdx);
const spv = naga.spv_out(moduleIdx);
console.log(`spv_out: ${spv.byteLength} byte`);
const outName = path.format({
  ...inPath,
  base: undefined,
  ext: ".spv",
});
console.log("writing to:", outName);
fs.writeFileSync(outName, spv);
