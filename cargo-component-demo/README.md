# cargo-component-demo

This repository is sample implementation.

https://github.com/WebAssembly/component-model


## Components

- adder

rust-component

- calculator

depends upon adder for add

- cli-app

depends upon calculator and adder and uses `wasi:cli/run` runtime

- rust-app

depends upon adder and uses `wasmtime` + `wasmtime-wasi` runtime

- actix-app

depends upon adder and uses `wasmtime` + `wasmtime-wasi` runtime

- node-app

depends upon adder and uses `jco` transpile and `nodejs` runtime

- express-app

depends upon adder and uses `jco` transpile and `browser` as runtime


## Dependencies

rust and cargo

https://github.com/bytecodealliance/cargo-component

https://github.com/bytecodealliance/wasmtime

https://github.com/bytecodealliance/wasm-tools

nodejs

https://github.com/bytecodealliance/jco


## Quick start

```console
just
just build
```

- Running cli-app

```console
just run-cli-app
```

- Running rust-app

```console
just run-rust-app
```

- Running actix-app

```console
just serve-actix-app
```

```console
curl http://127.0.0.1:8080
curl http://127.0.0.1:8080/wasm
curl http://127.0.0.1:8080/wasm-at-startup
```

- Running node-app

```console
just js-transpile
just run-node-app
```

- Running express-app

```console
just js-transpile
just run-node-app

# Visit http://127.0.0.1:3000 via browser
```


## Inspect *.wasm file

```console
wasm-tools component wit target/wasm32-wasi/release/adder.wasm
wasm-tools component wit target/wasm32-wasi/release/calculator.wasm
wasm-tools component wit target/wasm32-wasi/release/calculator.composed.wasm
wasm-tools component wit target/wasm32-wasi/release/cli-app.wasm
wasm-tools component wit target/wasm32-wasi/release/cli-app.composed.wasm
```

