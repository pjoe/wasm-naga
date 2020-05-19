mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn convert(input: &str) -> String {
    let module = naga::front::wgsl::parse_str(&input).unwrap();

    format!("Compiled, header {:?}", module.header);

    use naga::back::msl;
    let mut binding_map = msl::BindingMap::default();
    binding_map.insert(
        msl::BindSource { set: 0, binding: 0 },
        msl::BindTarget { buffer: None, texture: Some(1), sampler: None, mutable: false },
    );
    binding_map.insert(
        msl::BindSource { set: 0, binding: 1 },
        msl::BindTarget { buffer: None, texture: None, sampler: Some(1), mutable: false },
    );
    let options = msl::Options {
        binding_map: &binding_map,
    };
    msl::write_string(&module, options).unwrap()
}