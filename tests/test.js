const naga = require("../pkg/wasm_naga");
const fs = require("fs");
const path = require("path");

if (process.argv.length < 3) {
  console.log(`usage: node ${process.argv[1]} [--msl] <input_file>`);
  process.exit(-1);
}

let outMode = "spv";
for (const arg of process.argv.slice(2)) {
  const fname = arg;
  console.log("arg:", fname);
  if (fname === "--msl") {
    outMode = "msl";
    continue;
  }
  const inPath = path.parse(fname);
  const input = fs.readFileSync(fname, "utf8");

  try {
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
    switch (outMode) {
      case "msl": {
        const msl = naga.msl_out(moduleIdx);
        const outName = path.format({
          ...inPath,
          base: undefined,
          ext: ".msl",
        });
        console.log("writing to:", outName);
        fs.writeFileSync(outName, msl);
        break;
      }
      case "spv":
      default: {
        const spv = naga.spv_out(moduleIdx);
        console.log(`spv_out: ${spv.byteLength} byte`);
        const outName = path.format({
          ...inPath,
          base: undefined,
          ext: ".spv",
        });
        console.log("writing to:", outName);
        fs.writeFileSync(outName, spv);
        break;
      }
    }
  } catch (error) {
    console.error("caught:", error);
  }
}
console.log("Done");
