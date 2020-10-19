use std::{env, fs, str};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    println!("reading: {}", args[1]);
    let input_vu8 = fs::read(&args[1]).unwrap();
    let input = str::from_utf8(&input_vu8).unwrap();
    println!("input: {}", input);
    let module = wasm_naga::glsl_in(input, "vertex");
    println!("module: {}", module);
    println!("result: {:?}", wasm_naga::spv_out(module));
    println!("done");
}
