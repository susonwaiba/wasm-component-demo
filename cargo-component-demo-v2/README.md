# cargo-component-demo-v2

This repository is sample implementation.

https://github.com/WebAssembly/component-model


## Components

- adder

rust-component

- calculator

depends upon adder for add

- cli-app

depends upon calculator and adder and uses `wasi:cli/run` runtime

- proxy-app

depends upon adder and uses `wasmtime` + `wasmtime-wasi` runtime


## Dependencies

rust and cargo

https://github.com/bytecodealliance/cargo-component

https://github.com/bytecodealliance/wasmtime

https://github.com/bytecodealliance/wasm-tools

## Quick start

```console
just
just cargo-install
just install
just build
```

- Running cli-app

```console
just run
```

- Running proxy-app

```console
just serve
```

```console
curl http://127.0.0.1:8080
```


## Inspect *.wasm file

```console
wasm-tools component wit target/wasm32-wasip2/release/adder.wasm
wasm-tools component wit target/wasm32-wasip2/release/calculator.wasm
wasm-tools component wit cli-app-1.wasm
wasm-tools component wit cli-app.wasm
wasm-tools component wit proxy-app-1.wasm
wasm-tools component wit proxy-app.wasm
```
