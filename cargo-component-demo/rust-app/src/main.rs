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
        Component::from_file(&engine, "target/wasm32-wasi/release/adder.wasm")?;

    // let mut linker = Linker::new(&engine);
    let linker = Linker::new(&engine);
    // wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)?;
    // Adder::add_to_linker(&mut linker, |cx| cx)?;

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);
    let (bindings, _) = Adder::instantiate(&mut store, &component, &linker)?;

    let result = bindings.interface0.call_add(&mut store, 2, 3)?;
    println!("result: {}", result);
    Ok(())
}

fn main() {
    load_wasm_adder().unwrap();
    println!("Hello, world!");
}
