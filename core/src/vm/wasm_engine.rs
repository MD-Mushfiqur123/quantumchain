//! WASM execution engine

/// WASM virtual machine
pub struct WasmEngine {
    // TODO: Implement with wasmtime
}

impl WasmEngine {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WasmEngine {
    fn default() -> Self {
        Self::new()
    }
}
