#!/usr/bin/env bash

ARG1=${1:-"help"};
ARG2=${2:-"help"}

if [ "$ARG1" = "help" ]; then
    echo "";
    echo "-*--------*-";
    echo "-* app.sh *-";
    echo "-*--------*-";
    echo "";
fi;

if [ "$ARG1" = "help" ]; then
    echo "Usage: $0 [command]";
    echo "";
    echo "Commands:";
    echo "  help";
    echo "  clean [command/<dir>/all]";
    echo "  build";
    echo "  build:rust [command/<dir>/all]";
    echo "  build:virt [command/<dir>]";
    echo "  run";
    echo "  serve";
    echo "";
    exit;
fi;

if  [ "$ARG1" = "clean" ]; then
    if [ "$ARG2" = "help" ]; then
        echo "Usage: $0 clean [command/<dir>/all]";
        echo "";
        echo "Commands:";
        echo "  help";
        echo "";
        exit;
    fi;

    DIR_LIST=$(ls ./);

    for DIR in $DIR_LIST; do
        if [ -d "$DIR" ] && [ -f "$DIR/Cargo.toml" ]; then
            if [ "$ARG2" = "all" ] || [ "$ARG2" = "$DIR" ]; then
                echo "- Running ./app.sh clean $DIR";
                (cd $DIR && cargo-component clean);
            fi;
        fi;
    done;
    exit;
fi;

if  [ "$ARG1" = "build:rust" ]; then
    if [ "$ARG2" = "help" ]; then
        echo "Usage: $0 build:rust [command/<dir>/all]";
        echo "";
        echo "Commands:";
        echo "  help";
        echo "";
        exit;
    fi;

    DIR_LIST=$(ls ./);

    for DIR in $DIR_LIST; do
        if [ -d "$DIR" ] && [ -f "$DIR/Cargo.toml" ]; then
            if [ "$ARG2" = "all" ] || [ "$ARG2" = "$DIR" ]; then
                echo "- Running ./app.sh build:rust $DIR";
                if [ "$(cat $DIR/Cargo.toml | grep "cargo-component-bindings")" != "" ]; then
                    echo "- Building $DIR.wasm";
                    (cd $DIR && cargo-component build --release);
                else
                    echo "- Building $DIR via cargo";
                    (cd $DIR && cargo build --release);
                fi;
            fi;
        fi;
    done;
    exit;
fi;

if  [ "$ARG1" = "build:virt" ]; then
    if [ "$ARG2" = "help" ]; then
        echo "Usage: $0 build:virt [command/<dir>]";
        echo "";
        echo "Commands:";
        echo "  help";
        echo "";
        exit;
    fi;

    if [ -d "$ARG2" ] && [ -f "$ARG2/Cargo.toml" ]; then
        echo "- Running ./app.sh build:virt $ARG2";
        echo "- Building $ARG2.virt.wasm";
        (wasi-virt $ARG2/target/wasm32-wasi/release/$ARG2.wasm --stdio=allow -o $ARG2.virt.wasm);
    fi;
    exit;
fi;

if [ "$ARG1" = "run" ]; then
    echo "- Running ./app.sh run";
    (wasmtime --wasm component-model app.wasm);
    exit;
fi;

if [ "$ARG1" = "serve" ]; then
    echo "- Running ./app.sh serve";
    (cd actix-app && ./target/release/actix-app);
    exit;
fi;

if [ "$ARG1" = "build" ]; then
    echo "- Running ./app.sh build";
    ./app.sh build:rust all;
    # ./app.sh build:virt cli-app;
    # echo "- Building calculator.composed.wasm";
    # (wasm-tools compose calculator/target/wasm32-wasi/release/calculator.wasm \
    #     -d adder/target/wasm32-wasi/release/adder.wasm \
    #     -o calculator.composed.wasm);
    # echo "- Building app.wasm";
    # (wasm-tools compose cli-app.virt.wasm \
    #     -d adder/target/wasm32-wasi/release/adder.wasm \
    #     -o app.wasm);
    # (wasm-tools compose cli-app.virt.wasm \
    #     -d adder/target/wasm32-wasi/release/adder.wasm \
    #     -d calculator.composed.wasm \
    #     -o app.wasm);
    echo "- Building express-app";
    (cd express-app && npm install);
    echo "- Building js-divider";
    (cd js-divider && npm install);
    (cd js-divider && npm run build);
    exit;
fi;

