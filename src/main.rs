use std::{env, ffi::OsStr, fs, path::Path, str};

fn main() {
    let mut args = env::args().collect::<Vec<_>>();
    args.remove(0);
    for arg in args {
        println!("reading: {}", arg);
        let input_vu8 = fs::read(&arg).unwrap();
        let input = str::from_utf8(&input_vu8).unwrap();
        println!("input: {}", input);

        let ext = Path::new(&arg).extension().and_then(OsStr::to_str);
        let module = match ext {
            Some("vert") => wasm_naga::glsl_in_inner(input, "vertex").unwrap(),
            Some("frag") => wasm_naga::glsl_in_inner(input, "fragment").unwrap(),
            Some("comp") => wasm_naga::glsl_in_inner(input, "compute").unwrap(),
            Some("wgsl") => wasm_naga::wgsl_in_inner(input).unwrap(),
            _ => panic!("Unknown extension for: {}", arg),
        };
        println!("module: {}", module);
        println!("result: {:?}", wasm_naga::msl_out_inner(module).unwrap());
    }
    println!("done");
}
