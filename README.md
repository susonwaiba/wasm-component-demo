# wasm-component-demo

State: Under Testing

This repository is sample implementation.

https://github.com/WebAssembly/component-model


## Rust

- adder

rust-component

- calculator

depends upon adder for add

- cli-app

depends upon adder and uses `wasi:cli/run` runtime

- actix-app

depends upon adder and uses `wasmtime` + `wasmtime-wasi` runtime

- rust-app

depends upon adder and uses `wasmtime` + `wasmtime-wasi` runtime

- node-app

depends upon adder and uses `jco` transpile and `nodejs` runtime


## Dependencies

https://github.com/bytecodealliance/cargo-component

https://github.com/bytecodealliance/wasmtime

https://github.com/bytecodealliance/WASI-Virt

https://github.com/bytecodealliance/wasm-tools

https://github.com/bytecodealliance/jco


## Quick start

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
```

```console
curl http://127.0.0.1:8080
curl http://127.0.0.1:8080/wasm
curl http://127.0.0.1:8080/wasm-at-startup
```

- Running node-app

```console
jco transpile adder/target/wasm32-wasi/release/adder.wasm -o node-app/component
(cd node-app && npm install)
(cd node-app && npm run start)
```

## Inspect *.wasm file

```console
wasm-tools component wit calculator.composed.wasm
wasm-tools component wit app.wasm
```
