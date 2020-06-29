### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --release --target nodejs
```

### Goals

The target is reaching an output size acceptable for using run time in browser based apps.

This would be some thing in the 100KB (gz) range.

### Current status

Output size 11-jun-2020 for GLSL > MSL (new glsl front WIP)

```
$ ll pkg/wasm_naga_bg.wasm
-rw-r--r-- 1 code code 243428 Jun 29 17:11 pkg/wasm_naga_bg.wasm
```

Output size 11-jun-2020 for GLSL > MSL ('old' glsl front)

```
$ ll pkg
total 536
drwxr-xr-x 2 code code   4096 Jun 11 05:21 ./
drwxr-xr-x 8 code code   4096 May 19 14:07 ../
-rw-r--r-- 1 code code      1 Jun 11 05:21 .gitignore
-rw-r--r-- 1 code code    480 Jun 11 05:21 README.md
-rw-r--r-- 1 code code    281 Jun 11 05:21 package.json
-rw-r--r-- 1 code code    144 Jun 11 05:21 wasm_naga.d.ts
-rw-r--r-- 1 code code   2143 Jun 11 05:21 wasm_naga.js
-rw-r--r-- 1 code code    340 Jun 11 05:21 wasm_naga_bg.d.ts
-rw-r--r-- 1 code code   1216 May 19 13:16 wasm_naga_bg.js
-rw-r--r-- 1 code code 510289 Jun 11 05:21 wasm_naga_bg.wasm
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
