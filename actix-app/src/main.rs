use actix_web::{web, App, HttpServer, Responder};
use anyhow::Result;
use std::error::Error;
use std::sync::Mutex;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

bindgen!();

struct AppWasmState {
    bindings: Adder,
    store: Store<WasiCtx>,
}

struct AppState {
    adder_state: Mutex<Option<AppWasmState>>,
}

async fn load_wasm() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component =
        Component::from_file(&engine, "./../adder/target/wasm32-wasi/release/adder.wasm")?;
    let linker = Linker::new(&engine);

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);
    let (bindings, _) = Adder::instantiate(&mut store, &component, &linker)?;

    let result = bindings.interface0.call_add(&mut store, 2, 3)?;
    println!("result: {}", result);
    Ok(())
}

async fn wasm() -> impl Responder {
    if let Err(err) = load_wasm().await {
        return format!("Error loading wasm: {}", err);
    }
    "wasm".to_string()
}

async fn load_wasm_at_startup() -> Result<AppWasmState, Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component =
        Component::from_file(&engine, "./../adder/target/wasm32-wasi/release/adder.wasm")?;
    let linker = Linker::new(&engine);
    // wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)?;
    // Adder::add_to_linker(&mut linker, |cx| cx)?;

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);
    let (bindings, _) = Adder::instantiate(&mut store, &component, &linker)?;

    Ok(AppWasmState { bindings, store })
}

async fn wasm_at_startup(app_state: web::Data<AppState>) -> impl Responder {
    let mut adder_state = app_state.adder_state.lock().unwrap();
    let wasm_state = adder_state.as_mut().unwrap();
    let bindings = &mut wasm_state.bindings;
    let wasm_store = &mut wasm_state.store;
    let result = bindings.interface0.call_add(wasm_store, 1, 3).unwrap();
    println!("startup result: {}", result);
    format!("startup result: {}", result)
}

async fn index() -> impl Responder {
    "Hello world from actix!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut app_data = web::Data::new(AppState {
        adder_state: None.into(),
    });

    let wasm_state = load_wasm_at_startup().await;
    match wasm_state {
        Ok(wasm_state) => {
            app_data = web::Data::new(AppState {
                adder_state: Some(wasm_state).into(),
            });

            // let result = wasm_state.bindings.interface0.call_add(&mut wasm_state.store, 2, 3).unwrap();
            // println!("pre result: {}", result);
            println!("wasm was loaded at startup");
        }
        Err(err) => println!("Error loading wasm: {}", err),
    }

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/", web::get().to(index))
            .route("/wasm", web::get().to(wasm))
            .route("/wasm-at-startup", web::get().to(wasm_at_startup))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
