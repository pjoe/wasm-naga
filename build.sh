#!/bin/bash
set -e

rm -rf pkg

# web build
wasm-pack build --release --target web
mkdir pkg/web
mv pkg/wasm_naga* pkg/web/

# node build
wasm-pack build --release --target nodejs

# add web build to package.json
sed -i '/  "files":/a \ \ \ \ "web",' pkg/package.json
sed -i '/  "main":/a \ \ "module": "web/wasm_naga.js",' pkg/package.json