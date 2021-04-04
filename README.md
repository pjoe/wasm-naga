### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --release --target nodejs
```

### Goals

The target is reaching an output size acceptable for using run time in browser based apps.

This would be something in the 100KB (gz) range.

### Current status

Output size development (for glsl-in, wgsl-in, spv-out)

```
$ ll pkg/wasm_naga_bg.wasm

-rw-r--r-- 1 code code 694014 Apr  4 09:12 pkg/wasm_naga_bg.wasm (glsl-in, wgsl-in, spv-out, msl-out)
-rw-r--r-- 1 code code 559600 Feb 22 12:26 pkg/wasm_naga_bg.wasm (glsl-in, wgsl-in, spv-out)
-rw-r--r-- 1 code code 480386 Jan 26 07:44 pkg/wasm_naga_bg.wasm (glsl-in, wgsl-in, spv-out)
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
