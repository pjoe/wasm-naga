use std::{env, fs, str};

fn main() {
    let mut args = env::args().collect::<Vec<_>>();
    args.remove(0);
    for arg in args {
        println!("reading: {}", arg);
        let input_vu8 = fs::read(&arg).unwrap();
        let input = str::from_utf8(&input_vu8).unwrap();
        println!("input: {}", input);
        let module = wasm_naga::glsl_in_inner(input, "vertex").unwrap();
        println!("module: {}", module);
        println!("result: {:?}", wasm_naga::spv_out(module));
    }
    println!("done");
}
