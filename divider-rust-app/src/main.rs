use anyhow::Result;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::sync::WasiCtxBuilder;

bindgen!();

fn load_wasm_adder() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component =
        Component::from_file(&engine, "./../js-divider/js-divider.wasm")?;

    let linker = Linker::new(&engine);

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);
    let (bindings, _) = Divider::instantiate(&mut store, &component, &linker)?;

    let result = bindings.call_divide(&mut store, 4, 2)?;
    println!("result: {}", result);
    Ok(())
}

fn main() {
    load_wasm_adder().unwrap();
    println!("Hello, world!");
}
