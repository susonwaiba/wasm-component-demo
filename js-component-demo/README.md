# js-component-demo

This repository is sample implementation.

https://github.com/WebAssembly/component-model


## Components

- js-divider

javascript-component

- js-app

depends upon js-divider and uses `jco` transpile and `nodejs` runtime

- divider-rust-app

depends upon js-divider and uses `wasmtime` + `wasmtime-wasi` runtime


## Dependencies

nodejs

https://github.com/bytecodealliance/jco

rust and cargo


## Quick start

```console
just build
```

- Running js-divider

```console
just run-component-test
```

- Running js-app

```console
just js-transpile
just run-js-app
```

- Running rust-app

```console
just run-rust-app
```


## Inspect *.wasm file

```console
wasm-tools component wit js-divider/js-divider.wasm
```

