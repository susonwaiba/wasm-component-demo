# wasm-component-demo

State: Under Testing

This repository is sample implementation.

https://github.com/WebAssembly/component-model


## Components

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

- express-app

depends upon adder and uses `jco` transpile and `browser` as runtime

- js-divider

javascript-component

- js-app

depends upon js-divider and uses `jco` transpile and `nodejs` runtime

- divider-rust-app

depends upon js-divider and uses `wasmtime` + `wasmtime-wasi` runtime

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
wasmtime --wasm component-model cli-app/target/wasm32-wasi/release/cli-app.wasm

# Not ready yet
# ./app.sh run
```

- Running actix-app

```console
chmod +x app.sh
./app.sh build
./app.sh serve

# Or

(cd actix-app && ./target/release/actix-app)
```

```console
curl http://127.0.0.1:8080
curl http://127.0.0.1:8080/wasm
curl http://127.0.0.1:8080/wasm-at-startup
```

- Running node-app

```console
jco transpile adder/target/wasm32-wasi/release/adder.wasm -o node-app/component
(cd node-app && npm run start)
```

- Running express-app

```console
jco transpile adder/target/wasm32-wasi/release/adder.wasm -o express-app/public/component
(cd express-app && npm install)
(cd express-app && npm run start)

# Visit http://127.0.0.1:3000 via browser
```

- Running js-divider

```console
(cd js-divider && npm install)
(cd js-divider && npm run build)
```

- Running js-app

```console
jco transpile js-divider/js-divider.wasm -o js-app/component
(cd js-app && npm start)
```

- Running divider-rust-app

```console
(cd divider-rust-app && cargo build --release)
(cd divider-rust-app && ./target/release/divider-rust-app)
```

## Inspect *.wasm file

```console
wasm-tools component wit adder/target/wasm32-wasi/release/adder.wasm
wasm-tools component wit calculator/target/wasm32-wasi/release/calculator.wasm
wasm-tools component wit cli-app/target/wasm32-wasi/release/cli-app.wasm
wasm-tools component wit calculator.composed.wasm
wasm-tools component wit cli-app.virt.wasm
wasm-tools component wit app.wasm
wasm-tools component wit js-divider/js-divider.wasm
```
