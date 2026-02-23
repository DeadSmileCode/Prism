#!/bin/bash
set -e

APP_NAME="test-plugin"

#- Сборка WASM
cargo build -p $APP_NAME --target wasm32-unknown-unknown --release

#-  Создание компонента
wasm-tools component new \
    target/wasm32-unknown-unknown/release/${APP_NAME//-/_}.wasm \
    -o target/wasm32-unknown-unknown/release/${APP_NAME//-/_}.component.wasm \
    --adapt wasi_snapshot_preview1.reactor.wasm

#- Запуск хоста
cargo run -p super-app