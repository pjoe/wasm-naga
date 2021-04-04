mod utils;

use wasm_bindgen::prelude::*;

use lazy_static::lazy_static;
use naga::Module;
use std::collections::HashMap;
use std::sync::Mutex;

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
    static ref MODULES: Mutex<ModuleStore> = Mutex::new(ModuleStore {
        store: HashMap::new(),
        next_idx: 0,
    });
}

#[cfg(feature = "glsl-in")]
#[wasm_bindgen]
pub fn glsl_in(input: &str, stage: &str) -> Result<usize, JsValue> {
    glsl_in_inner(input, stage).map_err(|e| e.into())
}
#[cfg(feature = "glsl-in")]
pub fn glsl_in_inner(input: &str, stage: &str) -> Result<usize, String> {
    utils::set_panic_hook();
    let shader_stage = match stage {
        "vertex" => naga::ShaderStage::Vertex,
        "fragment" => naga::ShaderStage::Fragment,
        "compute" => naga::ShaderStage::Compute,
        _ => return Err("stage not supported".into()),
    };
    let mut entry_points: naga::FastHashMap<String, naga::ShaderStage> = Default::default();
    entry_points.insert("main".to_string(), shader_stage);
    let module = naga::front::glsl::parse_str(
        &input,
        &naga::front::glsl::Options {
            defines: Default::default(),
            entry_points,
        },
    )
    .map_err(|e| format!("{}", e))?;
    Ok(MODULES.lock().unwrap().append(module))
}

#[cfg(feature = "wgsl-in")]
#[wasm_bindgen]
pub fn wgsl_in(input: &str) -> Result<usize, JsValue> {
    wgsl_in_inner(input).map_err(|e| e.into())
}
#[cfg(feature = "wgsl-in")]
pub fn wgsl_in_inner(input: &str) -> Result<usize, String> {
    utils::set_panic_hook();
    let module = naga::front::wgsl::parse_str(&input).map_err(|e| format!("{}", e))?;
    Ok(MODULES.lock().unwrap().append(module))
}

#[cfg(feature = "msl-out")]
#[wasm_bindgen]
pub fn msl_out(module: usize) -> Result<String, JsValue> {
    msl_out_inner(module).map_err(|e| e.into())
}

#[cfg(feature = "msl-out")]
pub fn msl_out_inner(module: usize) -> Result<String, String> {
    utils::set_panic_hook();
    match MODULES.lock().unwrap().remove(module) {
        None => Err("module not found".into()),
        Some(module) => {
            use naga::back::msl;
            let options: msl::Options = Default::default();
            let analysis = naga::valid::Validator::new(naga::valid::ValidationFlags::all())
                .validate(&module)
                .map_err(|e| format!("{}", e))?;
            let (str, _) =
                msl::write_string(&module, &analysis, &options).map_err(|e| format!("{:?}", e))?;
            Ok(str)
        }
    }
}

#[cfg(feature = "spv-out")]
#[wasm_bindgen]
pub fn spv_out(module: usize) -> Result<Box<[u8]>, JsValue> {
    spv_out_inner(module).map_err(|e| e.into())
}

pub fn spv_out_inner(module: usize) -> Result<Box<[u8]>, String> {
    use naga::back::spv;
    use naga::FastHashSet;

    utils::set_panic_hook();
    let module_id = { MODULES.lock().unwrap().remove(module) };
    match module_id {
        None => Err("module not found".into()),
        Some(module) => {
            let analysis = naga::valid::Validator::new(naga::valid::ValidationFlags::all())
                .validate(&module)
                .map_err(|e| format!("{}", e))?;
            let capabilities: FastHashSet<spv::Capability> =
                vec![spv::Capability::Shader].into_iter().collect();
            let spv = spv::write_vec(
                &module,
                &analysis,
                &naga::back::spv::Options {
                    lang_version: (1, 0),
                    capabilities,
                    flags: spv::WriterFlags::empty(),
                },
            )
            .map_err(|e| format!("{}", e))?;

            let bytes = spv
                .iter()
                .fold(Vec::with_capacity(spv.len() * 4), |mut v, w| {
                    v.extend_from_slice(&w.to_le_bytes());
                    v
                });

            Ok(bytes.into_boxed_slice())
        }
    }
}
