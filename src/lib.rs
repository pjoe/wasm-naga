mod utils;

use wasm_bindgen::prelude::*;

use naga::Module;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct ModuleStore {
    store: HashMap<usize, Module>,
    next_idx: usize,
}

impl ModuleStore {
    fn append(&mut self, module: Module) -> usize {
        let idx = self.next_idx;
        self.next_idx += 1;
        self.store.insert(idx, module);
        idx
    }
    fn remove(&mut self, idx: usize) -> Option<Module> {
        self.store.remove(&idx)
    }
}

lazy_static! {
    static ref MODULES: Mutex<ModuleStore> = Mutex::new(ModuleStore{
        store: HashMap::new(),
        next_idx: 0,
    });
}

#[wasm_bindgen]
pub fn glsl_front(input: &str) -> usize {
    utils::set_panic_hook();
    let module =
        naga::front::glsl_new::parse_str(&input, String::from("main"), naga::ShaderStage::Vertex)
            .unwrap();
    MODULES.lock().unwrap().append(module)
}

#[wasm_bindgen]
pub fn msl_back(module: usize) -> String {
    utils::set_panic_hook();
    match MODULES.lock().unwrap().remove(module) {
        None => "Error: module not found".to_string(),
        Some(module) => {
            use naga::back::msl;
            let mut binding_map = msl::BindingMap::default();
            binding_map.insert(
                msl::BindSource { set: 0, binding: 0 },
                msl::BindTarget {
                    buffer: None,
                    texture: Some(1),
                    sampler: None,
                    mutable: false,
                },
            );
            binding_map.insert(
                msl::BindSource { set: 0, binding: 1 },
                msl::BindTarget {
                    buffer: None,
                    texture: None,
                    sampler: Some(1),
                    mutable: false,
                },
            );
            let options = msl::Options {
                binding_map: &binding_map,
            };
            msl::write_string(&module, options).unwrap()
        }
    }
}
