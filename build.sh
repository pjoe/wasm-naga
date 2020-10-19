#!/bin/bash
set -e

rm -rf pkg

# esm build
wasm-pack build --release
mkdir pkg/esm
mv pkg/wasm_naga* pkg/esm/

# node build
wasm-pack build --release --target nodejs

# add esm build to package.json
sed -i '/  "files":/a \ \ \ \ "esm",' pkg/package.json
sed -i '/  "main":/a \ \ "module": "esm/wasm_naga.js",' pkg/package.json