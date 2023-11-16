# wasm-component-demo

State: Under Testing

This repository is sample implementation.

https://github.com/WebAssembly/component-model


## Rust

- adder
- calculator

depends upon adder for add

- cli-app

depends upon adder and uses `wasi:cli/run` runtime

- actix-app

depends upon adder and uses `wasmtime` + `wasmtime-wasi` runtime


## Quick start

- Dependencies

https://github.com/bytecodealliance/cargo-component

https://github.com/bytecodealliance/wasmtime

https://github.com/bytecodealliance/WASI-Virt

https://github.com/bytecodealliance/wasm-tools


- Running rust-app

```console
chmod +x app.sh
./app.sh build
(cd rust-app && ./target/release/rust-app)
```

- Running cli-app

```console
chmod +x app.sh
./app.sh build
./app.sh run
```

- Running actix-app

```console
chmod +x app.sh
./app.sh build
./app.sh serve

# test
curl http://127.0.0.1:8080
curl http://127.0.0.1:8080/wasm
curl http://127.0.0.1:8080/wasm-at-startup
```

- Inspect *.wasm file

```console
wasm-tools component wit calculator.composed.wasm
wasm-tools component wit app.wasm
```
